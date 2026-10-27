//! A file types mask, VS-style: which files a search covers.

use globset::{Glob, GlobSet, GlobSetBuilder};

use crate::message::Message;
use crate::search::messages;

/// VS's "all files" masks: they'd skip extensionless files as globs.
const ALL_FILES: [&str; 2] = ["*.*", "*"];
/// Between a mask's globs.
const SEPARATORS: [char; 2] = [';', ','];
/// In front of a glob that excludes.
const EXCLUDE: char = '!';

/// A file types mask (`*.rs;*.toml`, `!*.lock`): globs without a `/` match
/// the file name, ones with a `/` the path relative to the search root.
#[derive(Clone, Debug)]
pub struct FileMask {
    include: Option<GlobSet>,
    exclude: Option<GlobSet>,
}

impl FileMask {
    /// Every file.
    #[must_use]
    pub fn all() -> Self {
        Self {
            include: None,
            exclude: None,
        }
    }

    /// Parses `mask`; an empty one covers every file.
    ///
    /// # Errors
    ///
    /// [`messages::BAD_MASK`] for a glob that doesn't parse.
    pub fn parse(mask: &str) -> Result<Self, Message> {
        let mut include = Globs::default();
        let mut exclude = Globs::default();
        let mut all = false;
        for part in mask
            .split(SEPARATORS)
            .map(str::trim)
            .filter(|p| !p.is_empty())
        {
            match part.strip_prefix(EXCLUDE) {
                Some(glob) => exclude.add(glob.trim(), part)?,
                None if ALL_FILES.contains(&part) => all = true,
                None => include.add(part, part)?,
            }
        }
        if all {
            include = Globs::default();
        }
        Ok(Self {
            include: include.build(mask)?,
            exclude: exclude.build(mask)?,
        })
    }

    /// Whether the file at `relative_path` (from the search root) is covered.
    #[must_use]
    pub fn matches(&self, relative_path: &str) -> bool {
        let name = relative_path
            .rsplit(['/', '\\'])
            .next()
            .unwrap_or(relative_path);
        let slashed = relative_path.replace('\\', "/");
        let hit = |set: &GlobSet| set.is_match(name) || set.is_match(&slashed);
        self.include.as_ref().is_none_or(hit) && !self.exclude.as_ref().is_some_and(hit)
    }
}

/// One side of a mask (includes or excludes), being parsed.
struct Globs {
    builder: GlobSetBuilder,
    any: bool,
}

impl Default for Globs {
    fn default() -> Self {
        Self {
            builder: GlobSetBuilder::new(),
            any: false,
        }
    }
}

impl Globs {
    /// Adds `glob`, written `part` in the mask.
    fn add(&mut self, glob: &str, part: &str) -> Result<(), Message> {
        let glob = Glob::new(&glob.replace('\\', "/")).map_err(|err| bad_mask(part, err))?;
        self.builder.add(glob);
        self.any = true;
        Ok(())
    }

    /// The set — `None` when nothing was added.
    fn build(self, mask: &str) -> Result<Option<GlobSet>, Message> {
        if !self.any {
            return Ok(None);
        }
        self.builder
            .build()
            .map(Some)
            .map_err(|err| bad_mask(mask, err))
    }
}

fn bad_mask(mask: &str, error: globset::Error) -> Message {
    Message::new(messages::BAD_MASK)
        .arg("mask", mask)
        .arg("error", error)
}
