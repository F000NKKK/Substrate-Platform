//! Settings a format keeps beyond the model.

/// A named group of settings a format keeps beyond the model — a `.sln`'s
/// `GlobalSection` or `ProjectSection`, a `.slnx`'s `<Properties>` — kept in
/// order, so a file is written back as it was read.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Section {
    /// Its name.
    pub name: String,
    /// When it applies, where the format says (a `.sln`'s `preSolution`,
    /// `postProject`); empty where it doesn't.
    pub scope: String,
    /// Its key–value entries, in order.
    pub entries: Vec<(String, String)>,
}
