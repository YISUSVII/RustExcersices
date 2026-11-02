#!/usr/bin/env python3
"""Generate the static GitHub Pages site in docs/ from the course sources.

Markdown READMEs, exercise prompts, starters, and solutions are the editing
surface; everything under docs/ (except docs/assets/) is generated. Standard
library only, so it runs anywhere Python 3.9+ is available.
"""
from __future__ import annotations

import html
import json
import re
import shutil
import sys
import urllib.parse
from dataclasses import dataclass, field
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
DOCS = ROOT / "docs"
REPO_URL = "https://github.com/YISUSVII/RustExcersices"
BRANCH = "main"
SITE_TITLE = "Rust Study — Zero → Ninja"
SITE_BASE = "/rust-study/"
BLOG_URL = "https://yisusvii.github.io/"
LEVEL_RE = re.compile(r"^(\d\d)-[a-z0-9-]+$")
EXERCISE_RE = re.compile(r"^(\d\d)-([a-z0-9-]+)\.md$")
BROKEN_MARKER = "Intentional broken line"


# --------------------------------------------------------------------------- #
# Rust syntax highlighting
# --------------------------------------------------------------------------- #
RUST_KEYWORDS = {
    "as", "async", "await", "break", "const", "continue", "crate", "dyn", "else",
    "enum", "extern", "false", "fn", "for", "if", "impl", "in", "let", "loop",
    "match", "mod", "move", "mut", "pub", "ref", "return", "self", "Self",
    "static", "struct", "super", "trait", "true", "type", "unsafe", "use",
    "where", "while", "union",
}
RUST_PRIMITIVES = {
    "i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16", "u32", "u64",
    "u128", "usize", "f32", "f64", "bool", "char", "str",
}
RUST_TOKEN = re.compile(
    r"""
    (?P<comment>//[^\n]*|/\*[\s\S]*?\*/)
  | (?P<rawstr>b?r(?P<hashes>\#*)"[\s\S]*?"(?P=hashes))
  | (?P<string>b?"(?:\\[\s\S]|[^"\\])*")
  | (?P<char>b?'(?:\\(?:x[0-9a-fA-F]{2}|u\{[0-9a-fA-F]+\}|.)|[^'\\\n])')
  | (?P<lifetime>'[A-Za-z_]\w*)
  | (?P<attr>\#!?\[[^\]\n]*\])
  | (?P<number>\b(?:0x[0-9a-fA-F_]+|0b[01_]+|0o[0-7_]+|\d[\d_]*(?:\.\d[\d_]*)?(?:[eE][+-]?\d+)?)
        (?:[iu](?:8|16|32|64|128|size)|f32|f64)?\b)
  | (?P<macro>\b[A-Za-z_]\w*!)
  | (?P<ident>\b[A-Za-z_]\w*\b)
    """,
    re.VERBOSE,
)


def highlight_rust(code: str) -> str:
    out: list[str] = []
    pos = 0
    for m in RUST_TOKEN.finditer(code):
        out.append(html.escape(code[pos:m.start()], quote=False))
        text = html.escape(m.group(0), quote=False)
        kind = m.lastgroup
        cls = None
        if kind in ("comment", "number", "macro", "lifetime", "attr"):
            cls = kind
        elif kind in ("string", "rawstr", "hashes", "char"):
            cls = "string"
        elif kind == "ident":
            word = m.group(0)
            if word in RUST_KEYWORDS:
                cls = "keyword"
            elif word in RUST_PRIMITIVES or word[0].isupper():
                cls = "type"
            elif code[m.end():m.end() + 1] == "(":
                cls = "fn"
        out.append(f'<span class="tok-{cls}">{text}</span>' if cls else text)
        pos = m.end()
    out.append(html.escape(code[pos:], quote=False))
    return "".join(out)


def code_block(code: str, lang: str = "") -> str:
    lang = (lang or "").strip().lower()
    body = highlight_rust(code) if lang in ("rust", "rs") else html.escape(code, quote=False)
    label = f' data-lang="{html.escape(lang)}"' if lang else ""
    return f'<pre class="code"{label}><code>{body}</code></pre>'


