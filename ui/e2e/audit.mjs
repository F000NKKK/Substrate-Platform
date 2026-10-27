// Runs inside the page: measures what the layout does, returns a list of human-readable problems.

/** Scroll containers that are meant to scroll (content areas, never chrome). */
const SCROLL_ALLOWLIST = [".cm-scroller", ".sp-output-window-body", ".sp-terminal-body", ".xterm-viewport", ".sp-tree", "[data-scroll]"];

export function auditLayout(allowlist) {
  const problems = [];
  const root = document.scrollingElement;
  if (root.scrollHeight > root.clientHeight) problems.push(`page scrolls vertically: ${root.scrollHeight} > ${root.clientHeight}`);
  if (root.scrollWidth > root.clientWidth) problems.push(`page scrolls horizontally: ${root.scrollWidth} > ${root.clientWidth}`);

  const describe = (el) => `${el.tagName.toLowerCase()}${el.className && typeof el.className === "string" ? "." + el.className.trim().split(/\s+/).join(".") : ""}`;
  for (const el of document.body.querySelectorAll("*")) {
    if (allowlist.some((selector) => el.matches(selector))) continue;
    const style = getComputedStyle(el);
    const scrollable = (value) => value === "auto" || value === "scroll";
    const vertical = scrollable(style.overflowY) && el.scrollHeight > el.clientHeight + 1;
    const horizontal = scrollable(style.overflowX) && el.scrollWidth > el.clientWidth + 1;
    if (vertical || horizontal) problems.push(`unexpected scrollbar on ${describe(el)}`);
  }

  const rect = (selector) => [...document.querySelectorAll(selector)].map((el) => ({ el, r: el.getBoundingClientRect() }));
  const boxes = [...rect(".sp-shell-main"), ...rect(".sp-panel-surface")].filter(({ r }) => r.width > 0 && r.height > 0);
  for (let i = 0; i < boxes.length; i++) {
    for (let j = i + 1; j < boxes.length; j++) {
      const a = boxes[i].r;
      const b = boxes[j].r;
      const overlap = Math.min(a.right, b.right) - Math.max(a.left, b.left) > 1 && Math.min(a.bottom, b.bottom) - Math.max(a.top, b.top) > 1;
      const nested = boxes[i].el.contains(boxes[j].el) || boxes[j].el.contains(boxes[i].el);
      if (overlap && !nested && !boxes[i].el.closest(".sp-flyout, .sp-floating-panel") && !boxes[j].el.closest(".sp-flyout, .sp-floating-panel"))
        problems.push(`${describe(boxes[i].el)} overlaps ${describe(boxes[j].el)}`);
    }
  }

  const shell = document.querySelector(".sp-shell")?.getBoundingClientRect();
  if (!shell) problems.push("no .sp-shell");
  else {
    if (shell.top !== 0 || shell.left !== 0) problems.push(`shell is offset: ${shell.left},${shell.top}`);
    if (Math.abs(shell.bottom - innerHeight) > 1 || Math.abs(shell.right - innerWidth) > 1) problems.push(`shell does not fill the window: ${shell.right}x${shell.bottom} in ${innerWidth}x${innerHeight}`);
  }
  const status = document.querySelector(".sp-shell-status-bar")?.getBoundingClientRect();
  if (status && (status.height < 1 || status.bottom > innerHeight + 0.5)) problems.push(`status bar is clipped: bottom ${status.bottom} of ${innerHeight}`);

  const pinned = (side) => rect(`.sp-dock--${side} .sp-panel-surface`).filter(({ r }) => r.width > 0)[0]?.r;
  const bottom = document.querySelector(".sp-shell-bottom-inset .sp-panel-surface")?.getBoundingClientRect();
  const left = pinned("left");
  if (bottom && left && Math.abs(bottom.left - left.left) > 1) problems.push(`bottom panel left ${bottom.left} does not line up with left panel ${left.left}`);
  return problems;
}

export { SCROLL_ALLOWLIST };
