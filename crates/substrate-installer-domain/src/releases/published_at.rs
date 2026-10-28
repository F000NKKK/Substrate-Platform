//! When a release came out.

use crate::texts::define_text;

define_text! {
    /// When a release came out, as the source says it (RFC 3339, which orders
    /// as text).
    PublishedAt, PublishedAt, token
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_orders_as_text() {
        let early = PublishedAt::new("2026-01-02T03:04:05Z").unwrap();
        let late = PublishedAt::new("2026-10-02T03:04:05Z").unwrap();
        assert!(early < late);
    }
}
