//! One matching line, on the wire.

use serde::Serialize;
use substrate_search_domain::results::LineHit;

/// One matching line, as the Find Results window shows it.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct LineHitDto {
    line: u32,
    column: u32,
    text: String,
    ranges: Vec<(u32, u32)>,
}

impl From<LineHit> for LineHitDto {
    fn from(hit: LineHit) -> Self {
        Self {
            line: hit.line,
            column: hit.column,
            text: hit.text,
            ranges: hit.ranges,
        }
    }
}
