//! Answers the inherited-cursor query before the frontend has a registered session.

use std::io::{self, Write};

use substrate_terminal_app::ports::OnOutput;

const QUERY: &[u8] = b"\x1b[6n";
const ORIGIN: &[u8] = b"\x1b[1;1R";

pub(crate) struct StartupOutput<W> {
    writer: W,
    pending: Vec<u8>,
    waiting: bool,
}

impl<W: Write> StartupOutput<W> {
    pub(crate) fn new(writer: W) -> Self {
        Self {
            writer,
            pending: Vec::new(),
            waiting: true,
        }
    }

    pub(crate) fn accept(&mut self, bytes: Vec<u8>, output: &mut OnOutput) -> io::Result<()> {
        if !self.waiting {
            output(bytes);
            return Ok(());
        }
        self.pending.extend(bytes);
        if let Some(position) = self
            .pending
            .windows(QUERY.len())
            .position(|part| part == QUERY)
        {
            self.writer.write_all(ORIGIN)?;
            self.writer.flush()?;
            self.waiting = false;
            let rest = self.pending.split_off(position + QUERY.len());
            self.pending.truncate(position);
            self.finish(output);
            if !rest.is_empty() {
                output(rest);
            }
        } else {
            let retained = (1..QUERY.len())
                .rev()
                .find(|&size| self.pending.ends_with(&QUERY[..size]))
                .unwrap_or(0);
            let ready = self.pending.len() - retained;
            if ready > 0 {
                let tail = self.pending.split_off(ready);
                output(std::mem::replace(&mut self.pending, tail));
            }
        }
        Ok(())
    }

    pub(crate) fn finish(&mut self, output: &mut OnOutput) {
        if !self.pending.is_empty() {
            output(std::mem::take(&mut self.pending));
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use super::*;

    #[test]
    fn a_query_split_at_any_read_boundary_is_answered_once_without_polluting_output() {
        let bytes = b"hello\x1b[6nworld";
        for split in 0..=bytes.len() {
            let heard = Arc::new(Mutex::new(Vec::new()));
            let sink = heard.clone();
            let mut output: OnOutput = Box::new(move |chunk| sink.lock().unwrap().extend(chunk));
            let mut startup = StartupOutput::new(Vec::new());
            startup
                .accept(bytes[..split].to_vec(), &mut output)
                .unwrap();
            startup
                .accept(bytes[split..].to_vec(), &mut output)
                .unwrap();
            startup.accept(QUERY.to_vec(), &mut output).unwrap();
            startup.finish(&mut output);
            assert_eq!(startup.writer, ORIGIN);
            assert_eq!(*heard.lock().unwrap(), b"helloworld\x1b[6n");
        }
    }

    #[test]
    fn ordinary_bytes_and_an_unfinished_escape_sequence_survive_eof() {
        let heard = Arc::new(Mutex::new(Vec::new()));
        let sink = heard.clone();
        let mut output: OnOutput = Box::new(move |chunk| sink.lock().unwrap().extend(chunk));
        let mut startup = StartupOutput::new(Vec::new());
        startup.accept(b"text\x1b[".to_vec(), &mut output).unwrap();
        startup.finish(&mut output);
        assert_eq!(*heard.lock().unwrap(), b"text\x1b[");
        assert_eq!(startup.writer, Vec::<u8>::new());
    }
}
