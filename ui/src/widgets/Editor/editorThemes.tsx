import { HighlightStyle, syntaxHighlighting } from "@codemirror/language";
import { tags as t } from "@lezer/highlight";
import { EditorView } from "codemirror";
import type { Extension } from "@codemirror/state";
import type { EditorColorScheme } from "../../infra/theme";
import { hslToCss, type HslColor } from "../../infra/color";

function alpha(color: HslColor, a: number): string {
  return `hsl(${color.h} ${color.s}% ${color.l}% / ${a})`;
}

/**
 * Builds the CodeMirror extension for a color scheme — every entry in
 * `EditorColorScheme` maps to exactly one visual role here, so a user
 * recoloring "Keyword" in Settings only ever touches keyword-tagged tokens,
 * never anything else. Rebuilt (mount + re-run, not live-patched) whenever
 * the scheme changes, same as switching `language`.
 */
export function resolveEditorTheme(colors: EditorColorScheme): Extension {
  const highlightStyle = HighlightStyle.define([
    { tag: [t.keyword, t.atom, t.bool, t.special(t.variableName)], color: hslToCss(colors.keyword) },
    { tag: [t.processingInstruction, t.string, t.inserted], color: hslToCss(colors.string) },
    { tag: [t.meta, t.comment], color: hslToCss(colors.comment), fontStyle: "italic" },
    { tag: [t.number, t.changed], color: hslToCss(colors.number) },
    { tag: [t.typeName, t.className, t.annotation, t.modifier, t.self, t.namespace], color: hslToCss(colors.type) },
    { tag: [t.function(t.variableName), t.labelName], color: hslToCss(colors.function) },
    { tag: [t.name, t.deleted, t.character, t.propertyName, t.macroName, t.definition(t.name)], color: hslToCss(colors.variable) },
    { tag: [t.operator, t.operatorKeyword, t.url, t.escape, t.regexp, t.special(t.string)], color: hslToCss(colors.operator) },
    { tag: t.link, color: hslToCss(colors.function), textDecoration: "underline" },
    { tag: t.strong, fontWeight: "bold" },
    { tag: t.emphasis, fontStyle: "italic" },
    { tag: t.strikethrough, textDecoration: "line-through" },
    { tag: t.heading, fontWeight: "bold", color: hslToCss(colors.keyword) },
    { tag: t.invalid, color: "#f44747" },
  ]);

  const viewTheme = EditorView.theme(
    {
      "&": { backgroundColor: hslToCss(colors.background), color: hslToCss(colors.foreground) },
      ".cm-content": { caretColor: hslToCss(colors.cursor) },
      ".cm-cursor, .cm-dropCursor": { borderLeftColor: hslToCss(colors.cursor) },
      "&.cm-focused .cm-selectionBackground, .cm-selectionBackground, .cm-content ::selection": {
        backgroundColor: alpha(colors.selection, 0.35),
      },
      ".cm-gutters": { backgroundColor: hslToCss(colors.background), color: alpha(colors.foreground, 0.5), border: "none" },
      ".cm-activeLine": { backgroundColor: alpha(colors.foreground, 0.05) },
      ".cm-activeLineGutter": { backgroundColor: alpha(colors.foreground, 0.05) },
      ".cm-matchingBracket, .cm-nonmatchingBracket": { backgroundColor: alpha(colors.foreground, 0.1) },
    },
    { dark: colors.background.l < 50 }
  );

  return [viewTheme, syntaxHighlighting(highlightStyle)];
}
