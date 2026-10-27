import type { KeyboardEvent } from "react";
import { useT } from "../infra/i18n/i18n";
import { eventKey } from "../infra/keys/keys";
import { Button } from "../widgets/Button/Button";
import { CheckField, SelectField } from "../widgets/Form/Form";
import { SearchField, type SearchOptions } from "../widgets/SearchField/SearchField";
import { TextField } from "../widgets/TextField/TextField";
import { ToolbarModes } from "../widgets/Toolbar/Toolbar";
import { findAll, replaceAll } from "./findActions";
import { LOOK_IN, type LookIn } from "./findStore";
import { FIND_IN_FILES_PANEL, FIND_MODE } from "./findUi";
import { type FindForm, useFindForm } from "./useFindForm";
import "./find.css";

/** Alt+<key> toggles an option, as in Quick Find. */
const OPTION_KEYS: Record<string, keyof SearchOptions> = { c: "matchCase", w: "wholeWord", e: "regex" };

/** The fields' element ids (their labels point at them). */
const FIELD = { find: "sp-fif-find", replace: "sp-fif-replace", lookIn: "sp-fif-lookin", mask: "sp-fif-mask" } as const;

/** What File types shows when empty. */
const MASK_PLACEHOLDER = "*.rs;*.toml;!*.lock";

/**
 * Visual Studio's Find and Replace window (Find in Files / Replace in Files):
 * a tool window that opens floating and docks anywhere, like any other.
 * Find All lists every hit in Find Results; Replace All rewrites them —
 * open documents in their editors (undoable, left unsaved), every other
 * file on disk, after a confirmation.
 */
export function FindInFilesWindow() {
  const form = useFindForm();
  const { shell, mode, setMode, setRemembered } = form;

  function onKeyDown(e: KeyboardEvent) {
    // Escape closes it while it floats, like a dialog; docked, it stays.
    if (e.key === "Escape" && shell.isToolWindowFloating(FIND_IN_FILES_PANEL)) {
      e.preventDefault();
      shell.hideToolWindow(FIND_IN_FILES_PANEL);
      return;
    }
    const option = e.altKey && !e.ctrlKey && !e.metaKey ? OPTION_KEYS[eventKey(e.nativeEvent)] : undefined;
    if (option) {
      e.preventDefault();
      setRemembered((r) => ({ ...r, [option]: !r[option] }));
      return;
    }
    if (e.key === "Enter" && e.target instanceof HTMLInputElement) {
      e.preventDefault();
      if (mode === FIND_MODE.replace) void replaceAll(form);
      else findAll(form);
    }
  }

  return (
    <div className="sp-fif-body" onKeyDown={onKeyDown}>
      <div className="sp-fif-modes">
        <ToolbarModes
          showLabels
          value={mode}
          onChange={setMode}
          modes={[
            { value: FIND_MODE.find, label: "Find in Files" },
            { value: FIND_MODE.replace, label: "Replace in Files" },
          ]}
        />
      </div>
      <PatternFields form={form} />
      <LookInFields form={form} />
      <FindButtons form={form} />
    </div>
  );
}

/** Find what, and in Replace mode Replace with. */
function PatternFields({ form }: { form: FindForm }) {
  const t = useT();
  const { mode, pattern, setPattern, options, setRemembered, replacement, setReplacement, findRef } = form;
  return (
    <>
      <label className="sp-fif-label" htmlFor={FIELD.find}>
        {t("Find what:")}
      </label>
      <SearchField
        id={FIELD.find}
        inputRef={findRef}
        value={pattern}
        onValueChange={setPattern}
        options={options}
        onOptionsChange={(o) => setRemembered((r) => ({ ...r, ...o }))}
        placeholder={t("Search text")}
      />
      {mode === FIND_MODE.replace && (
        <>
          <label className="sp-fif-label" htmlFor={FIELD.replace}>
            {t("Replace with:")}
          </label>
          <TextField id={FIELD.replace} className="sp-fif-input" value={replacement} spellCheck={false} onChange={(e) => setReplacement(e.target.value)} />
          {options.regex && <div className="sp-fif-hint">{t("$1, $2, ${name} insert the captured groups.")}</div>}
        </>
      )}
    </>
  );
}

/** Look in and File types, with what Entire project skips. */
function LookInFields({ form }: { form: FindForm }) {
  const t = useT();
  const { mode, lookIn, available, remembered, setRemembered } = form;
  const project = lookIn === LOOK_IN.project;
  return (
    <>
      <div className="sp-fif-row">
        <div className="sp-fif-col">
          <label className="sp-fif-label" htmlFor={FIELD.lookIn}>
            {t("Look in:")}
          </label>
          <SelectField
            id={FIELD.lookIn}
            value={lookIn}
            onChange={(v) => setRemembered((r) => ({ ...r, lookIn: v as LookIn }))}
            options={[
              { value: LOOK_IN.current, label: t("Current document"), disabled: !available.current },
              { value: LOOK_IN.open, label: t("All open documents"), disabled: !available.open },
              { value: LOOK_IN.project, label: t("Entire project"), disabled: !available.project },
            ]}
          />
        </div>
        <div className="sp-fif-col">
          <label className="sp-fif-label" htmlFor={FIELD.mask}>
            {t("File types:")}
          </label>
          <TextField
            id={FIELD.mask}
            className="sp-fif-input"
            value={remembered.fileMask}
            spellCheck={false}
            placeholder={MASK_PLACEHOLDER}
            onChange={(e) => setRemembered((r) => ({ ...r, fileMask: e.target.value }))}
          />
        </div>
      </div>
      {project && <div className="sp-fif-hint">{t("Includes subfolders; skips .gitignore'd files, target/, node_modules/ and binaries.")}</div>}
      {project && mode === FIND_MODE.find && (
        <div className="sp-fif-hint">
          <CheckField checked={!!remembered.includeGenerated} onChange={(on) => setRemembered((r) => ({ ...r, includeGenerated: on }))} label={t("Include generated files")} />
        </div>
      )}
    </>
  );
}

/** Find All; in Replace mode also Replace All (the default). */
function FindButtons({ form }: { form: FindForm }) {
  const t = useT();
  const { mode, canRun } = form;
  return (
    <div className="sp-fif-buttons">
      {mode === FIND_MODE.find ? (
        <Button variant="primary" disabled={!canRun} onClick={() => findAll(form)}>
          {t("Find All")}
        </Button>
      ) : (
        <>
          <Button disabled={!canRun} onClick={() => findAll(form)}>
            {t("Find All")}
          </Button>
          <Button variant="primary" disabled={!canRun} onClick={() => void replaceAll(form)}>
            {t("Replace All")}
          </Button>
        </>
      )}
    </div>
  );
}
