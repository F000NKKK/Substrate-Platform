//! One problem in a text file.

use std::fmt;

use substrate_data::{Code, Diagnostic, Path};

use super::{Location, Reason};

/// One problem in a Substrate Data text file: where it is, which value, its
/// code and what is wrong.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Problem {
    /// The line and column, when known.
    pub location: Option<Location>,
    /// The value, as a path from the file's root (`components[2].source.repo`).
    pub path: Path,
    /// The Substrate Data code: `SD0001` for text that does not parse,
    /// `SD0006` for a missing field, `SD0000` for a value that breaks a
    /// rule of the installer's vocabulary.
    pub code: Code,
    /// What is wrong, in words.
    pub message: String,
    /// The rule of the vocabulary it breaks, for an `SD0000` problem.
    pub reason: Option<Reason>,
}

impl Problem {
    /// What Substrate Data said.
    pub(crate) fn of(diagnostic: Diagnostic) -> Self {
        Self {
            location: diagnostic
                .line
                .zip(diagnostic.col)
                .map(|(line, col)| Location::from(substrate_data::LineCol { line, col })),
            path: diagnostic.path,
            code: diagnostic
                .code
                .unwrap_or(Code::Data(substrate_data::Problem::Invalid)),
            message: diagnostic.message,
            reason: None,
        }
    }

    /// A value at `path` that breaks `reason`.
    pub(crate) fn broken(location: Location, path: Path, reason: Reason) -> Self {
        Self {
            location: Some(location),
            path,
            code: Code::Data(substrate_data::Problem::Invalid),
            message: reason.to_string(),
            reason: Some(reason),
        }
    }
}

impl fmt::Display for Problem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(at) = self.location {
            write!(f, "{at}: ")?;
        }
        if !self.path.is_root() {
            write!(f, "{}: ", self.path)?;
        }
        write!(f, "{} ({})", self.message, self.code)
    }
}
