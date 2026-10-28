//! One thing that happened.

use super::Outcome;
use crate::ids::ItemId;
use crate::versions::Version;

/// One thing that happened, for a progress display (as Visual Studio
/// Installer shows it).
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum Event {
    /// The plan starts.
    Started {
        /// How many packages it changes.
        packages: usize,
        /// How many bytes to fetch, as far as known.
        bytes: u64,
    },
    /// Bytes of a package's download arrived.
    Downloading {
        /// The package.
        item: ItemId,
        /// Bytes so far.
        done: u64,
        /// Bytes in all, where known.
        total: Option<u64>,
    },
    /// A package of the plan is being installed.
    Installing {
        /// The package.
        item: ItemId,
        /// Its number in the plan, from 1.
        index: usize,
        /// How many packages the plan has.
        of: usize,
    },
    /// A line a tool printed while installing.
    Output {
        /// The package being installed.
        item: ItemId,
        /// The line.
        line: String,
    },
    /// A package is installed.
    Installed {
        /// The package.
        item: ItemId,
        /// Its version.
        version: Version,
    },
    /// A package is removed.
    Removed {
        /// The package.
        item: ItemId,
    },
    /// The plan ended: every package, or stopped at an error or a cancel.
    Finished {
        /// How it ended.
        outcome: Outcome,
    },
}
