//! The opener of real ptys.

use portable_pty::{PtyPair, native_pty_system};
use substrate_terminal_app::ports::{OnExit, OnOutput, PtyOpener, Session};
use substrate_terminal_domain::{SessionOptions, TerminalError, TerminalSize};

use crate::command::command;
use crate::pty_session::{PtySession, pty_size};
use crate::read_in_background::read_in_background;
use crate::writer::Writer;

/// Starts shells on the OS's own ptys through `portable-pty`.
#[derive(Debug, Clone, Copy, Default)]
pub struct PortablePty;

impl PtyOpener for PortablePty {
    fn open(
        &self,
        options: SessionOptions,
        size: TerminalSize,
        on_output: OnOutput,
        on_exit: OnExit,
    ) -> Result<Box<dyn Session>, TerminalError> {
        let pair = native_pty_system()
            .openpty(pty_size(size))
            .map_err(|e| TerminalError::Open(e.into()))?;
        // ConPTY can emit startup output before spawning returns. Drain it now,
        // so a verbose PowerShell profile cannot fill the output pipe.
        let reader = pair
            .master
            .try_clone_reader()
            .map_err(|e| TerminalError::Stream(e.into()))?;
        let writer = Writer::new(
            pair.master
                .take_writer()
                .map_err(|e| TerminalError::Stream(e.into()))?,
        );
        read_in_background(
            reader,
            on_output,
            on_exit,
            #[cfg(windows)]
            writer.clone(),
        );
        let child = pair
            .slave
            .spawn_command(command(options))
            .map_err(|e| TerminalError::Spawn(e.into()))?;
        // The slave end is only needed to spawn the child; drop our copy so
        // the master gets EOF when the child exits instead of hanging open.
        let PtyPair { master, slave } = pair;
        drop(slave);
        Ok(Box::new(PtySession {
            master,
            writer,
            child,
        }))
    }
}
