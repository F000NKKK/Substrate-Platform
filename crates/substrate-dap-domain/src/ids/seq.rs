//! A message's sequence number.

/// The sequence number a message carries. Each side numbers its own
/// messages from 1; a response names the request it answers by that
/// request's number.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Seq(pub i64);

impl Seq {
    /// The first number.
    pub const FIRST: Self = Self(1);
}

impl std::fmt::Display for Seq {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numbering_starts_at_one() {
        assert_eq!(Seq::FIRST.to_string(), "1");
    }
}
