import { registerTranslations } from "../i18n";
import { RU } from "./ru";

/** The data inspector in Russian. */
registerTranslations(RU, {
  "Data inspector": "Инспектор данных",
  value: "значение",
  none: "нет",
  default: "по умолчанию",
  missing: "отсутствует",
  "{count} item": { one: "{count} элемент", few: "{count} элемента", many: "{count} элементов", other: "{count} элемента" },
  "{count} entry": { one: "{count} запись", few: "{count} записи", many: "{count} записей", other: "{count} записи" },
  "Expected {expected}, found {found}.": "Ожидалось {expected}, найдено {found}.",
  "Unknown variant {name}.": "Неизвестный вариант {name}.",
  "No definition named {name}.": "Нет определения {name}.",
  "Required value is missing.": "Обязательное значение отсутствует.",
  "Enter a number.": "Введите число.",
  "Enter a whole number.": "Введите целое число.",
  "Enter a value from {min} to {max}.": "Введите значение от {min} до {max}.",
  "Value of {name}": "Значение {name}",
  "Variant of {name}": "Вариант {name}",
  "{count} problems inside": "Проблем внутри: {count}",
  "{at}: expected {expected}, found {found}.": "{at}: ожидалось {expected}, найдено {found}.",
  "{at}: unknown shape kind {found}.": "{at}: неизвестный вид формы {found}.",
  "{at}: unknown definition kind {found}.": "{at}: неизвестный вид определения {found}.",
  "{at}: no definition named {name}.": "{at}: нет определения {name}.",
});