# --------------------------------------------------------------------------- #
# Minimal Markdown renderer (the CommonMark subset used in this repository)
# --------------------------------------------------------------------------- #
LIST_RE = re.compile(r"^( *)([-*+]|\d+[.)])( +)(.*)$")
FENCE_RE = re.compile(r"^( *)(```+|~~~+)\s*([\w+-]*)\s*$")
HEADING_RE = re.compile(r"^(#{1,6})\s+(.*?)\s*#*\s*$")
TABLE_SEP_RE = re.compile(r"^\s*\|?\s*:?-{2,}:?\s*(\|\s*:?-{2,}:?\s*)*\|?\s*$")


def slugify(text: str) -> str:
    text = re.sub(r"<[^>]+>", "", text).lower()
    text = re.sub(r"[^\w\s-]", "", text)
    return re.sub(r"[\s_]+", "-", text).strip("-")


class Markdown:
    def __init__(self, link_resolver=None):
        self.resolve = link_resolver or (lambda url: url)

    def inline(self, text: str) -> str:
        stash: list[str] = []

        def keep(fragment: str) -> str:
            stash.append(fragment)
            return f"\x00{len(stash) - 1}\x00"

        text = re.sub(
            r"(`+)(.+?)\1",
            lambda m: keep(f"<code>{html.escape(m.group(2).strip(), quote=False)}</code>"),
            text,
        )
        text = re.sub(
            r"<(https?://[^>\s]+)>",
            lambda m: keep(self._link(m.group(1), html.escape(m.group(1)))),
            text,
        )
        text = html.escape(text, quote=False)
        text = re.sub(
            r"\[([^\]]+)\]\(([^)\s]+)\)",
            lambda m: keep(self._link(html.unescape(m.group(2)), self._emphasis(m.group(1)))),
            text,
        )
        text = self._emphasis(text)
        while "\x00" in text:
            text = re.sub(r"\x00(\d+)\x00", lambda m: stash[int(m.group(1))], text)
        return text

    @staticmethod
    def _emphasis(text: str) -> str:
        text = re.sub(r"\*\*(.+?)\*\*", r"<strong>\1</strong>", text)
        text = re.sub(r"(?<![\w*])\*(?!\s)(.+?)(?<!\s)\*(?![\w*])", r"<em>\1</em>", text)
        return text

    def _link(self, url: str, label: str) -> str:
        target = self.resolve(url)
        attrs = ' target="_blank" rel="noopener"' if target.startswith("http") else ""
        return f'<a href="{html.escape(target)}"{attrs}>{label}</a>'

    def render(self, text: str) -> str:
        return self._blocks(text.replace("\r\n", "\n").replace("\t", "    ").split("\n"))

    def _blocks(self, lines: list[str]) -> str:
        out: list[str] = []
        para: list[str] = []
        i = 0

        def flush():
            if para:
                out.append(f"<p>{self.inline(' '.join(s.strip() for s in para))}</p>")
                para.clear()

        while i < len(lines):
            line = lines[i]
            stripped = line.strip()
            if not stripped:
                flush()
                i += 1
                continue
            fence = FENCE_RE.match(line)
            if fence:
                flush()
                indent, marker, lang = len(fence.group(1)), fence.group(2), fence.group(3)
                body = []
                i += 1
                while i < len(lines) and not lines[i].strip().startswith(marker):
                    ln = lines[i]
                    body.append(ln[indent:] if ln[:indent].strip() == "" else ln)
                    i += 1
                i += 1
                out.append(code_block("\n".join(body), lang))
                continue
            heading = HEADING_RE.match(line)
            if heading:
                flush()
                level = len(heading.group(1))
                out.append(f'<h{level} id="{slugify(heading.group(2))}">'
                           f'{self.inline(heading.group(2))}</h{level}>')
                i += 1
                continue
            if stripped.startswith("|") and i + 1 < len(lines) and TABLE_SEP_RE.match(lines[i + 1]):
                flush()
                rows = []
                while i < len(lines) and lines[i].strip().startswith("|"):
                    rows.append(lines[i])
                    i += 1
                out.append(self._table(rows))
                continue
            if stripped.startswith(">"):
                flush()
                quote = []
                while i < len(lines) and lines[i].strip().startswith(">"):
                    quote.append(re.sub(r"^\s*>\s?", "", lines[i]))
                    i += 1
                out.append(f"<blockquote>{self._blocks(quote)}</blockquote>")
                continue
            if re.match(r"^ {0,3}([-*_])( *\1){2,} *$", line):
                flush()
                out.append("<hr>")
                i += 1
                continue
            if LIST_RE.match(line) and not para:
                i = self._list(lines, i, out)
                continue
            para.append(line)
            i += 1
        flush()
        return "\n".join(out)

    @staticmethod
    def _same_list(m, base: int, ordered: bool) -> bool:
        return bool(m) and len(m.group(1)) == base and m.group(2)[0].isdigit() == ordered

    def _list(self, lines: list[str], i: int, out: list[str]) -> int:
        first = LIST_RE.match(lines[i])
        base = len(first.group(1))
        ordered = first.group(2)[0].isdigit()
        start = int(re.match(r"\d+", first.group(2)).group(0)) if ordered else 1
        items: list[list[str]] = []
        loose = False
        while i < len(lines):
            m = LIST_RE.match(lines[i])
            if not self._same_list(m, base, ordered):
                break
            content_col = base + len(m.group(2)) + len(m.group(3))
            item = [m.group(4)]
            i += 1
            while i < len(lines):
                ln = lines[i]
                if not ln.strip():
                    nxt = next((l for l in lines[i + 1:] if l.strip()), None)
                    if nxt is None:
                        break
                    if len(nxt) - len(nxt.lstrip(" ")) > base:
                        item.append("")
                        i += 1
                        continue
                    if self._same_list(LIST_RE.match(nxt), base, ordered):
                        loose = True
                    break
                indent = len(ln) - len(ln.lstrip(" "))
                if indent > base:
                    item.append(ln[min(indent, content_col):])
                    i += 1
                elif LIST_RE.match(ln) or FENCE_RE.match(ln) or HEADING_RE.match(ln):
                    break
                else:
                    item.append(ln)  # lazy continuation line
                    i += 1
            while i < len(lines) and not lines[i].strip():
                nxt = next((l for l in lines[i + 1:] if l.strip()), None)
                if nxt and self._same_list(LIST_RE.match(nxt), base, ordered):
                    i += 1
                else:
                    break
            items.append(item)
        rendered = []
        for item in items:
            inner = self._blocks(item)
            if not loose:
                inner = re.sub(r"^<p>(.*?)</p>", r"\1", inner, count=1, flags=re.S)
            task = re.match(r"^\[( |x|X)\]\s+", inner)
            if task:
                checked = " checked" if task.group(1).lower() == "x" else ""
                inner = f'<input type="checkbox" disabled{checked}> ' + inner[task.end():]
            rendered.append(f"<li>{inner}</li>")
        tag = "ol" if ordered else "ul"
        start_attr = f' start="{start}"' if ordered and start != 1 else ""
        out.append(f"<{tag}{start_attr}>" + "".join(rendered) + f"</{tag}>")
        return i

    def _table(self, rows: list[str]) -> str:
        def cells(row: str) -> list[str]:
            return [c.strip() for c in row.strip().strip("|").split("|")]

        thead = "".join(f"<th>{self.inline(c)}</th>" for c in cells(rows[0]))
        tbody = "".join("<tr>" + "".join(f"<td>{self.inline(c)}</td>" for c in cells(r)) + "</tr>"
                        for r in rows[2:])
        return (f'<div class="table-wrap"><table><thead><tr>{thead}</tr></thead>'
                f"<tbody>{tbody}</tbody></table></div>")


