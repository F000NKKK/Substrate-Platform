import { describe, expect, it } from "vitest";
import { ZOOM_MAX, ZOOM_MIN, fitZoom, parseZoomPercent, stepZoom, zoomPercent } from "./zoom";

describe("zoom", () => {
  it("steps in and out from on and between steps", () => {
    expect(stepZoom(1, 1)).toBe(1.5);
    expect(stepZoom(1, -1)).toBe(0.75);
    expect(stepZoom(2.2, 1)).toBe(3);
    expect(stepZoom(2.2, -1)).toBe(2);
    expect(stepZoom(ZOOM_MAX, 1)).toBe(ZOOM_MAX);
    expect(stepZoom(ZOOM_MIN, -1)).toBe(ZOOM_MIN);
  });
  it("fits whole above 1 and exactly below", () => {
    expect(fitZoom(16, 16, 100, 60)).toBe(3);
    expect(fitZoom(400, 200, 100, 100)).toBe(0.25);
  });
  it("reads and shows percents", () => {
    expect(parseZoomPercent("250%")).toBe(2.5);
    expect(parseZoomPercent(" 12,5 % ")).toBe(0.125);
    expect(parseZoomPercent("abc")).toBeNull();
    expect(parseZoomPercent("0")).toBeNull();
    expect(zoomPercent(0.125)).toBe("12.5%");
    expect(zoomPercent(4)).toBe("400%");
  });
});
