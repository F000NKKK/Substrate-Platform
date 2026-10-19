//! What Substrate's host and UI share by name — its plugin and the events
//! it sends — named once here; the UI gets them generated with the message
//! templates (`substrate-messages`).

/// The host plugin's name: its commands are `plugin:<this>|<command>`.
pub const PLUGIN: &str = "substrate";

/// A Source Control run's lines, as [`crate::message::Message`]s.
pub const GIT_OUTPUT: &str = "substrate://git-output";

/// Every event, for the UI's generated copy: (name in code, event).
pub const ALL: &[(&str, &str)] = &[("gitOutput", GIT_OUTPUT)];
