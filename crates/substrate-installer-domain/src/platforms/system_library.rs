//! A shared library the system must have.

use crate::texts::define_text;

define_text! {
    /// A shared library the system must have, by its soname (`libgtk-3.so.0`).
    SystemLibrary, SystemLibrary, token
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_is_a_word() {
        assert!(SystemLibrary::new("libgtk-3.so.0").is_ok());
        assert!(SystemLibrary::new("lib gtk").is_err());
    }
}
