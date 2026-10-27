//! A real language server end to end: `cargo test -p substrate-lsp-app -- --ignored`
//! with rust-analyzer installed (`rustup component add rust-analyzer`).

use std::time::Duration;

use serde_json::{Value, json};
use substrate_lsp_app::Client;
use substrate_lsp_domain::{ServerDefinition, uri};
use substrate_run_app::LogSink;
use substrate_run_process::SystemLauncher;

#[test]
#[ignore = "needs rust-analyzer"]
fn rust_analyzer_answers_a_hover() {
    let dir = std::env::temp_dir().join(format!("substrate-lsp-{}", std::process::id()));
    std::fs::create_dir_all(dir.join("src")).unwrap();
    std::fs::write(
        dir.join("Cargo.toml"),
        "[package]\nname = \"probe\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    )
    .unwrap();
    let main = dir.join("src/main.rs");
    let text = "/// The answer.\nfn answer() -> u32 { 42 }\nfn main() { answer(); }\n";
    std::fs::write(&main, text).unwrap();

    let server = ServerDefinition::new("rust-analyzer");
    let client = Client::start(&SystemLauncher, &server, &dir, LogSink::new(), |_| {}).unwrap();
    let capabilities = client.initialize(&dir, None).unwrap();
    assert!(capabilities.get("hoverProvider").is_some());
    client.did_open(&main, "rust", 1, text).unwrap();
    // The first hover can come back empty while the workspace loads.
    let mut hover = Value::Null;
    for _ in 0..60 {
        hover = client
            .request(
                "textDocument/hover",
                json!({
                    "textDocument": { "uri": uri::from_path(&main) },
                    "position": { "line": 2, "character": 13 },
                }),
            )
            .unwrap();
        if !hover.is_null() {
            break;
        }
        std::thread::sleep(Duration::from_millis(500));
    }
    client.shutdown();
    std::fs::remove_dir_all(&dir).ok();
    assert!(
        hover["contents"]["value"]
            .as_str()
            .is_some_and(|v| v.contains("The answer."))
    );
}
