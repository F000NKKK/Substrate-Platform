import { createContext, useCallback, useContext, useState, type Dispatch, type ReactNode, type SetStateAction } from "react";

/**
 * Where UI state that should survive a restart lives — a flat map of
 * JSON-serializable values by key (`"shell.layout"`, `"explorer.expanded"`,
 * ...). The platform's own widgets persist through whichever store is
 * provided (`StateStoreProvider`); a product swaps in its own, e.g. one
 * backed by a per-project workspace file so every project reopens the way
 * it was left. `get` must be synchronous — a store loads its data before
 * it's provided.
 */
export interface StateStore {
  /** What this store persists for (e.g. a project root, or `"global"`) — when it changes, every `useStoredState` re-reads its key from the new store. */
  scope: string;
  get(key: string): unknown;
  /** `undefined` removes the key. */
  set(key: string, value: unknown): void;
}

const LOCAL_PREFIX = "sp-state:";

/** The default store — `localStorage`, best-effort (unavailable/corrupt storage just means "nothing to restore"). */
export const localStorageStore: StateStore = {
  scope: "local",
  get(key) {
    try {
      const raw = localStorage.getItem(LOCAL_PREFIX + key);
      return raw === null ? undefined : JSON.parse(raw);
    } catch {
      return undefined;
    }
  },
  set(key, value) {
    try {
      if (value === undefined) localStorage.removeItem(LOCAL_PREFIX + key);
      else localStorage.setItem(LOCAL_PREFIX + key, JSON.stringify(value));
    } catch {
      // Quota exceeded or unavailable — persistence is a nicety, not load-bearing.
    }
  },
};

const StateStoreContext = createContext<StateStore>(localStorageStore);

export function StateStoreProvider({ store, children }: { store: StateStore; children: ReactNode }) {
  return <StateStoreContext.Provider value={store}>{children}</StateStoreContext.Provider>;
}

/** The nearest provided store — `localStorageStore` when none is. */
export function useStateStore(): StateStore {
  return useContext(StateStoreContext);
}

/**
 * `useState`, persisted under `key` in the current `StateStore` — adding a
 * remembered piece of UI state is one line. The stored value is used when
 * present and (if `isValid` is given) valid, `defaultValue` otherwise; a
 * change of store scope (another project opened) re-reads it.
 */
export function useStoredState<T>(
  key: string,
  defaultValue: T,
  isValid?: (value: unknown) => boolean
): [T, Dispatch<SetStateAction<T>>] {
  const store = useStateStore();
  const read = (): T => {
    const stored = store.get(key);
    return stored !== undefined && (!isValid || isValid(stored)) ? (stored as T) : defaultValue;
  };
  const [state, setState] = useState(() => ({ scope: store.scope, key, value: read() }));

  // Adjusting state during render (React's documented pattern for "reset
  // when a prop changes") — avoids rendering one frame of the previous
  // project's value.
  let current = state;
  if (state.scope !== store.scope || state.key !== key) {
    current = { scope: store.scope, key, value: read() };
    setState(current);
  }

  const set = useCallback<Dispatch<SetStateAction<T>>>(
    (action) => {
      setState((prev) => {
        const value = typeof action === "function" ? (action as (prev: T) => T)(prev.value) : action;
        store.set(key, value);
        return { ...prev, value };
      });
    },
    [store, key]
  );

  return [current.value, set];
}
