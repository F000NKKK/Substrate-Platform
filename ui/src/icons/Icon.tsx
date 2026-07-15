import type { ReactNode, SVGProps } from "react";

export interface IconProps extends SVGProps<SVGSVGElement> {
  size?: number;
}

const DEFAULT_SIZE = 16;
const STROKE_WIDTH = 1.75;

/** Factory so every icon in the default set shares stroke width, caps, and viewBox — no icon reinvents its own SVG shell. */
export function createIcon(paths: ReactNode) {
  return function IconComponent({ size = DEFAULT_SIZE, ...props }: IconProps) {
    return (
      <svg
        width={size}
        height={size}
        viewBox="0 0 24 24"
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
