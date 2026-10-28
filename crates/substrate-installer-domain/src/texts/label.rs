//! The [`Label`] value.

use super::define_text;

define_text! {
    /// A display name: a product's, a component's, a group's.
    Label, Label, non_empty
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_keeps_its_text() {
        let value = Label::new("x").unwrap();
        assert_eq!(value.as_str(), "x");
        assert_eq!(value.to_string(), "x");
        assert_eq!("x".parse::<Label>().unwrap(), value);
    }
}
