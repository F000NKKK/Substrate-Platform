// Injected before the page loads: just enough of Tauri's IPC for a shell to boot with no host process.

export function installTauriMock() {
  let callbackId = 1;
  window.__TAURI_EVENT_PLUGIN_INTERNALS__ = { unregisterListener() {} };
  window.__TAURI_INTERNALS__ = {
    metadata: { currentWindow: { label: "main" }, currentWebview: { label: "main", windowLabel: "main" } },
    transformCallback: (callback) => {
      const id = callbackId++;
      window[`_${id}`] = callback;
      return id;
    },
    unregisterCallback() {},
    convertFileSrc: (path) => path,
    invoke: async (command) => {
      if (command === "plugin:event|listen") return 1;
      if (command === "plugin:window|is_maximized") return true;
      if (command === "plugin:app|version") return "0.0.0";
      return null;
    },
  };
}
