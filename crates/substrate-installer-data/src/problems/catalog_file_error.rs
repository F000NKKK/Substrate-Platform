//! Why a catalog file is refused.

use std::fmt;

use substrate_data::Path;
use substrate_installer_domain::channel::CatalogError;

use super::{Location, Problem};

/// Why a catalog file is refused.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum CatalogFileError {
    /// The text is not Substrate Data of the catalog's shape, or a value
    /// breaks a rule of the vocabulary. Never empty; in file order.
    #[error("{}", Summary(.problems))]
    Invalid {
        /// Every problem found.
        problems: Vec<Problem>,
    },
    /// The entries are individually fine but do not make a catalog (an id
    /// twice, a reference to nothing, a requirement cycle).
    #[error("{}{source}", place(*.location, .path.as_ref()))]
    Rejected {
        /// The line of the entry the domain names, when the file has it.
        location: Option<Location>,
        /// The entry's `id`, as a path.
        path: Option<Path>,
        /// What the domain said.
        #[source]
        source: CatalogError,
    },
}

impl CatalogFileError {
    /// The problems of an `Invalid` error; none for a rejected catalog.
    #[must_use]
    pub fn problems(&self) -> &[Problem] {
        match self {
            Self::Invalid { problems } => problems,
            Self::Rejected { .. } => &[],
        }
    }
}

/// The first problem, and how many more there are.
struct Summary<'p>(&'p [Problem]);

impl fmt::Display for Summary<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Some((first, rest)) = self.0.split_first() else {
            return f.write_str("the catalog is not valid");
        };
        write!(f, "{first}")?;
        match rest.len() {
            0 => Ok(()),
            more => write!(f, " (and {more} more)"),
        }
    }
}

fn place(location: Option<Location>, path: Option<&Path>) -> String {
    match (location, path) {
        (Some(at), Some(path)) => format!("{at}: {path}: "),
        (Some(at), None) => format!("{at}: "),
        _ => String::new(),
    }
}
