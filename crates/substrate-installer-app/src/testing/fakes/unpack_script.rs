//! How the fake unpacker behaves.

/// How the fake unpacker behaves.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum UnpackScript {
    #[default]
    Works,
    Fails,
}
