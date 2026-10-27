//! Reading one frame.

use std::io::{self, BufRead};

use serde_json::Value;

/// The header that gives the body's length.
const CONTENT_LENGTH: &str = "content-length";

/// The next frame's JSON — `None` at the end of the stream. Headers other
/// than `Content-Length` (`Content-Type`) are skipped.
///
/// # Errors
///
/// The stream can't be read, ends inside a frame, or the frame isn't one.
pub fn read_frame(input: &mut impl BufRead) -> io::Result<Option<Value>> {
    let Some(length) = read_headers(input)? else {
        return Ok(None);
    };
    let mut body = vec![0; length];
    input.read_exact(&mut body)?;
    serde_json::from_slice(&body).map(Some).map_err(Into::into)
}

/// The body's length, once the blank line ends the headers — `None` for a
/// stream that ends before any header.
fn read_headers(input: &mut impl BufRead) -> io::Result<Option<usize>> {
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
    length
        .ok_or_else(|| invalid("a frame without Content-Length".into()))
        .map(Some)
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
    fn frames_skip_other_headers_and_end_cleanly() {
        let text = b"Content-Type: application/vscode-jsonrpc; charset=utf-8\r\ncontent-length: 2\r\n\r\n{}";
        let mut input = Cursor::new(text.to_vec());
        assert_eq!(read_frame(&mut input).unwrap(), Some(json!({})));
        assert_eq!(read_frame(&mut input).unwrap(), None);
    }

    #[test]
    fn a_cut_frame_is_an_error() {
        let mut cut = Cursor::new(b"Content-Length: 9\r\n\r\n{}".to_vec());
        assert!(read_frame(&mut cut).is_err());
        let mut headless = Cursor::new(b"Content-Length: 2\r\n".to_vec());
        assert!(read_frame(&mut headless).is_err());
    }

    #[test]
    fn a_bad_header_or_length_is_refused() {
        for text in [
            &b"nonsense\r\n\r\n"[..],
            b"Content-Length: x\r\n\r\n",
            b"Other: 1\r\n\r\n",
        ] {
            assert!(read_frame(&mut Cursor::new(text.to_vec())).is_err());
        }
    }
}
