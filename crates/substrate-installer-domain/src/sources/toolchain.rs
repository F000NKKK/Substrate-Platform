//! The [`Toolchain`].

use crate::texts::define_text;

define_text! {
    /// A Rust toolchain as rustup names it (`1.99`, `stable`).
    Toolchain, Toolchain, token
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_keeps_its_text() {
        let value = Toolchain::new("1.99").unwrap();
        assert_eq!(value.as_str(), "1.99");
        assert!(Toolchain::new("").is_err());
    }
}
