//! The tables a catalog file has.

use std::fmt;

/// A table of the catalog file, as the file spells it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Table {
    /// The top level: `revision` and `manifest`.
    Catalog,
    /// `[installer]`.
    Installer,
    /// `[[group]]`.
    Group,
    /// `[[product]]`.
    Product,
    /// `[[workload]]`.
    Workload,
    /// `[[component]]`.
    Component,
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Catalog => "the top level",
            Self::Installer => "[installer]",
            Self::Group => "[[group]]",
            Self::Product => "[[product]]",
            Self::Workload => "[[workload]]",
            Self::Component => "[[component]]",
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_table_is_shown_as_the_file_spells_it() {
        assert_eq!(Table::Installer.to_string(), "[installer]");
        assert_eq!(Table::Group.to_string(), "[[group]]");
        assert_eq!(Table::Product.to_string(), "[[product]]");
        assert_eq!(Table::Workload.to_string(), "[[workload]]");
        assert_eq!(Table::Component.to_string(), "[[component]]");
        assert_eq!(Table::Catalog.to_string(), "the top level");
    }
}
