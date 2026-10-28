//! Who put an installed thing there.

/// Who installed something found on the machine.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Origin {
    /// The installer did: it may update and remove it.
    Managed,
    /// It was there before, or someone else put it there (rustup's toolchain,
    /// a JDK of the system): it counts as installed and is left alone.
    System,
}
