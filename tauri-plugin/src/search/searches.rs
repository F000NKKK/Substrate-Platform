//! Which search may keep running.

use std::sync::Arc;
use std::sync::atomic::AtomicU64;

/// The id of the search allowed to keep running — a new search replaces it,
/// `find_cancel` clears it.
#[derive(Default)]
pub struct Searches {
    pub(super) current: Arc<AtomicU64>,
}
