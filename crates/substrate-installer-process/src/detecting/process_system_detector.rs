//! The system detector over real programs.

use substrate_installer_app::ports::detection::SystemDetector;
use substrate_installer_domain::channel::Catalog;
use substrate_installer_domain::inventory::Found;

use super::probe::Probe;
use super::process_probe::ProcessProbe;
use super::scan::Scan;
use crate::cargo_bin::CargoBin;

/// Finds the components of a catalog the machine already has, as
/// [`Origin::System`](substrate_installer_domain::inventory::Origin::System),
/// by asking `rustup` and `cargo`: a component whose source is a toolchain
/// (`rustup toolchain list`), targets (`rustup target list --installed`) or
/// a crate (`cargo install --list`). Sources installed from files (GitHub
/// releases, tags) are not found; the installer's own records cover them. A
/// tool that is missing or fails finds nothing.
#[derive(Debug, Clone)]
pub struct ProcessSystemDetector {
    bin: CargoBin,
}

impl ProcessSystemDetector {
    /// A detector that looks for rustup and cargo where `bin` says (its folder, then its search path).
    #[must_use]
    pub fn new(bin: CargoBin) -> Self {
        Self { bin }
    }
}

impl SystemDetector for ProcessSystemDetector {
    fn detect(&self, catalog: &Catalog) -> Vec<Found> {
        detect_with(&ProcessProbe, &self.bin, catalog)
    }
}

/// [`SystemDetector::detect`] over any [`Probe`].
pub(super) fn detect_with(probe: &dyn Probe, bin: &CargoBin, catalog: &Catalog) -> Vec<Found> {
    let scan = Scan::new(probe, bin, catalog);
    catalog
        .components()
        .iter()
        .filter_map(|component| scan.find(component))
        .collect()
}
