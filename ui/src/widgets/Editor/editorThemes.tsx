import { HighlightStyle, syntaxHighlighting } from "@codemirror/language";
import { tags as t } from "@lezer/highlight";
import { EditorView } from "codemirror";
import type { Extension } from "@codemirror/state";
import type { EditorThemeId } from "../../infra/theme";
import type { HslColor } from "../../infra/color";

function selectionCss(color: HslColor): string {
  return `hsl(${color.h} ${color.s}% ${color.l}% / 0.35)`;
}

const vsDarkHighlightStyle = HighlightStyle.define([
  { tag: [t.keyword, t.atom, t.bool, t.special(t.variableName)], color: "#569cd6" },
  { tag: [t.name, t.deleted, t.character, t.propertyName, t.macroName], color: "#9cdcfe" },
  { tag: [t.function(t.variableName), t.labelName], color: "#dcdcaa" },
  { tag: [t.color, t.constant(t.name), t.standard(t.name)], color: "#4fc1ff" },
  { tag: t.definition(t.name), color: "#9cdcfe" },
  { tag: [t.typeName, t.className, t.number, t.changed, t.annotation, t.modifier, t.self, t.namespace], color: "#4ec9b0" },
  { tag: [t.operator, t.operatorKeyword, t.url, t.escape, t.regexp, t.link, t.special(t.string)], color: "#d4d4d4" },
  { tag: [t.meta, t.comment], color: "#6a9955", fontStyle: "italic" },
  { tag: t.strong, fontWeight: "bold" },
  { tag: t.emphasis, fontStyle: "italic" },
  { tag: t.strikethrough, textDecoration: "line-through" },
  { tag: t.link, color: "#4ec9b0", textDecoration: "underline" },
  { tag: t.heading, fontWeight: "bold", color: "#569cd6" },
  { tag: [t.processingInstruction, t.string, t.inserted], color: "#ce9178" },
  { tag: t.invalid, color: "#f44747" },
]);

/**
 * A VS Dark+-like editor theme — the harsh, disconnected default CodeMirror
 * colors clashed badly against this app's otherwise muted dark chrome, so
 * this is the default (see `defaultEditorTheme` in `infra/theme/presets`).
 * `selectionColor` is threaded in separately from everything else here since
 * it's independently user-configurable (`ThemeContext.selectionColor`), not
 * baked into the syntax palette.
 */
function vsDarkTheme(selectionColor: HslColor): Extension {
  return [
    EditorView.theme(
      {
        "&": { backgroundColor: "var(--sp-surface-0)", color: "#d4d4d4" },
        ".cm-content": { caretColor: "#d4d4d4" },
        ".cm-cursor, .cm-dropCursor": { borderLeftColor: "#d4d4d4" },
        "&.cm-focused .cm-selectionBackground, .cm-selectionBackground, .cm-content ::selection": {
          backgroundColor: selectionCss(selectionColor),
        },
        ".cm-gutters": { backgroundColor: "var(--sp-surface-1)", color: "var(--sp-text-faint)", border: "none" },
        ".cm-activeLine": { backgroundColor: "rgba(255, 255, 255, 0.04)" },
        ".cm-activeLineGutter": { backgroundColor: "rgba(255, 255, 255, 0.04)" },
        ".cm-matchingBracket, .cm-nonmatchingBracket": { backgroundColor: "rgba(255, 255, 255, 0.08)" },
      },
      { dark: true }
    ),
    syntaxHighlighting(vsDarkHighlightStyle),
  ];
}

/** Just the selection-color override, for the "classic" (plain CodeMirror default) profile — every other color stays whatever `basicSetup` ships. */
function classicTheme(selectionColor: HslColor): Extension {
  return EditorView.theme({
    "&.cm-focused .cm-selectionBackground, .cm-selectionBackground, .cm-content ::selection": {
      backgroundColor: selectionCss(selectionColor),
    },
  });
}

export function resolveEditorTheme(id: EditorThemeId, selectionColor: HslColor): Extension {
  return id === "vs-dark" ? vsDarkTheme(selectionColor) : classicTheme(selectionColor);
}
