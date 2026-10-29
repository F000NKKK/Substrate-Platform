import { act, createElement } from 'react';
import { createRoot } from 'react-dom/client';
import { expect, it } from 'vitest';
import { PlatformShell } from './PlatformShell';

it('switches every tab in a group without remounting the dock', async () => {
  (globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;
  const host = document.createElement('div');
  const root = createRoot(host);
  const panel = (id: string) => ({ id, title: id, component: () => createElement('p', null, `content ${id}`) });
  await act(async () => root.render(createElement(PlatformShell, {
    main: panel('main'),
    toolWindows: { bottom: [panel('assets'), panel('output'), panel('terminal')] },
    defaultPinned: { bottom: 'assets' },
    tabbedAnchors: ['bottom'],
  })));
  try {
    for (const id of ['output', 'terminal', 'assets', 'terminal', 'output']) {
      const tab = [...host.querySelectorAll<HTMLButtonElement>('[role="tab"]')].find(button => button.title === id)!;
      await act(async () => tab.click());
      expect(host.querySelector('.sp-tabgroup-body')?.textContent).toBe(`content ${id}`);
    }
  } finally {
    await act(async () => root.unmount());
  }
});
