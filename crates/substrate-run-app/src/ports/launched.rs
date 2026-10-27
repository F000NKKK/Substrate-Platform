//! A process just started, with its output streams.

use std::io::Read;

use super::ChildProcess;

/// A process just started: the process and its output streams.
#[expect(
    missing_debug_implementations,
    reason = "holds trait objects of a launcher"
)]
pub struct Launched {
    /// The process.
    pub child: Box<dyn ChildProcess>,
    /// Its standard output, if it was piped.
    pub stdout: Option<Box<dyn Read + Send>>,
    /// Its standard error, if it was piped.
    pub stderr: Option<Box<dyn Read + Send>>,
}
