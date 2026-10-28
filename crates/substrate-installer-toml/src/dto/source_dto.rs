//! `source = { kind = "…", … }`.

use std::collections::BTreeMap;
use std::ops::Range;

use serde::Deserialize;
use substrate_installer_domain::sources::{CrateName, RepoSlug, RustTarget, Source, Toolchain};
use toml::Spanned;

use super::assets::platform_assets;
use super::file_kind::FileKind;
use super::system_dto::SystemDto;
use crate::error::{CatalogFileError, Reason};
use crate::scope::Scope;

/// A source table. Its fields differ by `kind`, so all but `kind` are
/// optional here and [`into_source`](Self::into_source) asks for the ones its
/// kind needs. Keys no kind has are ignored (a kind this crate does not know
/// is refused by name first).
#[derive(Debug, Deserialize)]
pub(super) struct SourceDto {
    kind: Spanned<String>,
    repo: Option<Spanned<String>>,
    assets: Option<Spanned<BTreeMap<String, String>>>,
    system: Option<Spanned<SystemDto>>,
    name: Option<Spanned<String>>,
    toolchain: Option<Spanned<String>>,
    targets: Option<Spanned<Vec<Spanned<String>>>>,
}

impl SourceDto {
    /// The domain's source.
    pub(super) fn into_source(self, scope: &Scope<'_>) -> Result<Source, CatalogFileError> {
        let kind = scope.parse::<FileKind>("source.kind", &self.kind)?;
        self.reject_unexpected(kind, scope)?;
        match kind {
            FileKind::Github => self.github(scope),
            FileKind::Git => Ok(Source::GitTag {
                repo: self.repo(kind, scope)?,
            }),
            FileKind::Crate => {
                let name = self.need(kind, scope, "source.name", self.name.as_ref())?;
                Ok(Source::CratesIo {
                    name: scope.parse::<CrateName>("source.name", name)?,
                })
            }
            FileKind::Rustup => Ok(Source::Rustup {
                toolchain: self.toolchain(kind, scope)?,
            }),
            FileKind::RustTargets => self.rust_targets(scope),
        }
    }

    fn github(self, scope: &Scope<'_>) -> Result<Source, CatalogFileError> {
        let kind = FileKind::Github;
        let assets = self.need(kind, scope, "source.assets", self.assets.as_ref())?;
        Ok(Source::GithubRelease {
            repo: self.repo(kind, scope)?,
            assets: platform_assets(scope, "source.assets", assets)?,
            system: self
                .system
                .map(|system| system.into_inner().into_package(scope))
                .transpose()?,
        })
    }

    fn rust_targets(self, scope: &Scope<'_>) -> Result<Source, CatalogFileError> {
        let kind = FileKind::RustTargets;
        let targets = self.need(kind, scope, "source.targets", self.targets.as_ref())?;
        Ok(Source::RustTargets {
            toolchain: self.toolchain(kind, scope)?,
            targets: scope.parse_all::<RustTarget>("source.targets", targets.get_ref())?,
        })
    }

    fn repo(&self, kind: FileKind, scope: &Scope<'_>) -> Result<RepoSlug, CatalogFileError> {
        let repo = self.need(kind, scope, "source.repo", self.repo.as_ref())?;
        scope.parse("source.repo", repo)
    }

    fn toolchain(&self, kind: FileKind, scope: &Scope<'_>) -> Result<Toolchain, CatalogFileError> {
        let toolchain = self.need(kind, scope, "source.toolchain", self.toolchain.as_ref())?;
        scope.parse("source.toolchain", toolchain)
    }

    /// `value`, or the error that a source of `kind` lacks `field`.
    fn need<'v, T>(
        &self,
        kind: FileKind,
        scope: &Scope<'_>,
        field: &'static str,
        value: Option<&'v Spanned<T>>,
    ) -> Result<&'v Spanned<T>, CatalogFileError> {
        value.ok_or_else(|| {
            scope.fail(
                &self.kind.span(),
                field,
                Reason::MissingField { kind: kind.name() },
            )
        })
    }

    /// Refuses a field `kind` has no use for.
    fn reject_unexpected(&self, kind: FileKind, scope: &Scope<'_>) -> Result<(), CatalogFileError> {
        let stray = self
            .present()
            .into_iter()
            .find(|(field, _)| !kind.fields().contains(field));
        match stray {
            Some((field, span)) => {
                Err(scope.fail(&span, field, Reason::UnexpectedField { kind: kind.name() }))
            }
            None => Ok(()),
        }
    }

    /// The optional fields the file has, with where.
    fn present(&self) -> Vec<(&'static str, Range<usize>)> {
        [
            ("source.repo", self.repo.as_ref().map(Spanned::span)),
            ("source.assets", self.assets.as_ref().map(Spanned::span)),
            ("source.system", self.system.as_ref().map(Spanned::span)),
            ("source.name", self.name.as_ref().map(Spanned::span)),
            (
                "source.toolchain",
                self.toolchain.as_ref().map(Spanned::span),
            ),
            ("source.targets", self.targets.as_ref().map(Spanned::span)),
        ]
        .into_iter()
        .filter_map(|(field, span)| Some((field, span?)))
        .collect()
    }
}
