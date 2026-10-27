//! A long-running process that talks over its standard streams.

use std::io::{Read, Write};

use super::ChildProcess;

/// A process just started to be talked to: the process and its three
/// standard streams, all piped (a language server, a debug adapter).
#[expect(
    missing_debug_implementations,
    reason = "holds trait objects of a launcher"
)]
pub struct Service {
    /// The process.
    pub child: Box<dyn ChildProcess>,
    /// Its standard input.
    pub stdin: Box<dyn Write + Send>,
    /// Its standard output.
    pub stdout: Box<dyn Read + Send>,
    /// Its standard error.
    pub stderr: Box<dyn Read + Send>,
}
