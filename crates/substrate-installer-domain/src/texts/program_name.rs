//! The [`ProgramName`] value.

use super::define_text;

define_text! {
    /// The file stem of the program that starts a product (`yog-idle`).
    ProgramName, ProgramName, non_empty
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_keeps_its_text() {
        let value = ProgramName::new("x").unwrap();
        assert_eq!(value.as_str(), "x");
        assert_eq!(value.to_string(), "x");
        assert_eq!("x".parse::<ProgramName>().unwrap(), value);
    }
}
