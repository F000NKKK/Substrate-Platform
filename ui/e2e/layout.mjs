// Headless layout check of a running app built on the shell (Meridian IDLE, Yog IDLE, ...):
//   npm run test:layout            (app at http://localhost:1420, or LAYOUT_URL=...)
// Loads it with a mocked Tauri IPC at several window sizes and device scale
// factors, with classic (non-overlay) scrollbars like GTK WebKit's, and fails
// on page overflow, stray scrollbars, overlapping panels or a clipped status
// bar. Playwright is not a dependency: it must be resolvable (NODE_PATH).
// LAYOUT_SHOTS=dir saves a screenshot per case.

import { createRequire } from "node:module";
import { auditLayout, SCROLL_ALLOWLIST } from "./audit.mjs";
import { installTauriMock } from "./tauriMock.mjs";

const { chromium } = createRequire(import.meta.url)("playwright");
const url = process.env.LAYOUT_URL ?? "http://localhost:1420/";
const sizes = [[1920, 1034], [1366, 768], [1280, 720], [2560, 1440]];
const scales = [1, 1.25, 1.5, 2];

const browser = await chromium.launch({ ignoreDefaultArgs: ["--hide-scrollbars"], args: ["--disable-features=OverlayScrollbar"] });
let failures = 0;
for (const [width, height] of sizes) {
  for (const deviceScaleFactor of scales) {
    const context = await browser.newContext({ viewport: { width, height }, deviceScaleFactor });
    const page = await context.newPage();
    await page.addInitScript(installTauriMock);
    await page.goto(url);
    await page.waitForSelector(".sp-shell");
    await page.waitForTimeout(1500);
    const problems = await page.evaluate(auditLayout, SCROLL_ALLOWLIST);
    const name = `${width}x${height}@${deviceScaleFactor}`;
    if (process.env.LAYOUT_SHOTS) await page.screenshot({ path: `${process.env.LAYOUT_SHOTS}/${name}.png` });
    console.log(`${problems.length ? "FAIL" : "ok  "} ${name}`);
    for (const problem of problems) console.log(`     ${problem}`);
    failures += problems.length;
    await context.close();
  }
}
await browser.close();
process.exit(failures ? 1 : 0);
