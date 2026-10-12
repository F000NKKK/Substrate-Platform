import { useSyncExternalStore } from "react";
import type { EditorComponent } from "./EditorBase";
import { ImageEditor } from "./ImageEditor";
import { PropertiesGridEditor } from "./PropertiesGridEditor";
import {
  GroovyEditor,
  JavaEditor,
  JsonEditor,
  MarkdownEditor,
  PlainTextEditor,
  PropertiesTextEditor,
  PythonEditor,
  RustEditor,
  ShellEditor,
  TomlEditor,
  XmlEditor,
  YamlEditor,
} from "./languageEditors";

/**
 * One way of viewing/editing a kind of file — the platform's counterpart to
 * Visual Studio's editor factories. A file can have several (a `.resx`-style
 * designer grid *and* its raw text, an image preview *and* the SVG source):
 * the host shows the highest-`priority` one by default and lets the user
 * switch between them (`MultiViewEditor`) or pick one ("Open With…").
 *
 * Products add their own with `registerEditorProvider` — a designer for a
 * file format only that product knows doesn't need any change here.
 */
export interface EditorProvider {
  /** Stable, unique — what a host remembers as the user's chosen view. */
  id: string;
  /** Shown on the view switcher and in "Open With…" (e.g. "Designer", "Code"). */
  label: string;
  /** Lowercase extensions without the dot this applies to. */
  extensions?: string[];
  /** Finer matching than extensions (e.g. only `lang/*.json`) — checked on the absolute path; when both are set, both must match. */
  match?: (path: string) => boolean;
  component: EditorComponent;
  /** The default view is the matching provider with the highest priority; ties go to the one registered last. Text editors are 0, built-in designers/previews 10. */
  priority?: number;
  /**
   * `"text"` — edits the file as text (gets the host's text-editor features:
   * go-to-line, breakpoints, Find). `"designer"` — a structured view over the
   * same text. `"binary"` — works on the file's bytes and ignores `content`
   * (see `isBinaryFile`).
   */
  kind: "text" | "designer" | "binary";
}

const providers: EditorProvider[] = [];
const subscribers = new Set<() => void>();
let revision = 0;

function changed() {
  revision++;
  subscribers.forEach((notify) => notify());
}

/** Adds an editor for some kind of file — returns the matching unregister. */
export function registerEditorProvider(provider: EditorProvider): () => void {
  providers.push(provider);
  changed();
  return () => {
    const index = providers.indexOf(provider);
    if (index < 0) return;
    providers.splice(index, 1);
    changed();
  };
}

function extensionOf(path: string): string {
  const name = path.split(/[/\\]/).pop() ?? path;
  return name.includes(".") ? (name.split(".").pop()?.toLowerCase() ?? "") : "";
}

/** Every editor that can open `path`, default first. Never empty — plain text opens anything. */
export function editorProvidersFor(path: string): EditorProvider[] {
  const ext = extensionOf(path);
  const matching = providers
    .map((provider, order) => ({ provider, order }))
    .filter(({ provider }) => (!provider.extensions || provider.extensions.includes(ext)) && (!provider.match || provider.match(path)))
    .filter(({ provider }) => provider.extensions || provider.match)
    .sort((a, b) => (b.provider.priority ?? 0) - (a.provider.priority ?? 0) || b.order - a.order)
    .map(({ provider }) => provider);
  if (!matching.some((p) => p.kind === "text") && !matching.some((p) => p.kind === "binary")) {
    matching.push(plainText);
  }
  return matching;
}

function subscribe(notify: () => void) {
  subscribers.add(notify);
  return () => {
    subscribers.delete(notify);
  };
}

/** `editorProvidersFor(path)`, re-rendering the caller when providers are registered or removed. */
export function useEditorProviders(path: string): EditorProvider[] {
  useSyncExternalStore(subscribe, () => revision);
  return editorProvidersFor(path);
}

/** Resolves which editor opens `filename` by default — falls back to plain, unhighlighted text for anything unrecognized rather than refusing to open it. */
export function resolveEditor(filename: string): EditorComponent {
  return editorProvidersFor(filename)[0].component;
}

/**
 * Whether every editor for `filename` works on the file's raw bytes rather
 * than its text (e.g. a PNG's `ImageEditor`) — a host should skip reading
 * such a file as UTF-8 text before opening it (that read fails for binary
 * content) and pass an empty `content` instead. An SVG isn't: its preview
 * is binary, but its source view needs the text.
 */
export function isBinaryFile(filename: string): boolean {
  return editorProvidersFor(filename).every((p) => p.kind === "binary");
}

const plainText: EditorProvider = { id: "substrate.text", label: "Text", component: PlainTextEditor, kind: "text" };

const builtIns: EditorProvider[] = [
  { id: "substrate.json", label: "Code", extensions: ["json"], component: JsonEditor, kind: "text" },
  { id: "substrate.rust", label: "Code", extensions: ["rs"], component: RustEditor, kind: "text" },
  { id: "substrate.xml", label: "Code", extensions: ["xml", "svg"], component: XmlEditor, kind: "text" },
  { id: "substrate.java", label: "Code", extensions: ["java"], component: JavaEditor, kind: "text" },
  { id: "substrate.markdown", label: "Code", extensions: ["md", "markdown"], component: MarkdownEditor, kind: "text" },
  { id: "substrate.toml", label: "Code", extensions: ["toml"], component: TomlEditor, kind: "text" },
  { id: "substrate.yaml", label: "Code", extensions: ["yml", "yaml"], component: YamlEditor, kind: "text" },
  { id: "substrate.shell", label: "Code", extensions: ["sh", "bash"], component: ShellEditor, kind: "text" },
  { id: "substrate.groovy", label: "Code", extensions: ["gradle", "groovy"], component: GroovyEditor, kind: "text" },
  { id: "substrate.python", label: "Code", extensions: ["py"], component: PythonEditor, kind: "text" },
  { id: "substrate.properties.text", label: "Code", extensions: ["properties"], component: PropertiesTextEditor, kind: "text" },
  { id: "substrate.properties.grid", label: "Designer", extensions: ["properties"], component: PropertiesGridEditor, priority: 10, kind: "designer" },
  {
    id: "substrate.image",
    label: "Preview",
    extensions: ["png", "jpg", "jpeg", "gif", "webp", "bmp", "ico", "svg"],
    component: ImageEditor,
    priority: 10,
    kind: "binary",
  },
];
builtIns.forEach((provider) => providers.push(provider));
