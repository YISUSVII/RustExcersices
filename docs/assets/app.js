// Rust Study — progressive enhancement: theme, nav, tabs, copy, progress, search.
(function () {
  "use strict";
  const DONE_KEY = "rust-study-done";
  const LAST_KEY = "rust-study-last";
  const THEME_KEY = "rust-study-theme";
  const store = {
    get(key, fallback) { try { return JSON.parse(localStorage.getItem(key)) ?? fallback; } catch (e) { return fallback; } },
    set(key, value) { try { localStorage.setItem(key, JSON.stringify(value)); } catch (e) { /* private mode */ } },
  };
  const $ = (sel, root = document) => root.querySelector(sel);
  const $$ = (sel, root = document) => Array.from(root.querySelectorAll(sel));

  // Theme
  $("#theme-btn")?.addEventListener("click", () => {
    const next = document.documentElement.dataset.theme === "dark" ? "light" : "dark";
    document.documentElement.dataset.theme = next;
    try { localStorage.setItem(THEME_KEY, next); } catch (e) { /* ignore */ }
  });

  // Mobile sidebar
  const menuBtn = $("#menu-btn");
  menuBtn?.addEventListener("click", () => {
    const open = document.body.classList.toggle("nav-open");
    menuBtn.setAttribute("aria-expanded", String(open));
  });
  $("#content")?.addEventListener("click", () => {
    document.body.classList.remove("nav-open");
    menuBtn?.setAttribute("aria-expanded", "false");
  });
  $(".sidebar .active")?.scrollIntoView({ block: "center" });

  // Tabs + solution spoiler
  const tabs = $$(".tab");
  function selectTab(name) {
    tabs.forEach((t) => {
      const on = t.dataset.tab === name;
      t.classList.toggle("active", on);
      t.setAttribute("aria-selected", String(on));
    });
    $$(".tab-panel").forEach((p) => { p.hidden = p.dataset.panel !== name; });
  }
  tabs.forEach((t) => t.addEventListener("click", () => selectTab(t.dataset.tab)));
  function revealSolution() {
    $("#spoiler")?.remove();
    const body = $("#solution-body");
    if (body) body.hidden = false;
  }
  $("#reveal-btn")?.addEventListener("click", revealSolution);
  if (location.hash === "#solution" && $("#solution")) {
    selectTab("solution");
    revealSolution();
  }

  // Copy buttons
  $$(".copy-btn").forEach((btn) => btn.addEventListener("click", async () => {
    const code = btn.closest(".code-panel").querySelector("pre code").innerText;
    try {
      await navigator.clipboard.writeText(code);
      btn.textContent = "Copied!";
    } catch (e) {
      btn.textContent = "Copy failed";
    }
    setTimeout(() => { btn.textContent = "Copy"; }, 1500);
  }));

  // Progress tracking
  let done = new Set(store.get(DONE_KEY, []));
  function renderProgress() {
    $$("[data-ex]").forEach((el) => {
      if (el.classList.contains("done-check")) el.checked = done.has(el.dataset.ex);
      else el.classList.toggle("is-done", done.has(el.dataset.ex));
    });
    $$("[data-level-exs]").forEach((el) => {
      const ids = el.dataset.levelExs ? el.dataset.levelExs.split(",") : [];
      const count = ids.filter((id) => done.has(id)).length;
      const bar = el.querySelector(".mini-progress > span");
      if (bar) bar.style.width = ids.length ? `${(100 * count) / ids.length}%` : "0";
      if (el.classList.contains("level-progress") && ids.length) el.textContent = `${count} / ${ids.length} completed`;
    });
    const ring = $("#overall-ring");
    if (ring) {
      const total = Number(ring.dataset.total) || 1;
      const count = Math.min(done.size, total);
      $("#overall-count").textContent = String(count);
      ring.style.setProperty("--pct", String((100 * count) / total));
    }
  }
  $$(".done-check").forEach((box) => box.addEventListener("change", () => {
    if (box.checked) done.add(box.dataset.ex); else done.delete(box.dataset.ex);
    store.set(DONE_KEY, Array.from(done));
    renderProgress();
  }));
  window.addEventListener("storage", (e) => {
    if (e.key === DONE_KEY) { done = new Set(store.get(DONE_KEY, [])); renderProgress(); }
  });
  renderProgress();

  // Remember last visited exercise for "Continue"
  const current = $(".side-ex.active[data-ex]");
  if (current && $(".exercise")) {
    store.set(LAST_KEY, { href: new URL(current.getAttribute("href"), location.href).pathname, label: current.textContent.trim() });
  }
  const cont = $("#continue-btn");
  const last = store.get(LAST_KEY, null);
  if (cont && last && last.href) {
    cont.href = last.href;
    cont.textContent = `Continue: ${last.label} →`;
    cont.hidden = false;
  }

  // Search on the roadmap page
  const search = $("#search");
  if (search) {
    const results = $("#search-results");
    const roadmap = $("#roadmap");
    const empty = $("#search-empty");
    const cards = $$(".ex-card", results);
    search.addEventListener("input", () => {
      const terms = search.value.toLowerCase().trim().split(/\s+/).filter(Boolean);
      const active = terms.length > 0;
      let shown = 0;
      cards.forEach((card) => {
        const match = active && terms.every((t) => card.dataset.search.includes(t));
        card.hidden = !match;
        if (match) shown += 1;
      });
      results.hidden = !active;
      roadmap.hidden = active;
      empty.hidden = !active || shown > 0;
    });
    document.addEventListener("keydown", (e) => {
      if (e.key === "/" && document.activeElement !== search) { e.preventDefault(); search.focus(); }
    });
  }
})();
