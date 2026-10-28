//! Why a catalog file is refused.

use substrate_installer_domain::channel::CatalogError;

use super::{Location, Problem};

/// Why a catalog file is refused: it is not TOML of the catalog's shape, a
/// field is wrong, or the entries do not make a catalog.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum CatalogFileError {
    /// The text is not TOML, or not TOML of the catalog's shape (a missing
    /// key, a misspelled one, a value of the wrong type).
    #[error("{}{}", at(*.location), .source.message())]
    Syntax {
        /// Where, when the parser says.
        location: Option<Location>,
        /// What the parser said.
        #[source]
        source: toml::de::Error,
    },
    /// A field breaks a rule of the catalog's vocabulary.
    #[error("{0}")]
    Invalid(Box<Problem>),
    /// The entries are individually fine but do not make a catalog (an id
    /// twice, a reference to nothing, a requirement cycle).
    #[error("{}{source}", at(*.location))]
    Rejected {
        /// The line of the entry the domain names, when the file has it.
        location: Option<Location>,
        /// What the domain said.
        #[source]
        source: CatalogError,
    },
}

fn at(location: Option<Location>) -> String {
    location.map_or_else(String::new, |at| format!("{at}: "))
}
