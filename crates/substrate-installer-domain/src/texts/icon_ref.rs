//! The [`IconRef`] value.

use super::define_text;

define_text! {
    /// What draws an icon: an image link or an icon name the front end knows.
    IconRef, IconRef, free
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_keeps_its_text() {
        let value = IconRef::new("x").unwrap();
        assert_eq!(value.as_str(), "x");
        assert_eq!(value.to_string(), "x");
        assert_eq!("x".parse::<IconRef>().unwrap(), value);
    }
}
