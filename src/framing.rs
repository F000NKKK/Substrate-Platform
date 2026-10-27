//! The framing the Language Server and Debug Adapter protocols share: a
//! `Content-Length` header, a blank line, then that many bytes of JSON.

use std::io::{self, BufRead, Write};

use serde_json::Value;

const CONTENT_LENGTH: &str = "content-length";

/// Writes `message` as one frame.
pub(crate) fn write(out: &mut impl Write, message: &Value) -> io::Result<()> {
    let body = serde_json::to_vec(message)?;
    write!(out, "Content-Length: {}\r\n\r\n", body.len())?;
    out.write_all(&body)?;
    out.flush()
}

/// The next frame's JSON — `None` at the end of the stream. Headers other
/// than `Content-Length` (`Content-Type`) are skipped.
pub(crate) fn read(input: &mut impl BufRead) -> io::Result<Option<Value>> {
    let mut length = None;
    let mut line = String::new();
    loop {
        line.clear();
        if input.read_line(&mut line)? == 0 {
            return if length.is_none() {
                Ok(None)
            } else {
                Err(io::ErrorKind::UnexpectedEof.into())
            };
        }
        let header = line.trim_end_matches(['\r', '\n']);
        if header.is_empty() {
            break;
        }
        let Some((name, value)) = header.split_once(':') else {
            return Err(invalid(format!("not a header: {header:?}")));
        };
        if name.trim().eq_ignore_ascii_case(CONTENT_LENGTH) {
            let value = value.trim();
            length = Some(
                value
                    .parse::<usize>()
                    .map_err(|_| invalid(format!("not a length: {value:?}")))?,
            );
        }
    }
    let length = length.ok_or_else(|| invalid("a frame without Content-Length".into()))?;
    let mut body = vec![0; length];
    input.read_exact(&mut body)?;
    serde_json::from_slice(&body).map(Some).map_err(Into::into)
}

fn invalid(detail: String) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, detail)
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use serde_json::json;

    use super::*;

    #[test]
    fn frames_round_trip_and_skip_other_headers() {
        let mut out = Vec::new();
        write(&mut out, &json!({ "a": "ü" })).unwrap();
        assert!(out.starts_with(b"Content-Length: 10\r\n\r\n"));

        let text = b"Content-Type: application/vscode-jsonrpc; charset=utf-8\r\ncontent-length: 2\r\n\r\n{}";
        let mut input = Cursor::new(text.to_vec());
        assert_eq!(read(&mut input).unwrap(), Some(json!({})));
        assert_eq!(read(&mut input).unwrap(), None);

        let mut cut = Cursor::new(b"Content-Length: 9\r\n\r\n{}".to_vec());
        assert!(read(&mut cut).is_err());
    }
}
