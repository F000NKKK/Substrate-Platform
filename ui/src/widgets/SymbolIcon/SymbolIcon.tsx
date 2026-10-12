import type { ReactNode } from "react";
import "./SymbolIcon.css";

/**
 * What a code-navigation result is — the language-neutral set Code Search,
 * an outline or a symbol picker shows icons for. Languages map their own
 * constructs onto these (a Rust trait is an `interface`-like `trait`, an
 * impl method a `method`).
 */
export type SymbolKind =
  | "file"
  | "text"
  | "module"
  | "struct"
  | "class"
  | "enum"
  | "trait"
  | "interface"
  | "typeAlias"
  | "union"
  | "function"
  | "method"
  | "field"
  | "variant"
  | "const"
  | "static"
  | "macro"
  | "associatedType";

type Family = "type" | "interface" | "function" | "field" | "constant" | "module" | "plain";

const FAMILY: Record<SymbolKind, Family> = {
  file: "plain",
  text: "plain",
  module: "module",
  struct: "type",
  class: "type",
  enum: "type",
  union: "type",
  typeAlias: "type",
  associatedType: "type",
  trait: "interface",
  interface: "interface",
  function: "function",
  method: "function",
  macro: "function",
  field: "field",
  variant: "constant",
  const: "constant",
  static: "field",
};

// 16×16 outlines, stroked in the kind's family color.
const GLYPHS: Record<SymbolKind, ReactNode> = {
  file: (
    <>
      <path d="M4 1.5h5l3 3v10H4z" />
      <path d="M9 1.5v3h3" />
    </>
  ),
  text: (
    <>
      <path d="M2.5 4h11M2.5 8h11M2.5 12h7" />
    </>
  ),
  module: <path d="M6 2.5c-2 0-2 1-2 2.5s-1 2.5-2 3c1 .5 2 1.5 2 3s0 2.5 2 2.5M10 2.5c2 0 2 1 2 2.5s1 2.5 2 3c-1 .5-2 1.5-2 3s0 2.5-2 2.5" />,
  struct: (
    <>
      <rect x="2" y="2.5" width="12" height="11" rx="1.5" />
      <path d="M5 6h6M5 10h6" />
    </>
  ),
  class: (
    <>
      <rect x="2" y="2.5" width="12" height="11" rx="1.5" />
      <path d="M5 6h6M5 10h6" />
    </>
  ),
  union: (
    <>
      <rect x="2" y="2.5" width="12" height="11" rx="1.5" />
      <path d="M5.5 5.5v3a2.5 2.5 0 0 0 5 0v-3" />
    </>
  ),
  enum: (
    <>
      <rect x="2" y="2.5" width="12" height="11" rx="1.5" />
      <path d="M10.5 5.5h-5v5h5M5.5 8h4" />
    </>
  ),
  typeAlias: (
    <>
      <rect x="2" y="2.5" width="12" height="11" rx="1.5" />
      <path d="M5 5.5h6M8 5.5v5" />
    </>
  ),
  associatedType: (
    <>
      <rect x="3" y="3.5" width="10" height="9" rx="1.5" strokeDasharray="2 1.5" />
      <path d="M5.5 6h5M8 6v4" />
    </>
  ),
  trait: (
    <>
      <circle cx="10" cy="8" r="3.5" />
      <path d="M1.5 8h5" />
    </>
  ),
  interface: (
    <>
      <circle cx="10" cy="8" r="3.5" />
      <path d="M1.5 8h5" />
    </>
  ),
  function: (
    <>
      <path d="M8 1.5l5.5 3v7L8 14.5l-5.5-3v-7z" />
      <path d="M2.5 4.5L8 7.5l5.5-3M8 7.5v7" />
    </>
  ),
  method: (
    <>
      <path d="M8 1.5l5.5 3v7L8 14.5l-5.5-3v-7z" />
      <path d="M2.5 4.5L8 7.5l5.5-3M8 7.5v7" />
    </>
  ),
  macro: (
    <>
      <path d="M8 1.5l5.5 3v7L8 14.5l-5.5-3v-7z" />
      <path d="M8 5v4M8 11v.5" />
    </>
  ),
  field: (
    <>
      <path d="M8 2l6 6-6 6-6-6z" />
    </>
  ),
  static: (
    <>
      <path d="M8 2l6 6-6 6-6-6z" />
      <path d="M8 6v4" />
    </>
  ),
  variant: (
    <>
      <rect x="2.5" y="4.5" width="11" height="7" rx="1" />
      <path d="M5.5 8h5" />
    </>
  ),
  const: (
    <>
      <rect x="2.5" y="3.5" width="11" height="9" rx="1" />
      <path d="M5.5 6.5h5M5.5 9.5h5" />
    </>
  ),
};

export interface SymbolIconProps {
  kind: SymbolKind | string;
  size?: number;
  className?: string;
}

/** A symbol kind's icon, colored by its family (types orange, interfaces and fields blue, functions purple — Visual Studio's scheme). Unknown kinds show as a plain text glyph. */
export function SymbolIcon({ kind, size = 16, className }: SymbolIconProps) {
  const known = (kind in GLYPHS ? kind : "text") as SymbolKind;
  return (
    <svg
      className={["sp-symbol-icon", className].filter(Boolean).join(" ")}
      data-family={FAMILY[known]}
      width={size}
      height={size}
      viewBox="0 0 16 16"
      fill="none"
      stroke="currentColor"
      strokeWidth={1.3}
      strokeLinecap="round"
      strokeLinejoin="round"
      aria-hidden="true"
    >
      {GLYPHS[known]}
    </svg>
  );
}