# --------------------------------------------------------------------------- #
# Course model
# --------------------------------------------------------------------------- #
@dataclass
class Exercise:
    number: str
    slug: str
    title: str
    difficulty: str
    concepts: str
    objective: str
    level: "Level"
    prompt_path: Path
    starter_path: Path | None
    solution_path: Path | None

    @property
    def stem(self) -> str:
        return f"{self.number}-{self.slug}"

    @property
    def page(self) -> str:
        return f"exercises/{self.stem}.html"

    @property
    def starter_broken(self) -> bool:
        return bool(self.starter_path and BROKEN_MARKER in self.starter_path.read_text())


@dataclass
class Level:
    dirname: str
    number: int
    title: str
    short_title: str
    summary: str
    readme: Path
    exercises: list[Exercise] = field(default_factory=list)

    @property
    def page(self) -> str:
        return f"levels/{self.dirname}.html"


DIFFICULTY_BY_LEVEL = {
    0: "Beginner", 1: "Beginner", 2: "Beginner", 3: "Beginner",
    4: "Intermediate", 5: "Intermediate", 6: "Intermediate", 7: "Intermediate",
    8: "Intermediate", 9: "Advanced", 10: "Advanced", 11: "Advanced",
    12: "Advanced", 13: "Advanced", 14: "Advanced", 15: "Advanced",
    16: "Advanced", 17: "Ninja", 18: "Ninja",
}


