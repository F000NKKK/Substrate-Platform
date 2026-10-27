//! Whether generated code is searched.

/// Whether generated code (the layout's `generated`) is searched too.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum GeneratedCode {
    /// Skipped with the rest of the build output.
    #[default]
    Skip,
    /// Searched, never replaced in.
    Include,
}
