//! What Substrate's host and UI share by name — its plugin and the events
//! it sends — named once here; the UI gets them generated with the message
//! templates (`substrate-messages`).

/// The host plugin's name: its commands are `plugin:<this>|<command>`.
pub const PLUGIN: &str = "substrate";

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

/// Every event, for the UI's generated copy: (name in code, event).
pub const ALL: &[(&str, &str)] = &[
    ("ideOutput", IDE_OUTPUT),
    ("gitOutput", GIT_OUTPUT),
    ("filesChanged", FILES_CHANGED),
    ("terminalOutput", TERMINAL_OUTPUT),
    ("terminalExit", TERMINAL_EXIT),
];
