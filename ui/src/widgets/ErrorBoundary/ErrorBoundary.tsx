import { Component, type ErrorInfo, type ReactNode } from "react";
import { t } from "../../infra/i18n";
import "./ErrorBoundary.css";

interface Props {
  /** What failed, for the message ("the Recipe Editor"). */
  what?: string;
  children: ReactNode;
}

interface State {
  error: Error | null;
  stack: string;
}

/**
 * Keeps a panel's or an editor's crash inside it: the error and where it
 * happened instead of its content, Try Again to mount it afresh — the
 * rest of the IDE goes on (React unmounts the whole tree otherwise).
 */
export class ErrorBoundary extends Component<Props, State> {
  state: State = { error: null, stack: "" };

  static getDerivedStateFromError(error: Error): Partial<State> {
    return { error };
  }

  componentDidCatch(error: Error, info: ErrorInfo) {
    console.error(error, info.componentStack);
    this.setState({ stack: info.componentStack ?? "" });
  }

  render() {
    const { error, stack } = this.state;
    if (!error) return this.props.children;
    const text = `${error.name}: ${error.message}\n${error.stack ?? ""}\n${stack}`;
    return (
      <div className="sp-error-boundary" role="alert">
        <strong>{this.props.what ? t("{what} failed: {error}", { what: this.props.what, error: error.message }) : t("This panel failed: {error}", { error: error.message })}</strong>
        <pre>{text}</pre>
        <div className="sp-error-boundary-actions">
          <button type="button" onClick={() => this.setState({ error: null, stack: "" })}>
            {t("Try Again")}
          </button>
          <button type="button" onClick={() => void navigator.clipboard?.writeText(text)}>
            {t("Copy Error")}
          </button>
        </div>
      </div>
    );
  }
}
