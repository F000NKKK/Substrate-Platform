//! Reading the fields of one entry, naming it when one is wrong.

use std::ops::Range;
use std::str::FromStr;

use toml::Spanned;

use crate::error::{CatalogFileError, Problem, Reason, Table};
use crate::lines::Lines;

/// The entry being read: which table, which `id`, and the lines of the file,
/// so a wrong field is reported where it is.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Scope<'a> {
    lines: &'a Lines,
    table: Table,
    entry: Option<&'a str>,
}

impl<'a> Scope<'a> {
    /// The scope of an entry of `table` whose id is `entry`.
    pub(crate) fn new(lines: &'a Lines, table: Table, entry: Option<&'a str>) -> Self {
        Self {
            lines,
            table,
            entry,
        }
    }

    /// The error for `field` at `span`.
    pub(crate) fn fail(
        &self,
        span: &Range<usize>,
        field: &'static str,
        reason: impl Into<Reason>,
    ) -> CatalogFileError {
        CatalogFileError::Invalid(Box::new(Problem {
            location: self.lines.locate(span.start),
            table: self.table,
            entry: self.entry.map(str::to_owned),
            field,
            reason: reason.into(),
        }))
    }

    /// Reads the text `value` as a `T`.
    pub(crate) fn parse<T>(
        &self,
        field: &'static str,
        value: &Spanned<String>,
    ) -> Result<T, CatalogFileError>
    where
        T: FromStr,
        T::Err: Into<Reason>,
    {
        value
            .get_ref()
            .parse()
            .map_err(|error: T::Err| self.fail(&value.span(), field, error))
    }

    /// Reads the text `value`, when there is one.
    pub(crate) fn parse_opt<T>(
        &self,
        field: &'static str,
        value: Option<&Spanned<String>>,
    ) -> Result<Option<T>, CatalogFileError>
    where
        T: FromStr,
        T::Err: Into<Reason>,
    {
        value.map(|text| self.parse(field, text)).transpose()
    }

    /// Reads every text of `values`.
    pub(crate) fn parse_all<T>(
        &self,
        field: &'static str,
        values: &[Spanned<String>],
    ) -> Result<Vec<T>, CatalogFileError>
    where
        T: FromStr,
        T::Err: Into<Reason>,
    {
        values.iter().map(|text| self.parse(field, text)).collect()
    }
}
