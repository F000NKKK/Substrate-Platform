//! A place in a file.

use std::fmt;

use substrate_data::LineCol;

/// A place in a text file: line and column, both counted from 1 (the column
/// in characters).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Location {
    /// The line.
    pub line: usize,
    /// The column.
    pub column: usize,
}

impl From<LineCol> for Location {
    fn from(at: LineCol) -> Self {
        Self {
            line: at.line as usize,
            column: at.col as usize,
        }
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_reads_as_line_and_column() {
        let at = Location::from(LineCol { line: 4, col: 7 });
        assert_eq!(at, Location { line: 4, column: 7 });
        assert_eq!(at.to_string(), "4:7");
    }
}
