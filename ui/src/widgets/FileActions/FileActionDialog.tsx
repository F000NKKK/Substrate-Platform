import { useState, type SyntheticEvent } from "react";
import { useT, errorText } from "../../infra/i18n";
import { Button } from "../Button";
import { FloatingWindow } from "../FloatingWindow";
import { TextField } from "../TextField";

export function FileActionDialog({ title, description, initial, submit, close }: {
  title: string; description?: string; initial?: string;
  submit(value: string): Promise<void>; close(): void;
}) {
  const t = useT();
  const [value, setValue] = useState(initial ?? ""), [busy, setBusy] = useState(false), [error, setError] = useState<string | null>(null);
  async function accept(event: SyntheticEvent) {
    event.preventDefault();
    if (busy) return;
    setBusy(true); setError(null);
    try { await submit(value); close(); }
    catch (error) { setError(errorText(error)); }
    finally { setBusy(false); }
  }
  return <FloatingWindow title={title} modal width={440} onClose={() => !busy && close()}>
    <form onSubmit={accept} style={{ padding: 16 }}>
      {description && <p>{description}</p>}
      {initial !== undefined && <TextField autoFocus aria-label={t("Name")} value={value} onChange={event => setValue(event.target.value)} disabled={busy} />}
      {error && <p role="alert">{error}</p>}
      <div style={{ display: "flex", gap: 8, marginTop: 12 }}>
        <Button type="submit" variant="primary" disabled={busy}>{t(initial === undefined ? "Delete" : "OK")}</Button>
        <Button type="button" disabled={busy} onClick={close}>{t("Cancel")}</Button>
      </div>
    </form>
  </FloatingWindow>;
}
