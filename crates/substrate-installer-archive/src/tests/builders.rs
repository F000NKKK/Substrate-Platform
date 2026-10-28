//! Archives made in memory.

use std::io::{Cursor, Write};

use flate2::Compression;
use flate2::write::GzEncoder;
use lzma_rust2::{XzOptions, XzWriter};
use zip::write::SimpleFileOptions;

/// What goes into a test archive.
pub(super) enum Entry<'a> {
    File(&'a str, &'a [u8], u32),
    Folder(&'a str, u32),
    Symlink(&'a str, &'a str),
    HardLink(&'a str, &'a str),
}

/// A tar with `entries`. Names are written into the header as they are, so a
/// test can put `..` or `/` in them.
pub(super) fn tar(entries: &[Entry<'_>]) -> Vec<u8> {
    let mut builder = tar::Builder::new(Vec::new());
    for entry in entries {
        let mut header = tar::Header::new_gnu();
        let (name, kind, mode, data, link) = match entry {
            Entry::File(name, data, mode) => (*name, tar::EntryType::Regular, *mode, *data, None),
            Entry::Folder(name, mode) => (*name, tar::EntryType::Directory, *mode, &[][..], None),
            Entry::Symlink(name, to) => (*name, tar::EntryType::Symlink, 0o777, &[][..], Some(*to)),
            Entry::HardLink(name, to) => (*name, tar::EntryType::Link, 0o644, &[][..], Some(*to)),
        };
        header.as_mut_bytes()[..name.len()].copy_from_slice(name.as_bytes());
        header.set_entry_type(kind);
        header.set_mode(mode);
        header.set_size(data.len() as u64);
        if let Some(link) = link {
            header.as_mut_bytes()[157..157 + link.len()].copy_from_slice(link.as_bytes());
        }
        header.set_cksum();
        builder.append(&header, data).unwrap();
    }
    builder.into_inner().unwrap()
}

pub(super) fn tar_gz(entries: &[Entry<'_>]) -> Vec<u8> {
    let mut gz = GzEncoder::new(Vec::new(), Compression::fast());
    gz.write_all(&tar(entries)).unwrap();
    gz.finish().unwrap()
}

pub(super) fn tar_xz(entries: &[Entry<'_>]) -> Vec<u8> {
    let mut xz = XzWriter::new(Vec::new(), XzOptions::with_preset(1)).unwrap();
    xz.write_all(&tar(entries)).unwrap();
    xz.finish().unwrap()
}

/// A zip with `entries`.
pub(super) fn zip(entries: &[Entry<'_>]) -> Vec<u8> {
    let mut writer = zip::ZipWriter::new(Cursor::new(Vec::new()));
    for entry in entries {
        match entry {
            Entry::File(name, data, mode) => {
                writer
                    .start_file(*name, SimpleFileOptions::default().unix_permissions(*mode))
                    .unwrap();
                writer.write_all(data).unwrap();
            }
            Entry::Folder(name, mode) => {
                let options = SimpleFileOptions::default().unix_permissions(*mode);
                writer.add_directory(*name, options).unwrap();
            }
            Entry::Symlink(name, to) => {
                writer
                    .add_symlink(*name, *to, SimpleFileOptions::default())
                    .unwrap();
            }
            // A zip has no hard links.
            Entry::HardLink(..) => {}
        }
    }
    writer.finish().unwrap().into_inner()
}

/// An `ar` member: its header, its data, the padding.
fn member(name: &str, data: &[u8]) -> Vec<u8> {
    let mut bytes = format!(
        "{name:<16}{:<12}{:<6}{:<6}{:<8}{:<10}`\n",
        0,
        0,
        0,
        644,
        data.len()
    )
    .into_bytes();
    bytes.extend_from_slice(data);
    if data.len() % 2 == 1 {
        bytes.push(b'\n');
    }
    bytes
}

/// A Debian package whose files are `data` in the member `data_name`.
pub(super) fn deb(data_name: &str, data: &[u8]) -> Vec<u8> {
    let mut bytes = b"!<arch>\n".to_vec();
    bytes.extend(member("debian-binary", b"2.0\n"));
    bytes.extend(member("control.tar.gz", b"odd"));
    bytes.extend(member(data_name, data));
    bytes
}
