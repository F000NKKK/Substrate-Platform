//! The ports that decide where things go and put them there.

mod files_error;
mod install_files;
mod install_layout;
mod product_error;
mod product_install;
mod product_installer;
mod product_removal;
mod removal_mode;

pub use files_error::FilesError;
pub use install_files::InstallFiles;
pub use install_layout::InstallLayout;
pub use product_error::ProductError;
pub use product_install::ProductInstall;
pub use product_installer::ProductInstaller;
pub use product_removal::ProductRemoval;
pub use removal_mode::RemovalMode;
