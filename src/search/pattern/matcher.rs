//! A search query compiled: every match in a text, and the text with them replaced.

use regex::{Regex, RegexBuilder};

use super::ReplacementMode;
use crate::search::SearchError;
use crate::search::SearchQuery;

/// The compiled pattern, with VS's "Match whole word" applied as a check on
/// each match's neighbours (the regex engine has no lookaround) rather than
/// by wrapping the pattern in `\b`, which would fail for patterns that start
/// or end with punctuation.
#[derive(Clone, Debug)]
pub struct Matcher {
    pattern: Regex,
    whole_word: bool,
}

impl Matcher {
    /// Compiles `query`'s pattern.
    ///
    /// # Errors
    ///
    /// [`SearchError::EmptyPattern`] for an empty pattern, [`SearchError::BadPattern`]
    /// for a regular expression that doesn't compile.
    pub fn new(query: &SearchQuery) -> Result<Self, SearchError> {
        if query.pattern.is_empty() {
            return Err(SearchError::EmptyPattern);
        }
        let source = if query.regex {
            query.pattern.clone()
        } else {
            regex::escape(&query.pattern)
        };
        let pattern = RegexBuilder::new(&source)
            .case_insensitive(!query.match_case)
            .multi_line(true)
            .crlf(true)
            .build()
            .map_err(SearchError::BadPattern)?;
        Ok(Self {
            pattern,
            whole_word: query.whole_word,
        })
    }

    /// Byte ranges of every non-empty match in `text`, left to right,
    /// non-overlapping, at most `limit` of them.
    #[must_use]
    pub fn find_all(&self, text: &str, limit: usize) -> Vec<(usize, usize)> {
        let mut out = Vec::new();
        let mut position = 0;
        while position <= text.len() && out.len() < limit {
            let Some(found) = self.pattern.find_at(text, position) else {
                break;
            };
            let (start, end) = (found.start(), found.end());
            if start == end || (self.whole_word && !is_whole_word(text, start, end)) {
                // Retry one character further on: a shorter or later match
                // may still qualify.
                position = start + text[start..].chars().next().map_or(1, char::len_utf8);
                continue;
            }
            out.push((start, end));
            position = end;
        }
        out
    }

    /// `text` with every match replaced by `replacement`, read as `mode` says.
    /// Returns the new text and the number of replacements.
    #[must_use]
    pub fn replace_all(
        &self,
        text: &str,
        replacement: &str,
        mode: ReplacementMode,
    ) -> (String, usize) {
        let matches = self.find_all(text, usize::MAX);
        if matches.is_empty() {
            return (text.to_string(), 0);
        }
        let mut out = String::with_capacity(text.len());
        let mut last = 0;
        for &(start, end) in &matches {
            out.push_str(&text[last..start]);
            match mode {
                ReplacementMode::Literal => out.push_str(replacement),
                ReplacementMode::Expand => self.expand(text, start, replacement, &mut out),
            }
            last = end;
        }
        out.push_str(&text[last..]);
        (out, matches.len())
    }

    /// `replacement` with the groups of the match at `start` expanded, onto `out`.
    fn expand(&self, text: &str, start: usize, replacement: &str, out: &mut String) {
        if let Some(captures) = self.pattern.captures_at(text, start) {
            captures.expand(replacement, out);
        }
    }
}

fn is_word_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

/// Whether the match `start..end` of `text` stands as a whole word.
fn is_whole_word(text: &str, start: usize, end: usize) -> bool {
    let before = text[..start].chars().next_back();
    let after = text[end..].chars().next();
    let first = text[start..end].chars().next();
    let last = text[start..end].chars().next_back();
    // Only a word character at the match's own edge needs a non-word
    // neighbour — `->` as a whole word still matches inside `a->b`.
    let left_ok = !(first.is_some_and(is_word_char) && before.is_some_and(is_word_char));
    let right_ok = !(last.is_some_and(is_word_char) && after.is_some_and(is_word_char));
    left_ok && right_ok
}
