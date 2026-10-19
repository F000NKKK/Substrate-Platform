// Panels in a shell are independent components with no prop channel
// between them — this is the glue that lets one (an explorer, a debugger,
// an error list) tell the editor area "open this file" from anywhere in the
// dock tree. One per app: a module-level bus.

export interface FileOpenRequest {
  path: string;
  /** Tab title — defaults to the path's last segment. */
  name?: string;
  /** 1-based line to reveal and put the cursor on (also for an already-open file). */
  line?: number;
  /** 1-based column on `line` (default: the line's start). */
  column?: number;
  /** Editor provider id to show the file with ("Open With…") — omitted, the tab keeps its view (a new one opens in the default). */
  view?: string;
}

type Listener = (request: FileOpenRequest) => void;

const listeners = new Set<Listener>();

/**
 * Opens `path` in an editor tab — or activates its tab if it's already
 * open — and, given `line`, scrolls to it and puts the cursor there:
 * `openFile({ path, line: 42 })`. A file that can't be read is reported
 * (Output + a notification), never thrown.
 */
export function openFile(request: FileOpenRequest): void {
  listeners.forEach((listener) => listener(request));
}

export function onOpenFileRequest(listener: Listener): () => void {
  listeners.add(listener);
  return () => listeners.delete(listener);
}
