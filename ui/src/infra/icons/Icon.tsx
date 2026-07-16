import type { ReactNode, SVGProps } from "react";

export interface IconProps extends SVGProps<SVGSVGElement> {
  size?: number;
}

const DEFAULT_SIZE = 16;
const DEFAULT_VIEWBOX = "0 0 24 24";
const STROKE_WIDTH = 1.75;

/** Factory so every glyph in the default set shares stroke width, caps, and (unless overridden) viewBox — no glyph reinvents its own SVG shell. */
export function createIcon(paths: ReactNode, viewBox: string = DEFAULT_VIEWBOX) {
  return function Glyph({ size = DEFAULT_SIZE, ...props }: IconProps) {
    return (
      <svg
        width={size}
        height={size}
        viewBox={viewBox}
        fill="none"
        stroke="currentColor"
        strokeWidth={STROKE_WIDTH}
        strokeLinecap="round"
        strokeLinejoin="round"
        {...props}
      >
        {paths}
      </svg>
    );
  };
}
