//! Tests of the pipe.

use std::io::{self, Read, Write};

use super::channel;

#[test]
fn what_is_written_is_read_in_order_until_the_writer_is_dropped() {
    let (mut writer, mut reader) = channel();
    writer.write_all(b"hello ").unwrap();
    writer.write_all(b"world").unwrap();
    writer.flush().unwrap();
    assert_eq!(writer.write(b"").unwrap(), 0);
    drop(writer);
    let mut text = String::new();
    reader.read_to_string(&mut text).unwrap();
    assert_eq!(text, "hello world");
}

#[test]
fn a_small_buffer_reads_a_chunk_in_pieces() {
    let (mut writer, mut reader) = channel();
    writer.write_all(b"abcde").unwrap();
    drop(writer);
    let mut two = [0; 2];
    let mut got = Vec::new();
    loop {
        let n = reader.read(&mut two).unwrap();
        if n == 0 {
            break;
        }
        got.extend_from_slice(&two[..n]);
    }
    assert_eq!(got, b"abcde");
}

#[test]
fn writing_to_a_dropped_reader_is_a_broken_pipe() {
    let (mut writer, reader) = channel();
    drop(reader);
    let error = writer.write(b"x").unwrap_err();
    assert_eq!(error.kind(), io::ErrorKind::BrokenPipe);
}