def field_value(md: str, name: str) -> str:
    m = re.search(rf"\*\*{name}:\*\*\s*(.+)", md)
    return m.group(1).strip() if m else ""


def section_text(md: str, heading: str) -> str:
    m = re.search(rf"^##\s+{heading}\s*\n(.*?)(?=^##\s|\Z)", md, re.S | re.M)
    if not m:
        return ""
    text = re.sub(r"```.*?```", "", m.group(1), flags=re.S)
    return re.sub(r"[`*]", "", " ".join(text.split()))


def load_course() -> list[Level]:
    levels: list[Level] = []
    for d in sorted(p for p in ROOT.iterdir() if p.is_dir() and LEVEL_RE.match(p.name)):
        readme = d / "README.md"
        if not readme.exists():
            continue
        text = readme.read_text()
        h1 = re.search(r"^#\s+(.+)$", text, re.M)
        title = h1.group(1).strip() if h1 else d.name
        summary = section_text(text, "Simple explanation") or section_text(text, "What you will learn")
        level = Level(d.name, int(d.name[:2]), title, title.split("—", 1)[-1].strip(), summary, readme)
        ex_dir = d / "exercises"
        for md_path in sorted(ex_dir.glob("*.md")) if ex_dir.is_dir() else []:
            m = EXERCISE_RE.match(md_path.name)
            if not m:
                continue
            md = md_path.read_text()
            h = re.search(r"^#\s+(.+)$", md, re.M)
            heading = h.group(1).strip() if h else md_path.stem
            starter = md_path.with_suffix(".rs")
            solution = ROOT / "solutions" / d.name / f"{md_path.stem}.rs"
            level.exercises.append(Exercise(
                number=m.group(1), slug=m.group(2),
                title=field_value(md, "Title") or heading.split("—", 1)[-1].strip(),
                difficulty=field_value(md, "Difficulty") or DIFFICULTY_BY_LEVEL.get(level.number, ""),
                concepts=field_value(md, "Concepts practiced"),
                objective=field_value(md, "Objective"),
                level=level, prompt_path=md_path,
                starter_path=starter if starter.exists() else None,
                solution_path=solution if solution.exists() else None,
            ))
        levels.append(level)
    return levels


class Linker:
    """Rewrites repository-relative Markdown links to site pages or GitHub URLs."""

    def __init__(self, levels: list[Level]):
        self.map: dict[Path, str] = {
            (ROOT / "README.md").resolve(): "guide.html",
            (ROOT / "resources" / "README.md").resolve(): "resources.html",
            (ROOT / "resources").resolve(): "resources.html",
        }
        for lv in levels:
            self.map[lv.readme.resolve()] = lv.page
            self.map[(ROOT / lv.dirname).resolve()] = lv.page
            for ex in lv.exercises:
                self.map[ex.prompt_path.resolve()] = ex.page
                if ex.starter_path:
                    self.map[ex.starter_path.resolve()] = ex.page
                if ex.solution_path:
                    self.map[ex.solution_path.resolve()] = ex.page + "#solution"

    def resolver(self, source: Path, prefix: str):
        def resolve(url: str) -> str:
            if re.match(r"^[a-z][a-z0-9+.-]*:", url) or url.startswith("#"):
                return url
            path_part, _, anchor = url.partition("#")
            target = (source.parent / urllib.parse.unquote(path_part)).resolve()
            page = self.map.get(target)
            if page:
                return prefix + page + (f"#{anchor}" if anchor and "#" not in page else "")
            try:
                rel = target.relative_to(ROOT).as_posix()
            except ValueError:
                return url
            kind = "tree" if target.is_dir() else "blob"
            return f"{REPO_URL}/{kind}/{BRANCH}/{rel}"
        return resolve


# --------------------------------------------------------------------------- #
# Page templates
# --------------------------------------------------------------------------- #
def esc(text: str) -> str:
    return html.escape(text, quote=True)


def diff_class(difficulty: str) -> str:
    return "diff-" + (difficulty.split()[0].lower() if difficulty else "none")


