//! Whether case matters.

/// Whether upper and lower case letters differ.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CaseSensitivity {
    /// `a` finds `A` too.
    #[default]
    Insensitive,
    /// `a` finds only `a`.
    Sensitive,
}
