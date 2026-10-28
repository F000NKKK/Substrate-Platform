//! Reading the values of one entry, naming it when one is wrong.

use std::str::FromStr;

use substrate_data::{Doc, Path};

use crate::problems::{CatalogFileError, Location, Problem, Reason};

/// Where in the file an entry is being read: the text, its parsed document
/// (for positions) and the path of the entry, so a wrong value is reported at
/// its line, column and path.
#[derive(Debug, Clone)]
pub(super) struct Scope<'a> {
    text: &'a str,
    doc: &'a Doc,
    path: Path,
}

impl<'a> Scope<'a> {
    /// The scope of the whole document.
    pub(super) fn root(text: &'a str, doc: &'a Doc) -> Self {
        Self {
            text,
            doc,
            path: Path::root(),
        }
    }

    /// The scope of the value under `key`.
    pub(super) fn key(&self, key: &str) -> Self {
        Self {
            path: self.path.key(key),
            ..self.clone()
        }
    }

    /// The scope of the item `index`.
    pub(super) fn index(&self, index: usize) -> Self {
        Self {
            path: self.path.index(index),
            ..self.clone()
        }
    }

    /// The line and column of the value at `path`.
    pub(super) fn locate(&self, path: &Path) -> Location {
        self.doc.locate(self.text, path).into()
    }

    /// The error for `field` of this entry.
    pub(super) fn fail(&self, field: &str, reason: impl Into<Reason>) -> CatalogFileError {
        self.fail_at(self.path.key(field), reason)
    }

    fn fail_at(&self, path: Path, reason: impl Into<Reason>) -> CatalogFileError {
        let location = self.locate(&path);
        CatalogFileError::Invalid {
            problems: vec![Problem::broken(location, path, reason.into())],
        }
    }

    /// Reads the text `value` of `field` as a `T`.
    pub(super) fn parse<T>(&self, field: &str, value: &str) -> Result<T, CatalogFileError>
    where
        T: FromStr,
        T::Err: Into<Reason>,
    {
        value
            .parse()
            .map_err(|error: T::Err| self.fail(field, error))
    }

    /// Reads the text of `field`, when there is one.
    pub(super) fn parse_opt<T>(
        &self,
        field: &str,
        value: Option<&String>,
    ) -> Result<Option<T>, CatalogFileError>
    where
        T: FromStr,
        T::Err: Into<Reason>,
    {
        value.map(|text| self.parse(field, text)).transpose()
    }

    /// Reads every text of the list `field`.
    pub(super) fn parse_all<T>(
        &self,
        field: &str,
        values: &[String],
    ) -> Result<Vec<T>, CatalogFileError>
    where
        T: FromStr,
        T::Err: Into<Reason>,
    {
        values
            .iter()
            .enumerate()
            .map(|(index, text)| {
                text.parse()
                    .map_err(|error: T::Err| self.fail_at(self.path.key(field).index(index), error))
            })
            .collect()
    }
}