def sidebar(levels: list[Level], prefix: str, current_level: str, current_ex: str, current_doc: str) -> str:
    def doc_link(href: str, label: str, key: str) -> str:
        active = " active" if key == current_doc else ""
        return f'<a class="side-home{active}" href="{prefix}{href}">{label}</a>'

    parts = ['<nav class="sidebar" id="sidebar" aria-label="Chapters">',
             doc_link("index.html", "🏠 Roadmap", "index"),
             doc_link("guide.html", "📘 Course guide", "guide"),
             '<div class="side-title">Chapters</div>']
    for lv in levels:
        is_current = lv.dirname == current_level
        active = " active" if is_current and not current_ex else ""
        label = f'<span class="lvl-num">{lv.number:02d}</span><span class="lvl-name">{esc(lv.short_title)}</span>'
        if lv.exercises:
            parts.append(f'<details{" open" if is_current else ""}><summary class="side-level">'
                         f'{label}<span class="count">{len(lv.exercises)}</span></summary><ul>')
            parts.append(f'<li><a href="{prefix}{lv.page}" class="side-ex side-overview{active}">Overview</a></li>')
            for ex in lv.exercises:
                cls = " active" if ex.stem == current_ex else ""
                aria = ' aria-current="page"' if cls else ""
                parts.append(f'<li><a class="side-ex{cls}"{aria} data-ex="{ex.number}" href="{prefix}{ex.page}">'
                             f'<span class="done-dot" aria-hidden="true"></span>{ex.number} · {esc(ex.title)}</a></li>')
            parts.append("</ul></details>")
        else:
            parts.append(f'<a class="side-level single{active}" href="{prefix}{lv.page}">{label}</a>')
    parts.append(doc_link("resources.html", "📚 Resources", "resources"))
    parts.append("</nav>")
    return "\n".join(parts)


def layout(levels, prefix, title, body, description="", current_level="", current_ex="", current_doc="") -> str:
    full_title = title if title == SITE_TITLE else f"{title} · {SITE_TITLE}"
    desc = esc(description or "A progressive, hands-on Rust course from zero to ninja: "
                              "chapters, exercises, starter code and reference solutions.")
    return f"""<!doctype html>
<html lang="en" data-theme="dark">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>{esc(full_title)}</title>
<meta name="description" content="{desc}">
<meta property="og:title" content="{esc(full_title)}">
<meta property="og:description" content="{desc}">
<meta property="og:type" content="website">
<link rel="icon" href="{prefix}assets/favicon.svg" type="image/svg+xml">
<link rel="stylesheet" href="{prefix}assets/style.css">
<script>try{{var t=localStorage.getItem('rust-study-theme');if(t)document.documentElement.dataset.theme=t;}}catch(e){{}}</script>
</head>
<body>
<a class="skip-link" href="#content">Skip to content</a>
<header class="topbar">
  <button class="icon-btn menu-btn" id="menu-btn" type="button" aria-label="Toggle chapters" aria-controls="sidebar" aria-expanded="false">☰</button>
  <a class="brand" href="{prefix}index.html"><img src="{prefix}assets/favicon.svg" alt="" width="28" height="28">Rust Study<span class="brand-sub">Zero → Ninja</span></a>
  <div class="top-actions">
    <a class="top-link" href="{BLOG_URL}">Blog</a>
    <a class="top-link" href="{REPO_URL}" target="_blank" rel="noopener">GitHub</a>
    <button class="icon-btn" id="theme-btn" type="button" aria-label="Toggle light/dark theme">◐</button>
  </div>
</header>
<div class="shell">
{sidebar(levels, prefix, current_level, current_ex, current_doc)}
<main class="content" id="content">
{body}
<footer class="footer">Generated from <a href="{REPO_URL}" target="_blank" rel="noopener">the course repository</a> · Learn Rust by building 🦀</footer>
</main>
</div>
<script src="{prefix}assets/app.js"></script>
</body>
</html>
"""


def playground_url(code: str) -> str:
    return ("https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code="
            + urllib.parse.quote(code))


