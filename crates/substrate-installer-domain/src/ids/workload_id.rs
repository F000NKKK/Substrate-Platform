//! The [`WorkloadId`].

use super::define_id;

define_id! {
    /// A workload: a named set of components for one kind of work.
    WorkloadId, Workload
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ids::IdError;

    #[test]
    fn it_is_checked_where_it_is_made() {
        let id = WorkloadId::new("a.b-1").unwrap();
        assert_eq!(id.as_str(), "a.b-1");
        assert_eq!(id.to_string(), "a.b-1");
        assert_eq!("a.b-1".parse::<WorkloadId>().unwrap(), id);
        assert!(matches!(
            WorkloadId::new("A"),
            Err(IdError::InvalidCharacter { .. })
        ));
    }
}
