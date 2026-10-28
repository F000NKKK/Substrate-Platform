//! The change that brings one thing to its newest version.

use super::Change;
use crate::ids::ItemId;
use crate::inventory::Found;
use crate::versions::Version;

/// What makes `id` installed at `newest`: an install when it is not found,
/// an update when the installer put it there and `newest` is newer, nothing
/// otherwise (what the system had before stays as it is).
#[must_use]
pub fn derive_item(id: &ItemId, newest: &Version, installed: &[Found]) -> Option<Change> {
    let present = installed.iter().find(|found| &found.id == id);
    match present {
        None => Some(Change::Install {
            id: id.clone(),
            version: newest.clone(),
        }),
        Some(found) if found.is_managed() && newest.compare(&found.version).is_gt() => {
            Some(Change::Update {
                id: id.clone(),
                from: found.version.clone(),
                to: newest.clone(),
            })
        }
        Some(_) => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::inventory::Origin;
    use crate::testing::{found, item_component, version};

    #[test]
    fn it_installs_updates_or_leaves_alone() {
        let id = item_component("a");
        let newest = version("2");
        assert!(matches!(
            derive_item(&id, &newest, &[]),
            Some(Change::Install { .. })
        ));
        let managed = [found("a", "1", Origin::Managed)];
        assert!(matches!(
            derive_item(&id, &newest, &managed),
            Some(Change::Update { .. })
        ));
        let system = [found("a", "1", Origin::System)];
        assert_eq!(derive_item(&id, &newest, &system), None);
        let current = [found("a", "2", Origin::Managed)];
        assert_eq!(derive_item(&id, &newest, &current), None);
    }
}