def code_panel(path: Path) -> str:
    code = path.read_text()
    rel = path.relative_to(ROOT).as_posix()
    return f"""<div class="code-panel">
  <div class="code-toolbar">
    <span class="file-name">{esc(rel)}</span>
    <span class="code-actions">
      <button class="btn small copy-btn" type="button">Copy</button>
      <a class="btn small" href="{esc(playground_url(code))}" target="_blank" rel="noopener">▶ Playground</a>
      <a class="btn small ghost" href="{REPO_URL}/blob/{BRANCH}/{esc(rel)}" target="_blank" rel="noopener">GitHub</a>
    </span>
  </div>
  {code_block(code.rstrip(), "rust")}
</div>"""


def pager(prev_href, prev_label, next_href, next_label) -> str:
    left = (f'<a class="pager-link prev" href="{prev_href}"><small>← Previous</small>{prev_label}</a>'
            if prev_href else '<span class="pager-link disabled"></span>')
    right = (f'<a class="pager-link next" href="{next_href}"><small>Next →</small>{next_label}</a>'
             if next_href else '<span class="pager-link disabled"></span>')
    return f'<nav class="pager" aria-label="Pagination">{left}{right}</nav>'


def exercise_page(ex: Exercise, prev_ex, next_ex, levels, linker) -> str:
    prefix = "../"
    lv = ex.level
    md = re.sub(r"^#\s+.+\n", "", ex.prompt_path.read_text(), count=1)
    # Metadata bullets are shown in the page header instead.
    md = re.sub(r"^- \*\*(Title|Objective|Difficulty|Concepts practiced):\*\*.*\n", "", md, flags=re.M)
    prompt = Markdown(linker.resolver(ex.prompt_path, prefix)).render(md)
    starter = code_panel(ex.starter_path) if ex.starter_path else '<p class="muted">No starter file for this exercise.</p>'
    broken = ('<div class="callout warn">⚠️ This starter <strong>intentionally does not compile</strong>. '
              'Read the compiler error carefully and fix it.</div>' if ex.starter_broken else "")
    if ex.solution_path:
        solution = f"""<div class="spoiler" id="spoiler">
  <p>🧠 Try it yourself first — compile, read the errors, iterate. That's how Rust sticks.</p>
  <button class="btn primary" id="reveal-btn" type="button">Reveal solution</button>
</div>
<div id="solution-body" hidden>{code_panel(ex.solution_path)}</div>"""
        has_tests = "#[cfg(test)]" in ex.solution_path.read_text()
    else:
        solution, has_tests = '<p class="muted">Solution coming soon.</p>', False
    run_cmd = f"rustc --edition 2021 {ex.stem}.rs -o {ex.stem} && ./{ex.stem}"
    if has_tests:
        run_cmd += f"\n\n# run the unit tests\nrustc --edition 2021 --test {ex.stem}.rs -o {ex.stem}-tests && ./{ex.stem}-tests"
    concepts = "".join(f'<span class="chip">{esc(c.strip(" ."))}</span>'
                       for c in re.split(r",\s*", ex.concepts) if c.strip())
    body = f"""<nav class="breadcrumb"><a href="{prefix}index.html">Roadmap</a><span>/</span><a href="{prefix}{lv.page}">Level {lv.number} — {esc(lv.short_title)}</a><span>/</span><span>Exercise {ex.number}</span></nav>
<article class="exercise">
<header class="page-head">
  <div class="eyebrow">Exercise {ex.number} · Level {lv.number} — {esc(lv.short_title)}</div>
  <h1>{esc(ex.title)}</h1>
  {f'<p class="lead">{Markdown().inline(ex.objective)}</p>' if ex.objective else ""}
  <div class="meta"><span class="badge {diff_class(ex.difficulty)}">{esc(ex.difficulty)}</span>{concepts}</div>
  <label class="done-toggle"><input type="checkbox" class="done-check" data-ex="{ex.number}"> Mark as completed</label>
</header>
<section class="prose">{prompt}</section>
<section class="code-section" aria-label="Code">
  <div class="tabs" role="tablist">
    <button class="tab active" type="button" role="tab" aria-selected="true" data-tab="starter">Starter code</button>
    <button class="tab" type="button" role="tab" aria-selected="false" data-tab="solution">Solution</button>
    <button class="tab" type="button" role="tab" aria-selected="false" data-tab="run">How to run</button>
  </div>
  <div class="tab-panel" role="tabpanel" data-panel="starter">{broken}{starter}</div>
  <div class="tab-panel" role="tabpanel" data-panel="solution" id="solution" hidden>{solution}</div>
  <div class="tab-panel prose" role="tabpanel" data-panel="run" hidden>
    <p>Every exercise is a single standalone file — no Cargo project needed. Save it locally and run:</p>
    {code_block(run_cmd, "bash")}
    <p>Or click <strong>▶ Playground</strong> to run it in your browser on the official Rust Playground.</p>
  </div>
</section>
{pager(prefix + prev_ex.page if prev_ex else None, f"{prev_ex.number} · {esc(prev_ex.title)}" if prev_ex else "",
       prefix + next_ex.page if next_ex else None, f"{next_ex.number} · {esc(next_ex.title)}" if next_ex else "")}
</article>"""
    return layout(levels, prefix, f"{ex.number} · {ex.title}", body,
                  description=ex.objective, current_level=lv.dirname, current_ex=ex.stem)


