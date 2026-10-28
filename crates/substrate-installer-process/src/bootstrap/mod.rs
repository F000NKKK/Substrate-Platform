//! Getting rustup onto a machine that has none.

mod error;
mod rustup_bootstrap;
mod rustup_init_fetcher;

pub use error::BootstrapError;
pub use rustup_bootstrap::RustupBootstrap;
pub use rustup_init_fetcher::RustupInitFetcher;
