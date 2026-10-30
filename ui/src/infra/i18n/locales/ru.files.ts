import { registerTranslations } from "../i18n";
import { messages } from "../../host/host.generated";
import { RU } from "./ru";

const fs = messages.fs;

/** File operations in Russian (templates generated from Rust). */
registerTranslations(RU, {
  [fs.notAFile]: "{path} не является файлом.",
  [fs.alreadyExists]: "{path} уже существует.",
  [fs.intoItself]: "Папку нельзя скопировать в саму себя.",
  [fs.failed]: "{path}: {error}",
});

registerTranslations(RU, {
  "Delete selected items": "Удалить выбранные элементы",
  "Delete these items and their sidecar files from disk? {names}": "Удалить с диска эти элементы и их сопутствующие файлы? {names}",
  "Enter a valid portable file name.": "Введите допустимое имя файла без разделителей пути и зарезервированных имён.",
  '"{name}" already exists here.': '«{name}» уже существует в этой папке.',
  "New Folder": "Новая папка", "New File": "Новый файл", "Rename": "Переименовать", "Delete": "Удалить", "Name": "Имя",
});
