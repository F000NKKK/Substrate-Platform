import { registerFileTypes } from "../../../infra/fileTypes";
import {
  groovyLanguage,
  javaLanguage,
  jsonLanguage,
  markdownLanguage,
  propertiesLanguage,
  pythonLanguage,
  rustLanguage,
  shellLanguage,
  tomlLanguage,
  xmlLanguage,
  yamlLanguage,
} from "../code/languageEditors";

// The kinds of file the platform itself knows — names, highlighting, MIME
// types. Products register their own formats the same way.
registerFileTypes([
  { id: "text", name: "Plain Text", extensions: ["txt", "log"] },
  { id: "json", name: "JSON", extensions: ["json"], language: jsonLanguage },
  { id: "rust", name: "Rust", extensions: ["rs"], language: rustLanguage },
  { id: "xml", name: "XML", extensions: ["xml"], language: xmlLanguage },
  { id: "java", name: "Java", extensions: ["java"], language: javaLanguage },
  { id: "markdown", name: "Markdown", extensions: ["md", "markdown"], language: markdownLanguage },
  { id: "toml", name: "TOML", extensions: ["toml"], language: tomlLanguage },
  { id: "yaml", name: "YAML", extensions: ["yml", "yaml"], language: yamlLanguage },
  { id: "shell", name: "Shell Script", extensions: ["sh", "bash"], language: shellLanguage },
  { id: "properties", name: "Java Properties", extensions: ["properties"], language: propertiesLanguage },
  { id: "gradle", name: "Gradle", extensions: ["gradle"], language: groovyLanguage },
  { id: "groovy", name: "Groovy", extensions: ["groovy"], language: groovyLanguage },
  { id: "kotlin-script", name: "Kotlin Script", extensions: ["kts"] },
  { id: "python", name: "Python", extensions: ["py"], language: pythonLanguage },
  { id: "svg", name: "SVG Image", extensions: ["svg"], mime: "image/svg+xml", language: xmlLanguage },
  { id: "png", name: "PNG Image", extensions: ["png"], mime: "image/png" },
  { id: "jpeg", name: "JPEG Image", extensions: ["jpg", "jpeg"], mime: "image/jpeg" },
  { id: "gif", name: "GIF Image", extensions: ["gif"], mime: "image/gif" },
  { id: "webp", name: "WebP Image", extensions: ["webp"], mime: "image/webp" },
  { id: "bmp", name: "Bitmap Image", extensions: ["bmp"], mime: "image/bmp" },
  { id: "ico", name: "Icon", extensions: ["ico"], mime: "image/x-icon" },
]);
