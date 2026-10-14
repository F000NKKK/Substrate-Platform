import { afterEach, describe, expect, it } from "vitest";
import { contextKey, formatShortcut, pt, registerTranslations, resolvedLocale, setLocale, t, tn } from "./i18n";

registerTranslations({ id: "xx", name: "Test" }, {
  Save: "Sauv",
  "Open {name}": "Ouv {name}",
  "{count} file": { one: "{count} fichier", other: "{count} fichiers" },
  Ctrl: "Strg",
  [contextKey("menu", "Build")]: "Menu-Build",
});
registerTranslations("ru", { "{count} file": { one: "{count} файл", few: "{count} файла", many: "{count} файлов", other: "{count} файла" } });

afterEach(() => setLocale(null));

describe("i18n", () => {
  it("falls back to the English text, region to language", () => {
    setLocale("xx-YY");
    expect(resolvedLocale()).toBe("xx");
    expect(t("Save")).toBe("Sauv");
    expect(t("Untranslated {x}", { x: 1 })).toBe("Untranslated 1");
    expect(t("Open {name}", {})).toBe("Ouv {name}");
    setLocale("zz");
    expect(resolvedLocale()).toBe("en");
    expect(t("Save")).toBe("Save");
  });

  it("tells contexts apart", () => {
    setLocale("xx");
    expect(pt("menu", "Build")).toBe("Menu-Build");
    expect(pt("menu", "Save")).toBe("Sauv");
    expect(pt("command", "Build")).toBe("Build");
  });

  it("picks the locale's plural form", () => {
    setLocale("ru");
    expect([1, 3, 5, 21, 1.5].map((n) => tn(n, "{count} file", "{count} files"))).toEqual(["1 файл", "3 файла", "5 файлов", "21 файл", "1.5 файла"]);
    setLocale("en");
    expect([1, 2].map((n) => tn(n, "{count} file", "{count} files"))).toEqual(["1 file", "2 files"]);
  });

  it("shows shortcuts with translated key names", () => {
    setLocale("xx");
    expect(formatShortcut("ctrl+shift+k esc")).toBe("Strg+Shift+K Esc");
    expect(formatShortcut("Alt+F4")).toBe("Alt+F4");
  });
});
