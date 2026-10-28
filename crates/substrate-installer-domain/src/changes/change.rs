//! One thing a plan does.

use crate::ids::ItemId;
use crate::versions::Version;

/// One thing a plan does.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Change {
    /// Install `id` at `version`.
    Install {
        /// What to install.
        id: ItemId,
        /// The version to install.
        version: Version,
    },
    /// Replace the installed `from` of `id` with `to`.
    Update {
        /// What to update.
        id: ItemId,
        /// The version installed now.
        from: Version,
        /// The version to install.
        to: Version,
    },
    /// Remove `id` at `version`.
    Remove {
        /// What to remove.
        id: ItemId,
        /// The installed version to remove.
        version: Version,
    },
}

impl Change {
    /// What the change is about.
    #[must_use]
    pub fn id(&self) -> &ItemId {
        match self {
            Self::Install { id, .. } | Self::Update { id, .. } | Self::Remove { id, .. } => id,
        }
    }

    /// The version that will be installed; none for a removal.
    #[must_use]
    pub fn target(&self) -> Option<&Version> {
        match self {
            Self::Install { version, .. } | Self::Update { to: version, .. } => Some(version),
            Self::Remove { .. } => None,
        }
    }

    /// Whether it puts something on the machine (an install or an update).
    #[must_use]
    pub fn installs(&self) -> bool {
        self.target().is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::{item_component, version};

    #[test]
    fn it_tells_its_id_and_target() {
        let id = item_component("a");
        let install = Change::Install {
            id: id.clone(),
            version: version("2"),
        };
        let update = Change::Update {
            id: id.clone(),
            from: version("1"),
            to: version("3"),
        };
        let remove = Change::Remove {
            id: id.clone(),
            version: version("1"),
        };
        assert_eq!(install.target(), Some(&version("2")));
        assert_eq!(update.target(), Some(&version("3")));
        assert_eq!(remove.target(), None);
        assert!(install.installs() && update.installs() && !remove.installs());
        assert!([&install, &update, &remove].iter().all(|c| c.id() == &id));
    }
}
