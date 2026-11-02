#!/usr/bin/env python3
"""Compile, run, and test every exercise starter and reference solution.

Checks performed (rustc only, no Cargo needed):
- every exercise prompt NN-slug.md has a starter .rs and a solution
- solutions build with `-D warnings`, run successfully, and pass their unit tests
- starters compile, except intentionally broken ones (marked "Intentional broken line"),
  which must FAIL to compile
"""
from __future__ import annotations

import os
import re
import subprocess
import sys
import tempfile
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
BROKEN_MARKER = "Intentional broken line"
RUSTC = ["rustc", "--edition", "2021"]
TIMEOUT = 120


def run(cmd: list[str], cwd: Path) -> tuple[bool, str]:
    try:
        p = subprocess.run(cmd, cwd=cwd, capture_output=True, text=True, timeout=TIMEOUT)
    except subprocess.TimeoutExpired:
        return False, f"timed out after {TIMEOUT}s: {' '.join(cmd)}"
    return p.returncode == 0, (p.stdout + p.stderr)[-4000:]


def check_solution(path: Path, tmp: Path) -> list[str]:
    errors = []
    exe, tests = tmp / f"{path.stem}-bin", tmp / f"{path.stem}-tests"
    ok, out = run(RUSTC + ["-D", "warnings", str(path), "-o", str(exe)], tmp)
    if not ok:
        return [f"{path}: build failed\n{out}"]
    ok, out = run([str(exe)], tmp)
    if not ok:
        errors.append(f"{path}: run failed\n{out}")
    if "#[cfg(test)]" in path.read_text():
        ok, out = run(RUSTC + ["--test", "-D", "warnings", str(path), "-o", str(tests)], tmp)
        if ok:
            ok, out = run([str(tests), "--quiet"], tmp)
        if not ok:
            errors.append(f"{path}: tests failed\n{out}")
    return errors


def check_starter(path: Path, tmp: Path) -> list[str]:
    broken = BROKEN_MARKER in path.read_text()
    ok, out = run(RUSTC + [str(path), "-o", str(tmp / f"{path.stem}-starter")], tmp)
    if broken and ok:
        return [f"{path}: marked as intentionally broken but compiled"]
    if not broken and not ok:
        return [f"{path}: starter failed to compile\n{out}"]
    return []


def main() -> int:
    prompts = sorted(ROOT.glob("[0-9][0-9]-*/exercises/[0-9][0-9]-*.md"))
    errors: list[str] = []
    jobs = []
    for md in prompts:
        starter = md.with_suffix(".rs")
        solution = ROOT / "solutions" / md.parent.parent.name / starter.name
        for kind, f in (("starter", starter), ("solution", solution)):
            if f.exists():
                jobs.append((kind, f))
            else:
                errors.append(f"{md.relative_to(ROOT)}: missing {kind} {f.relative_to(ROOT)}")
    numbers = [re.match(r"\d+", p.name).group(0) for p in prompts]
    dupes = sorted({n for n in numbers if numbers.count(n) > 1})
    if dupes:
        errors.append(f"duplicate exercise numbers: {', '.join(dupes)}")

    with tempfile.TemporaryDirectory() as tmpdir:
        tmp = Path(tmpdir)

        def work(job):
            kind, f = job
            sub = tmp / f"{kind}-{f.parent.name}"
            sub.mkdir(exist_ok=True)
            return (check_solution if kind == "solution" else check_starter)(f, sub)

        with ThreadPoolExecutor(max_workers=os.cpu_count() or 2) as pool:
            for result in pool.map(work, jobs):
                errors.extend(result)

    print(f"Checked {len(prompts)} exercises ({len(jobs)} files).")
    if errors:
        print("\n\n".join(errors), file=sys.stderr)
        print(f"\n{len(errors)} problem(s) found.", file=sys.stderr)
        return 1
    print("All starters and solutions OK.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
