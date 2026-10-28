//! An unpacker that unpacks nothing and tells what it was asked.

use std::io::Read;
use std::path::Path;

use super::UnpackScript;
use crate::control::CancelToken;
use crate::ports::unpacking::{UnpackError, UnpackRequest, Unpacker};
use crate::testing::{Call, Journal};

/// Unpacks nothing.
#[derive(Debug)]
pub(crate) struct FakeUnpacker {
    pub(crate) journal: Journal,
    pub(crate) script: UnpackScript,
}

impl FakeUnpacker {
    pub(crate) fn new(journal: Journal) -> Self {
        Self {
            journal,
            script: UnpackScript::Works,
        }
    }

    fn fail_by_script(&self) -> Result<(), UnpackError> {
        match self.script {
            UnpackScript::Works => Ok(()),
            UnpackScript::Fails => Err(UnpackError::Failed("damaged archive".into())),
        }
    }
}

impl Unpacker for FakeUnpacker {
    fn unpack_file(
        &self,
        archive: &Path,
        request: &UnpackRequest<'_>,
        cancel: &CancelToken,
    ) -> Result<(), UnpackError> {
        cancel.check()?;
        self.journal.record(Call::UnpackFile {
            archive: archive.to_path_buf(),
            destination: request.destination.to_path_buf(),
        });
        self.fail_by_script()
    }

    fn unpack_stream(
        &self,
        reader: &mut dyn Read,
        request: &UnpackRequest<'_>,
        cancel: &CancelToken,
    ) -> Result<(), UnpackError> {
        cancel.check()?;
        self.fail_by_script()?;
        let mut bytes = Vec::new();
        reader
            .read_to_end(&mut bytes)
            .map_err(|error| UnpackError::Failed(Box::new(error)))?;
        self.journal.record(Call::UnpackStream {
            destination: request.destination.to_path_buf(),
            bytes: bytes.len(),
        });
        Ok(())
    }
}
