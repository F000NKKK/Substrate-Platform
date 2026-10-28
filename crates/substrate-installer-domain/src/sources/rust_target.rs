//! The [`RustTarget`].

use crate::texts::define_text;

define_text! {
    /// A Rust compilation target triple (`x86_64-pc-windows-gnu`).
    RustTarget, RustTarget, token
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_keeps_its_text() {
        let value = RustTarget::new("x86_64-unknown-linux-gnu").unwrap();
        assert_eq!(value.as_str(), "x86_64-unknown-linux-gnu");
        assert!(RustTarget::new("").is_err());
    }
}
