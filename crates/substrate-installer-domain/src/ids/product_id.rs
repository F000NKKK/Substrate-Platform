//! The [`ProductId`].

use super::define_id;

define_id! {
    /// A product: what the installer is for, installed as a whole (also the installer's own entry).
    ProductId, Product
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ids::IdError;

    #[test]
    fn it_is_checked_where_it_is_made() {
        let id = ProductId::new("a.b-1").unwrap();
        assert_eq!(id.as_str(), "a.b-1");
        assert_eq!(id.to_string(), "a.b-1");
        assert_eq!("a.b-1".parse::<ProductId>().unwrap(), id);
        assert!(matches!(
            ProductId::new("A"),
            Err(IdError::InvalidCharacter { .. })
        ));
    }
}
