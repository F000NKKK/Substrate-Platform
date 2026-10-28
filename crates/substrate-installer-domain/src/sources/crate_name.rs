//! The [`CrateName`].

use crate::texts::define_text;

define_text! {
    /// A crate on crates.io, installed with `cargo install`.
    CrateName, CrateName, token
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_keeps_its_text() {
        let value = CrateName::new("cargo-zigbuild").unwrap();
        assert_eq!(value.as_str(), "cargo-zigbuild");
        assert!(CrateName::new("").is_err());
    }
}
