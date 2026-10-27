//! What a search and a replace answer.

mod file_result_dto;
mod find_summary_dto;
mod line_hit_dto;
mod replace_summary_dto;
mod replaced_document_dto;
mod replaced_file_dto;

pub(crate) use file_result_dto::FileResultDto;
pub(crate) use find_summary_dto::FindSummaryDto;
pub(crate) use line_hit_dto::LineHitDto;
pub(crate) use replace_summary_dto::ReplaceSummaryDto;
pub(crate) use replaced_document_dto::ReplacedDocumentDto;
pub(crate) use replaced_file_dto::ReplacedFileDto;
