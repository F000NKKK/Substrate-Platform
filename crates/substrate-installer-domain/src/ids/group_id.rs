//! The [`GroupId`].

use super::define_id;

define_id! {
    /// A group of individual components, as the Modify window lists them.
    GroupId, Group
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ids::IdError;

    #[test]
    fn it_is_checked_where_it_is_made() {
        let id = GroupId::new("a.b-1").unwrap();
        assert_eq!(id.as_str(), "a.b-1");
        assert_eq!(id.to_string(), "a.b-1");
        assert_eq!("a.b-1".parse::<GroupId>().unwrap(), id);
        assert!(matches!(
            GroupId::new("A"),
            Err(IdError::InvalidCharacter { .. })
        ));
    }
}
