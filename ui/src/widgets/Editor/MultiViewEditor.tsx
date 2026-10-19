import type { ReactNode } from "react";
import { ErrorBoundary } from "../ErrorBoundary";
import { Tab } from "../Tab";
import { useT } from "../../infra/i18n";
import type { EditorProps } from "./EditorBase";
import { EDITOR_KIND, type EditorProvider } from "./registry";
import { recordDocumentChange, setDocumentWriter } from "../../infra/history";
import "./MultiViewEditor.css";

export interface MultiViewEditorProps extends EditorProps {
  /** The views to offer, default first — usually `useEditorProviders(path)`. */
  providers: EditorProvider[];
  /** The chosen view's provider id; unset or unknown falls back to the first provider. */
  view?: string;
  onViewChange?: (providerId: string) => void;
  /**
   * Renders one view — for a host that passes a view more than the shared
   * `EditorProps` (breakpoints into a text view, say). Defaults to rendering
   * `provider.component` with the editor props as-is.
   */
  renderView?: (provider: EditorProvider, props: EditorProps) => ReactNode;
  /** Set `false` when the host offers the views elsewhere (buttons in the tab strip). Default `true`. */
  showSwitcher?: boolean;
}

/**
 * One file shown through any of several editors with a view switcher along
 * the bottom — Visual Studio's Design/Code tabs under a designer. Every view
 * edits the same `content`; only the active one is mounted, so a view
 * switched to always starts from the latest text (a text view's undo
 * history doesn't survive the round trip through a designer).
 *
 * With a single provider it renders just that editor, no switcher.
 *
 * A designer's changes go into the document's history (`documentHistory`):
 * Undo/Redo work in every designer, whatever it keeps itself.
 */
export function MultiViewEditor({ providers, view, onViewChange, renderView, showSwitcher = true, ...props }: MultiViewEditorProps) {
  const t = useT();
  const active = providers.find((p) => p.id === view) ?? providers[0];
  // Text views keep CodeMirror's history; a designer's edits are recorded here.
  const { onChange, content: text, path } = props;
  if (onChange && path) setDocumentWriter(path, onChange);
  const viewProps: EditorProps =
    active.kind === EDITOR_KIND.text || !onChange || !path
      ? props
      : {
          ...props,
          onChange: (next: string) => {
            recordDocumentChange(path, text, next);
            onChange(next);
          },
        };
  // A view's crash stays in it (its message, Try Again); the switcher still works.
  const content = (
    <ErrorBoundary key={active.id} what={t(active.name ?? active.label)}>
      {renderView ? renderView(active, viewProps) : <active.component {...viewProps} />}
    </ErrorBoundary>
  );
  if (providers.length < 2 || !showSwitcher) return <>{content}</>;
  return (
    <div className="sp-multiview-editor">
      <div className="sp-multiview-editor-body">{content}</div>
      <div className="sp-multiview-editor-views" role="tablist">
        {providers.map((p) => (
          <Tab key={p.id} orientation="horizontal" active={p === active} onClick={() => onViewChange?.(p.id)}>
            {t(p.label)}
          </Tab>
        ))}
      </div>
    </div>
  );
}
