//! Writes what Substrate's host and UI share for the UI — message
//! templates, the plugin's name, event names:
//! `substrate-messages <file.ts>` (default: the UI's generated copy).

/// Where the UI imports the templates from, from the crate.
const DEFAULT_OUT: &str = "ui/src/infra/host/host.generated.ts";

fn main() -> Result<(), String> {
    let path = std::env::args().nth(1).unwrap_or_else(|| DEFAULT_OUT.to_string());
    let text = substrate_platform::message::typescript();
    if std::fs::read_to_string(&path).ok().as_deref() == Some(text.as_str()) {
        return Ok(());
    }
    std::fs::write(&path, text).map_err(|e| format!("{path}: {e}"))
}
