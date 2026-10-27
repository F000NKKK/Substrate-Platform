//! The events Substrate's host sends its UI, named once here; the UI gets
//! them generated with the message templates (`substrate-messages`).

/// The General output: what the IDE itself reports (errors, notices).
pub const IDE_OUTPUT: &str = "substrate://ide-output";

/// A Source Control run's lines, as [`crate::message::Message`]s.
pub const GIT_OUTPUT: &str = "substrate://git-output";

/// Paths changed outside the IDE, in the watched folders: a list of paths.
pub const FILES_CHANGED: &str = "substrate://files-changed";

/// A terminal's output: `{ id, data }`.
pub const TERMINAL_OUTPUT: &str = "substrate://terminal-output";

/// A terminal's shell exited: `{ id }`.
pub const TERMINAL_EXIT: &str = "substrate://terminal-exit";

/// What a language server said unasked: `{ root, language, method, params }`.
pub const LSP_NOTIFICATION: &str = "substrate://lsp-notification";

/// A language server's own log (its stderr), a line: `{ root, language, line }`.
pub const LSP_OUTPUT: &str = "substrate://lsp-output";

/// Every event, for the UI's generated copy: (name in code, event).
pub const ALL: &[(&str, &str)] = &[
    ("ideOutput", IDE_OUTPUT),
    ("gitOutput", GIT_OUTPUT),
    ("filesChanged", FILES_CHANGED),
    ("terminalOutput", TERMINAL_OUTPUT),
    ("terminalExit", TERMINAL_EXIT),
    ("lspNotification", LSP_NOTIFICATION),
    ("lspOutput", LSP_OUTPUT),
];

/// `events` — (name in code, event) — as a TypeScript constant `name`; `doc`
/// is its comment. A product's own events go to its UI the same way.
pub fn typescript(name: &str, doc: &str, events: &[(&str, &str)]) -> String {
    let mut out = format!("/** {doc} */\nexport const {name} = {{\n");
    for (key, event) in events {
        out.push_str(&format!("  {key}: {event:?},\n"));
    }
    out.push_str("} as const;\n");
    out
}
