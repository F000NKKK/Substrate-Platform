//! crates.io's version list into releases.

use std::cmp::Ordering;

use serde::Deserialize;
use substrate_installer_domain::releases::Release;
use substrate_installer_domain::versions::Version;

/// `GET /crates/{name}`.
#[derive(Debug, Deserialize)]
struct CrateDto {
    versions: Vec<CrateVersionDto>,
}

#[derive(Debug, Deserialize)]
struct CrateVersionDto {
    num: String,
    #[serde(default)]
    yanked: bool,
}

/// The crate's versions that were not yanked, newest first.
pub(super) fn parse(json: &str) -> Result<Vec<Release>, serde_json::Error> {
    let dto: CrateDto = serde_json::from_str(json)?;
    let mut releases: Vec<Release> = dto
        .versions
        .into_iter()
        .filter(|version| !version.yanked)
        .filter_map(|version| Version::new(&version.num).ok())
        .map(Release::new)
        .collect();
    releases.sort_by(|a, b| newest_first(&a.version, &b.version));
    Ok(releases)
}

fn newest_first(a: &Version, b: &Version) -> Ordering {
    b.compare(a)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn yanked_versions_are_left_out_and_the_newest_comes_first() {
        let releases = parse(include_str!("fixtures/crates_io.json")).unwrap();
        let versions: Vec<_> = releases.iter().map(|r| r.version.as_str()).collect();
        assert_eq!(versions, ["0.10.0", "0.9.0", "0.3.0"]);
        assert!(releases.iter().all(|r| r.assets.is_empty()));
    }

    #[test]
    fn an_answer_without_versions_is_an_error() {
        assert!(parse("{}").is_err());
    }
}
