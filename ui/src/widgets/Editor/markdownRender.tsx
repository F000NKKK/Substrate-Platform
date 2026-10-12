/// <reference path="./markdown-it-footnote.d.ts" />
import markdownIt, { type MarkdownIt, type Token } from "markdown-it";
import footnote from "markdown-it-footnote";
import githubAlerts from "markdown-it-github-alerts";
import katexPlugin from "@vscode/markdown-it-katex";
import katex from "katex";
import hljs from "highlight.js";
import DOMPurify from "dompurify";

// Markdown to HTML the way VS Code's preview does it: markdown-it with GFM
// (tables, strikethrough, autolinks, task lists), footnotes, GitHub alerts
// (`> [!NOTE]`), KaTeX math (`$...$`, `$$...$$`), highlight.js code blocks,
// and ```mermaid fences left as placeholders for the preview to draw. Every
// block carries `data-line` (its 0-based source line) for scroll sync and
// double-click-to-source, like VS Code's own preview.

function escapeHtml(text: string): string {
  return text.replace(/[&<>"]/g, (c) => ({ "&": "&amp;", "<": "&lt;", ">": "&gt;", '"': "&quot;" })[c]!);
}

/** GitHub-style heading anchors: lowercase, punctuation dropped, spaces to dashes, repeats numbered. */
function slugify(text: string, seen: Map<string, number>): string {
  const base = text
    .trim()
    .toLowerCase()
    .replace(/[^\p{L}\p{N}\s_-]/gu, "")
    .replace(/\s/g, "-");
  const count = seen.get(base) ?? 0;
  seen.set(base, count + 1);
  return count === 0 ? base : `${base}-${count}`;
}

/** `[ ]` / `[x]` at the start of a list item becomes a checkbox that knows its source line. */
function taskLists(md: MarkdownIt) {
  md.core.ruler.after("inline", "task-lists", (state) => {
    const tokens = state.tokens;
    for (let i = 2; i < tokens.length; i++) {
      const inline = tokens[i];
      if (inline.type !== "inline" || tokens[i - 1].type !== "paragraph_open" || tokens[i - 2].type !== "list_item_open") continue;
      const match = /^\[([ xX])\]\s/.exec(inline.content);
      const first = inline.children?.[0];
      if (!match || !first || first.type !== "text") continue;
      const item = tokens[i - 2];
      const checked = match[1] !== " ";
      const line = item.map?.[0] ?? 0;
      first.content = first.content.slice(match[0].length);
      const box = new state.Token("html_inline", "", 0);
      box.content = `<input type="checkbox" class="sp-md-task" data-line="${line}"${checked ? " checked" : ""}> `;
      inline.children!.unshift(box);
      item.attrJoin("class", "sp-md-task-item");
      for (let j = i - 3; j >= 0; j--) {
        if ((tokens[j].type === "bullet_list_open" || tokens[j].type === "ordered_list_open") && tokens[j].level === item.level - 1) {
          tokens[j].attrJoin("class", "sp-md-task-list");
          break;
        }
      }
    }
  });
}

function sourceLines(md: MarkdownIt) {
  md.core.ruler.push("source-lines", (state) => {
    const slugs = new Map<string, number>();
    state.tokens.forEach((token: Token, i: number) => {
      if (token.map && token.nesting >= 0 && token.block) token.attrSet("data-line", String(token.map[0]));
      if (token.type === "heading_open") {
        const text = state.tokens[i + 1]?.children?.map((c) => c.content).join("") ?? "";
        token.attrSet("id", slugify(text, slugs));
      }
    });
  });
}

const md: MarkdownIt = markdownIt({
  html: true,
  linkify: true,
  highlight(code, lang) {
    if (lang && hljs.getLanguage(lang)) {
      try {
        return `<pre class="hljs"><code>${hljs.highlight(code, { language: lang, ignoreIllegals: true }).value}</code></pre>`;
      } catch {
        // Falls through to the plain rendering below.
      }
    }
    return `<pre class="hljs"><code>${escapeHtml(code)}</code></pre>`;
  },
})
  .use(footnote)
  .use(githubAlerts)
  .use(katexPlugin, { katex, enableFencedBlocks: true })
  .use(taskLists)
  .use(sourceLines);

// ```mermaid fences become placeholders the preview renders into SVG.
const defaultFence = md.renderer.rules.fence!;
md.renderer.rules.fence = (tokens, idx, options, env, self) => {
  const token = tokens[idx];
  if (token.info.trim().split(/\s+/)[0] === "mermaid") {
    return `<div class="sp-md-mermaid" data-line="${token.map?.[0] ?? 0}" data-source="${escapeHtml(token.content)}"></div>`;
  }
  // markdown-it's own fence wraps highlight's `<pre>` in nothing more —
  // add the source line to it.
  return defaultFence(tokens, idx, options, env, self).replace(/^<pre/, `<pre data-line="${token.map?.[0] ?? 0}"`);
};

/** Front matter (`---` YAML at the very top) isn't shown, as in VS Code — blanked line for line so `data-line`s still match the source. */
function hideFrontMatter(source: string): string {
  const match = /^---\r?\n[\s\S]*?\r?\n---\r?\n/.exec(source);
  return match ? match[0].replace(/[^\n]/g, "") + source.slice(match[0].length) : source;
}

/** Sanitized HTML for `source`; raw HTML in the Markdown is allowed, scripts and handlers aren't. */
export function renderMarkdown(source: string): string {
  const html = md.render(hideFrontMatter(source));
  return DOMPurify.sanitize(html, {
    USE_PROFILES: { html: true, svg: true, mathMl: true },
    ADD_ATTR: ["data-line", "data-source", "target"],
  });
}
