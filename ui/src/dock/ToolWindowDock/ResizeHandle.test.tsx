import { act, createElement } from 'react';
import { createRoot } from 'react-dom/client';
import { afterEach, beforeEach, expect, it, vi } from 'vitest';
import { ResizeHandle } from './ToolWindowDock';
import type { ShellLayout } from '../hooks/useShellLayout';

const frames = new Map<number, FrameRequestCallback>();
let next = 0;
beforeEach(() => {
  (globalThis as any).IS_REACT_ACT_ENVIRONMENT = true;
  frames.clear();
  vi.stubGlobal('requestAnimationFrame', (callback: FrameRequestCallback) => { frames.set(++next, callback); return next; });
  vi.stubGlobal('cancelAnimationFrame', (id: number) => frames.delete(id));
});
afterEach(() => vi.unstubAllGlobals());

it('coalesces pointer moves and commits the last size on release', async () => {
  const host = document.createElement('div');
  const root = createRoot(host);
  const setAnchorSize = vi.fn();
  const layout = { anchorSize: () => 200, setAnchorSize } as unknown as ShellLayout;
  try {
    await act(async () => root.render(createElement(ResizeHandle, { anchor: 'bottom', panelId: 'output', mode: 'pinned', layout })));
    await act(async () => host.firstElementChild!.dispatchEvent(new PointerEvent('pointerdown', { button: 0, clientY: 300, bubbles: true })));
    for (const clientY of [290, 280, 270]) window.dispatchEvent(new PointerEvent('pointermove', { clientY }));
    expect(setAnchorSize).not.toHaveBeenCalled();
    expect(frames.size).toBe(1);
    await act(async () => window.dispatchEvent(new PointerEvent('pointerup')));
    expect(setAnchorSize).toHaveBeenCalledExactlyOnceWith('output', 'bottom', 'pinned', 230);
    expect(frames.size).toBe(0);
    window.dispatchEvent(new PointerEvent('pointermove', { clientY: 100 }));
    expect(setAnchorSize).toHaveBeenCalledTimes(1);
  } finally { await act(async () => root.unmount()); }
});

it('unmounting during a resize removes listeners and the queued frame', async () => {
  const host = document.createElement('div');
  const root = createRoot(host);
  const setAnchorSize = vi.fn();
  const layout = { anchorSize: () => 200, setAnchorSize } as unknown as ShellLayout;
  await act(async () => root.render(createElement(ResizeHandle, { anchor: 'left', panelId: 'tree', mode: 'pinned', layout })));
  await act(async () => host.firstElementChild!.dispatchEvent(new PointerEvent('pointerdown', { button: 0, clientX: 200, bubbles: true })));
  window.dispatchEvent(new PointerEvent('pointermove', { clientX: 250 }));
  expect(frames.size).toBe(1);
  await act(async () => root.unmount());
  expect(frames.size).toBe(0);
  window.dispatchEvent(new PointerEvent('pointermove', { clientX: 300 }));
  window.dispatchEvent(new PointerEvent('pointerup'));
  expect(setAnchorSize).not.toHaveBeenCalled();
});
