//! One of a project's items.

/// One of a project's items: a file and what it is to the build — an open
/// vocabulary, the format's own (`Compile`, `Content`, `None`, `EmbeddedResource`…).
#[derive(Debug, Clone, PartialEq)]
pub struct Item {
    /// What it is: an open vocabulary, the format's own.
    pub kind: String,
    /// The path or pattern (`src/**/*.cs`), relative to the project's folder.
    pub include: String,
    /// In the file's order.
    pub metadata: Vec<(String, String)>,
}
