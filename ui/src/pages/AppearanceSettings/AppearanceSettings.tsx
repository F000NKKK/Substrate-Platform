import { useRef, useState, type ChangeEvent } from "react";
import { Label } from "../../widgets/Label";
import { AccentColorField } from "../../widgets/AccentColorField";
import { EditorColorField } from "../../widgets/EditorColorField";
import { Button } from "../../widgets/Button";
import { useTheme, editorColorKeys, isBuiltInProfile } from "../../infra/theme";
import "./AppearanceSettings.css";

/** The Settings window's "Appearance" section — the UI's accent color, and the code editor's own color scheme (a profile picker plus a fully independent color per syntax category, VS's "Fonts and Colors" model), including creating/deleting/exporting/importing custom profiles so users can share them. */
export function AppearanceSettings() {
  const {
    editorTheme,
    setEditorTheme,
    editorProfiles,
    resetEditorColors,
    createEditorProfile,
    deleteEditorProfile,
    exportEditorProfile,
    importEditorProfile,
  } = useTheme();
  const fileInputRef = useRef<HTMLInputElement>(null);
  const [importError, setImportError] = useState<string | null>(null);

  const activeIsBuiltIn = isBuiltInProfile(editorTheme);

  function handleNewProfile() {
    const name = window.prompt("New profile name:");
    if (name) createEditorProfile(name);
  }

  function handleDeleteProfile() {
    if (activeIsBuiltIn) return;
    const profile = editorProfiles.find((p) => p.id === editorTheme);
    if (profile && window.confirm(`Delete profile "${profile.name}"?`)) deleteEditorProfile(editorTheme);
  }

  function handleExportProfile() {
    const json = exportEditorProfile(editorTheme);
    const profile = editorProfiles.find((p) => p.id === editorTheme);
    const blob = new Blob([json], { type: "application/json" });
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = `${(profile?.name ?? "editor-theme").replace(/\s+/g, "-").toLowerCase()}.json`;
    a.click();
    URL.revokeObjectURL(url);
  }

  function handleImportFile(e: ChangeEvent<HTMLInputElement>) {
    const file = e.target.files?.[0];
    e.target.value = "";
    if (!file) return;
    file.text().then((json) => {
      const id = importEditorProfile(json);
      setImportError(id ? null : "That file isn't a valid editor color profile.");
    });
  }

  return (
    <div className="sp-appearance-settings">
      <Label>Accent Color</Label>
      <AccentColorField />

      <div className="sp-appearance-settings-divider" />

      <Label>Editor Color Profile</Label>
      <div className="sp-appearance-settings-row">
        <select className="sp-appearance-settings-select" value={editorTheme} onChange={(e) => setEditorTheme(e.target.value)}>
          {editorProfiles.map((p) => (
            <option key={p.id} value={p.id}>
              {p.name}
            </option>
          ))}
        </select>
        <Button variant="subtle" onClick={handleNewProfile}>
          New Profile…
        </Button>
        {!activeIsBuiltIn && (
          <Button variant="ghost" onClick={handleDeleteProfile}>
            Delete
          </Button>
        )}
        {/* Reset only makes sense for a built-in profile — a custom one has no separate default to revert to. */}
        {activeIsBuiltIn && (
          <Button variant="ghost" onClick={resetEditorColors}>
            Reset to Profile Defaults
          </Button>
        )}
      </div>
      <div className="sp-appearance-settings-row">
        <Button variant="subtle" onClick={handleExportProfile}>
          Export…
        </Button>
        <Button variant="subtle" onClick={() => fileInputRef.current?.click()}>
          Import…
        </Button>
        <input ref={fileInputRef} type="file" accept="application/json" style={{ display: "none" }} onChange={handleImportFile} />
        {importError && <span className="sp-appearance-settings-error">{importError}</span>}
      </div>

      <Label>Editor Colors</Label>
      <div className="sp-appearance-settings-colors">
        {editorColorKeys.map((key) => (
          <EditorColorField key={key} colorKey={key} />
        ))}
      </div>
    </div>
  );
}
