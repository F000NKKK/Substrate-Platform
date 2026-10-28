//! One download in progress.

use std::fs::{self, File};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;

use sha2::{Digest, Sha256 as Hasher};
use substrate_installer_app::ports::downloads::{DownloadRequest, Progress, TransferControl};
use substrate_installer_domain::releases::{HttpUrl, Sha256};
use ureq::Body;
use ureq::http::Response;

use super::stop::Stop;
use crate::http_client::check_status;
use crate::{HttpClient, HttpError};

/// HTTP's answer to a `Range` request it honours, and to one past the end
/// (everything is there).
const PARTIAL_CONTENT: u16 = 206;
const RANGE_NOT_SATISFIABLE: u16 = 416;
/// Bytes read between progress reports and cancel checks.
const CHUNK: usize = 64 * 1024;

/// How a stretch of the download ended.
enum Stretch {
    /// Every byte has arrived.
    Whole,
    /// The operation was paused: the connection is closed.
    Paused,
}

/// A download: the partial file, the hash of what it holds, and where the
/// bytes go as they arrive.
pub(super) struct Transfer<'a, 't> {
    client: &'a HttpClient,
    url: &'a HttpUrl,
    expected: Option<&'a Sha256>,
    control: &'a TransferControl<'a>,
    tee: Option<&'t mut dyn Write>,
    file: File,
    path: &'a Path,
    hasher: Hasher,
    done: u64,
    total: Option<u64>,
}

impl<'a, 't> Transfer<'a, 't> {
    /// Opens `part` (making its folders), and feeds what an earlier try left
    /// in it to the hash and to `tee`, so both see the file from its start.
    pub(super) fn open(
        client: &'a HttpClient,
        request: &DownloadRequest<'a>,
        control: &'a TransferControl<'a>,
        tee: Option<&'t mut dyn Write>,
        part: &'a Path,
    ) -> Result<Self, Stop> {
        if let Some(folder) = request.destination.parent() {
            fs::create_dir_all(folder).map_err(|source| io_error(folder, source))?;
        }
        let file = File::options()
            .create(true)
            .truncate(false)
            .read(true)
            .write(true)
            .open(part)
            .map_err(|source| io_error(part, source))?;
        let mut transfer = Self {
            client,
            url: request.url,
            expected: request.sha256,
            control,
            tee,
            file,
            path: part,
            hasher: Hasher::new(),
            done: 0,
            total: None,
        };
        transfer.replay()?;
        Ok(transfer)
    }

    /// Goes on until the file is whole and checked.
    pub(super) fn run(mut self) -> Result<(), Stop> {
        loop {
            // One connection per stretch between pauses: a pause closes it,
            // so the download really stops, and going on asks for the rest.
            self.control.cancel.check()?;
            if let Stretch::Whole = self.stretch()? {
                return self.verify();
            }
        }
    }

    fn replay(&mut self) -> Result<(), Stop> {
        let mut buffer = vec![0; CHUNK];
        loop {
            let count = self
                .file
                .read(&mut buffer)
                .map_err(|source| io_error(self.path, source))?;
            if count == 0 {
                break;
            }
            self.hasher.update(&buffer[..count]);
            self.hand_on(&buffer[..count])?;
            self.done += count as u64;
        }
        if self.done > 0 {
            self.report();
        }
        Ok(())
    }

    fn stretch(&mut self) -> Result<Stretch, Stop> {
        let url = self.url.as_str();
        let response = self.client.file(url, self.done)?;
        let status = response.status().as_u16();
        if self.done > 0 && status == RANGE_NOT_SATISFIABLE {
            return Ok(Stretch::Whole);
        }
        check_status(url, &response)?;
        if self.done > 0 && status != PARTIAL_CONTENT {
            self.start_over()?;
        }
        self.total = self
            .total
            .or(length(&response).map(|rest| rest + self.done));
        self.pump(response.into_body().as_reader())
    }

    /// The server sent everything again. That is fine for a file, not for a
    /// stream whose consumer already has the first part.
    fn start_over(&mut self) -> Result<(), Stop> {
        if self.tee.is_some() {
            return Err(HttpError::ResumeUnsupported {
                url: self.url.to_string(),
            }
            .into());
        }
        self.file
            .set_len(0)
            .and_then(|()| self.file.seek(SeekFrom::Start(0)).map(drop))
            .map_err(|source| io_error(self.path, source))?;
        self.hasher = Hasher::new();
        self.done = 0;
        Ok(())
    }

    fn pump(&mut self, mut body: impl Read) -> Result<Stretch, Stop> {
        let mut buffer = vec![0; CHUNK];
        while !self.control.cancel.is_paused() {
            self.control.cancel.check()?;
            let count = body.read(&mut buffer).map_err(|source| HttpError::Body {
                url: self.url.to_string(),
                source,
            })?;
            if count == 0 {
                return Ok(Stretch::Whole);
            }
            self.take(&buffer[..count])?;
        }
        Ok(Stretch::Paused)
    }

    /// A chunk that arrived: into the file, the hash and the consumer.
    fn take(&mut self, chunk: &[u8]) -> Result<(), Stop> {
        self.file
            .write_all(chunk)
            .map_err(|source| io_error(self.path, source))?;
        self.hasher.update(chunk);
        self.hand_on(chunk)?;
        self.done += chunk.len() as u64;
        self.report();
        Ok(())
    }

    fn hand_on(&mut self, chunk: &[u8]) -> Result<(), Stop> {
        match self.tee.as_mut() {
            Some(tee) => tee.write_all(chunk).map_err(Stop::Consumer),
            None => Ok(()),
        }
    }

    fn report(&self) {
        (self.control.progress)(Progress {
            done: self.done,
            total: self.total,
        });
    }

    /// The whole file against the digest the release published.
    fn verify(self) -> Result<(), Stop> {
        let Some(expected) = self.expected else {
            return Ok(());
        };
        let actual = Sha256::from_bytes(self.hasher.finalize().into());
        if &actual == expected {
            Ok(())
        } else {
            Err(Stop::Checksum {
                expected: *expected,
                actual,
            })
        }
    }
}

fn length(response: &Response<Body>) -> Option<u64> {
    response
        .headers()
        .get("content-length")?
        .to_str()
        .ok()?
        .parse()
        .ok()
}

fn io_error(path: &Path, source: std::io::Error) -> Stop {
    Stop::Failed(HttpError::Io {
        path: path.to_path_buf(),
        source,
    })
}
