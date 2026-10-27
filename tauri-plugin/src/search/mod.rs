//! Find and Replace in Files: a search streams its files to the UI over a
//! channel; a newer one (or `find_cancel`) stops it. What the product keeps
//! out of the walk is the [`Layout`](substrate_platform::search::Layout) it
//! gives the plugin. Feature `search`.

pub(crate) mod commands;
mod find_batch;
mod find_request;
mod searches;

use find_batch::FindBatch;
use find_request::FindRequest;
pub(crate) use searches::Searches;
