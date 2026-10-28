//! The [`Description`] value.

use super::define_text;

define_text! {
    /// A longer explanation shown beside a name; may be empty.
    Description, Description, free
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_keeps_its_text() {
        let value = Description::new("x").unwrap();
        assert_eq!(value.as_str(), "x");
        assert_eq!(value.to_string(), "x");
        assert_eq!("x".parse::<Description>().unwrap(), value);
    }
}