def exercise_card(ex: Exercise, prefix: str, show_level: bool = False) -> str:
    level = f'<span class="card-level">Level {ex.level.number} · {esc(ex.level.short_title)}</span>' if show_level else ""
    search = esc(" ".join([ex.number, ex.title, ex.concepts, ex.objective, ex.level.short_title]).lower())
    return f"""<a class="card ex-card" href="{prefix}{ex.page}" data-ex="{ex.number}" data-search="{search}">
  <div class="card-top"><span class="ex-num">{ex.number}</span><span class="badge {diff_class(ex.difficulty)}">{esc(ex.difficulty)}</span><span class="done-mark" title="Completed">✓</span></div>
  <h3>{esc(ex.title)}</h3>
  {level}<p>{esc(ex.objective.replace("`", ""))}</p>
</a>"""


def level_page(lv: Level, prev_lv, next_lv, levels, linker) -> str:
    prefix = "../"
    md = re.sub(r"^#\s+.+\n", "", lv.readme.read_text(), count=1)
    content = Markdown(linker.resolver(lv.readme, prefix)).render(md)
    cards = ""
    if lv.exercises:
        nums = ",".join(e.number for e in lv.exercises)
        cards = (f'<section class="level-exercises"><div class="section-head"><h2 id="exercise-cards">Exercises</h2>'
                 f'<span class="level-progress" data-level-exs="{nums}"></span></div><div class="grid">'
                 + "".join(exercise_card(e, prefix) for e in lv.exercises) + "</div></section>")
    body = f"""<nav class="breadcrumb"><a href="{prefix}index.html">Roadmap</a><span>/</span><span>Level {lv.number}</span></nav>
<header class="page-head">
  <div class="eyebrow">Level {lv.number} · {esc(DIFFICULTY_BY_LEVEL.get(lv.number, ''))}</div>
  <h1>{esc(lv.short_title)}</h1>
  <p class="lead">{esc(lv.summary)}</p>
</header>
{cards}
<section class="prose">{content}</section>
{pager(prefix + prev_lv.page if prev_lv else None, f"Level {prev_lv.number} — {esc(prev_lv.short_title)}" if prev_lv else "",
       prefix + next_lv.page if next_lv else None, f"Level {next_lv.number} — {esc(next_lv.short_title)}" if next_lv else "")}"""
    return layout(levels, prefix, lv.title, body, description=lv.summary, current_level=lv.dirname)


