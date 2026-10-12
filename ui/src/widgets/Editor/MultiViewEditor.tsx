import type { ReactNode } from "react";
import { Tab } from "../Tab";
import type { EditorProps } from "./EditorBase";
import type { EditorProvider } from "./registry";
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
}

/**
 * One file shown through any of several editors with a view switcher along
 * the bottom — Visual Studio's Design/Code tabs under a designer. Every view
 * edits the same `content`; only the active one is mounted, so a view
 * switched to always starts from the latest text (a text view's undo
 * history doesn't survive the round trip through a designer).
 *
 * With a single provider it renders just that editor, no switcher.
 */
export function MultiViewEditor({ providers, view, onViewChange, renderView, ...props }: MultiViewEditorProps) {
  const active = providers.find((p) => p.id === view) ?? providers[0];
  const content = renderView ? renderView(active, props) : <active.component {...props} />;
  if (providers.length < 2) return <>{content}</>;
  return (
    <div className="sp-multiview-editor">
      <div className="sp-multiview-editor-body">{content}</div>
      <div className="sp-multiview-editor-views" role="tablist">
        {providers.map((p) => (
          <Tab key={p.id} orientation="horizontal" active={p === active} onClick={() => onViewChange?.(p.id)}>
            {p.label}
          </Tab>
        ))}
      </div>
    </div>
  );
}
