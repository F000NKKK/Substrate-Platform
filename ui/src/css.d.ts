// Stylesheets are imported for their side effect and the host's bundler
// loads them; TypeScript 7 checks side-effect imports, so it's told they
// exist.
declare module "*.css";
