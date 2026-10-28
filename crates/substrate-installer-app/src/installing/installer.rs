//! The use case that carries out a plan.

use substrate_installer_domain::channel::Catalog;
use substrate_installer_domain::events::{Event, Outcome};
use substrate_installer_domain::planning::Plan;
use substrate_installer_domain::platforms::Host;

use super::run::Run;
use super::{Adapters, ApplyError, ApplyOptions};
use crate::control::CancelToken;

/// Carries out plans against a catalog on a host, through its adapters.
#[derive(Debug, Clone, Copy)]
pub struct Installer<'a> {
    pub(super) adapters: Adapters<'a>,
    pub(super) catalog: &'a Catalog,
    pub(super) host: &'a Host,
}

impl<'a> Installer<'a> {
    /// An installer for `catalog` on `host`, working through `adapters`.
    #[must_use]
    pub fn new(adapters: Adapters<'a>, catalog: &'a Catalog, host: &'a Host) -> Self {
        Self {
            adapters,
            catalog,
            host,
        }
    }

    /// Carries out `plan`, change by change: downloads, installs or removes,
    /// records the result in the store after each change, and tells the
    /// events (`Started` first, `Finished` last). A component that replaces
    /// an older version removes it. Stops at the first error; what was done
    /// stays done and recorded.
    ///
    /// # Errors
    ///
    /// [`ApplyError`]: [`Cancelled`](ApplyError::Cancelled) when `cancel`
    /// was triggered, else what stopped the plan.
    pub fn apply(
        &self,
        plan: &Plan,
        options: ApplyOptions,
        cancel: &CancelToken,
    ) -> Result<(), ApplyError> {
        let bytes = plan.download_size(self.catalog, self.host);
        self.adapters.sink.emit(Event::Started {
            packages: plan.changes.len(),
            bytes,
        });
        let result = Run::new(self, plan, options, cancel).execute();
        self.adapters.sink.emit(Event::Finished {
            outcome: outcome_of(&result),
        });
        result
    }
}

fn outcome_of(result: &Result<(), ApplyError>) -> Outcome {
    match result {
        Ok(()) => Outcome::Completed,
        Err(error) if error.is_cancelled() => Outcome::Cancelled,
        Err(error) => Outcome::Failed {
            reason: error.to_string(),
        },
    }
}