def index_page(levels: list[Level]) -> str:
    all_ex = [e for lv in levels for e in lv.exercises]
    stages = [("ZERO", "Rust syntax and fundamentals", range(0, 4)),
              ("BEGINNER", "Ownership, structs, enums, collections", range(4, 9)),
              ("ADVANCED", "Traits, lifetimes, iterators, testing", range(9, 14)),
              ("NINJA", "Concurrency, async, macros, unsafe Rust and real projects", range(14, 19))]
    sections = []
    for name, desc, rng in stages:
        cards = []
        for lv in (l for l in levels if l.number in rng):
            diff = DIFFICULTY_BY_LEVEL.get(lv.number, "")
            if lv.exercises:
                meta = f"{len(lv.exercises)} exercise{'s' if len(lv.exercises) != 1 else ''}"
            else:
                meta = "Project briefs" if lv.number == 18 else "Guide"
            cards.append(f"""<a class="card level-card" href="{lv.page}" data-level-exs="{','.join(e.number for e in lv.exercises)}">
  <div class="card-top"><span class="lvl-big">{lv.number:02d}</span><span class="badge {diff_class(diff)}">{diff}</span></div>
  <h3>{esc(lv.short_title)}</h3>
  <p>{esc(lv.summary)}</p>
  <div class="card-foot"><span>{meta}</span><span class="mini-progress"><span></span></span></div>
</a>""")
        sections.append(f'<section class="stage"><div class="stage-head"><h2>{name}</h2><span>{esc(desc)}</span></div>'
                        f'<div class="grid">{"".join(cards)}</div></section>')
    body = f"""<section class="hero">
  <div class="hero-text">
    <div class="eyebrow">🦀 Learn Rust by building</div>
    <h1>Rust <span class="accent">Zero → Ninja</span></h1>
    <p class="lead">{len(levels)} chapters and {len(all_ex)} hands-on exercises, each with starter code and a reference solution — from <code>let x = 5;</code> to thread pools, async executors, macros and <code>unsafe</code>.</p>
    <div class="hero-actions">
      <a class="btn primary" href="{levels[0].page}">Start at Level 0</a>
      <a class="btn" href="guide.html">Course guide</a>
      <a class="btn" id="continue-btn" href="#" hidden>Continue →</a>
    </div>
  </div>
  <div class="hero-progress card">
    <div class="ring" id="overall-ring" data-total="{len(all_ex)}"><div><strong id="overall-count">0</strong><small>/ {len(all_ex)}</small></div></div>
    <p>exercises completed</p>
    <p class="muted small">Progress is saved in this browser.</p>
  </div>
</section>
<section class="search-box">
  <input type="search" id="search" placeholder="Search exercises — try “HashMap”, “lifetimes”, “threads”…" aria-label="Search exercises" autocomplete="off">
  <p id="search-empty" class="muted" hidden>No exercises match your search.</p>
  <div id="search-results" class="grid" hidden>{"".join(exercise_card(e, "", show_level=True) for e in all_ex)}</div>
</section>
<div id="roadmap">{''.join(sections)}</div>"""
    return layout(levels, "", SITE_TITLE, body, current_doc="index")


def doc_page(levels, linker, source: Path, title: str, key: str) -> str:
    md = re.sub(r"^#\s+.+\n", "", source.read_text(), count=1)
    content = Markdown(linker.resolver(source, "")).render(md)
    body = f"""<nav class="breadcrumb"><a href="index.html">Roadmap</a><span>/</span><span>{esc(title)}</span></nav>
<header class="page-head"><h1>{esc(title)}</h1></header>
<section class="prose">{content}</section>"""
    return layout(levels, "", title, body, current_doc=key)


def build() -> int:
    levels = load_course()
    linker = Linker(levels)
    for sub in ("levels", "exercises"):
        shutil.rmtree(DOCS / sub, ignore_errors=True)
        (DOCS / sub).mkdir(parents=True)
    written = 0

    def write(rel: str, text: str):
        nonlocal written
        (DOCS / rel).write_text(text)
        written += 1

    write("index.html", index_page(levels))
    write("guide.html", doc_page(levels, linker, ROOT / "README.md", "Course guide", "guide"))
    write("resources.html", doc_page(levels, linker, ROOT / "resources" / "README.md", "Resources", "resources"))
    for i, lv in enumerate(levels):
        write(lv.page, level_page(lv, levels[i - 1] if i else None,
                                  levels[i + 1] if i + 1 < len(levels) else None, levels, linker))
    all_ex = [e for lv in levels for e in lv.exercises]
    for i, ex in enumerate(all_ex):
        write(ex.page, exercise_page(ex, all_ex[i - 1] if i else None,
                                     all_ex[i + 1] if i + 1 < len(all_ex) else None, levels, linker))
    write("404.html", layout(levels, SITE_BASE, "Page not found",
                             '<header class="page-head"><h1>404 — Page not found</h1><p class="lead">This page moved or '
                             f'never existed. Head back to the <a href="{SITE_BASE}index.html">roadmap</a>.</p></header>'))
    (DOCS / ".nojekyll").write_text("")
    print(f"Built {written} pages: {len(levels)} levels, {len(all_ex)} exercises -> docs/")
    missing = [e.stem for e in all_ex if not e.solution_path]
    if missing:
        print("Exercises without solutions: " + ", ".join(missing))
    return 0


if __name__ == "__main__":
    sys.exit(build())
