//! What a search and a replace return: matching files and their lines, and the totals.

mod file_result;
mod find_summary;
mod line_hit;
mod replace_summary;
mod replaced_document;
mod replaced_file;

pub use file_result::FileResult;
pub use find_summary::FindSummary;
pub use line_hit::{LineHit, line_hits};
pub use replace_summary::ReplaceSummary;
pub use replaced_document::ReplacedDocument;
pub use replaced_file::ReplacedFile;
