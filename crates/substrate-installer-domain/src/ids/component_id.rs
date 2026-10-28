//! The [`ComponentId`].

use super::define_id;

define_id! {
    /// A component: one installable part (a toolchain, a JDK, a tool).
    ComponentId, Component
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ids::IdError;

    #[test]
    fn it_is_checked_where_it_is_made() {
        let id = ComponentId::new("a.b-1").unwrap();
        assert_eq!(id.as_str(), "a.b-1");
        assert_eq!(id.to_string(), "a.b-1");
        assert_eq!("a.b-1".parse::<ComponentId>().unwrap(), id);
        assert!(matches!(
            ComponentId::new("A"),
            Err(IdError::InvalidCharacter { .. })
        ));
    }
}
