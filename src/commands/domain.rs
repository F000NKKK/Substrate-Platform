//! A group of commands.

/// A domain of commands: its key in the UI's copy, its plugin, its commands.
#[derive(Debug)]
pub struct Domain {
    /// Its key in the UI's copy.
    pub key: &'static str,
    /// The plugin its commands belong to.
    pub plugin: &'static str,
    /// Their names.
    pub commands: &'static [&'static str],
}
