//! Taking a program off the desktop's list.

use crate::products::host_products::HostProducts;
use crate::products::product_facts::ProductFacts;

use super::desktop_refresh::refresh;
use super::launcher_plan::entry_path;

impl HostProducts {
    /// Removes the product's launcher entry and tells the desktop. An entry
    /// that cannot be removed is left: the program is going anyway and a
    /// stale entry is harmless.
    pub(in crate::products) fn unlist_on_linux(&self, facts: &ProductFacts<'_>) {
        let entries = &self.launchers().desktop_entries;
        let entry = entry_path(entries, facts.id);
        if std::fs::remove_file(&entry).is_ok() {
            refresh(self.launchers().caches, entries);
        }
    }
}
