//! Writing one frame.

use std::io::{self, Write};

use serde_json::Value;

/// Writes `message` as one frame and flushes.
///
/// # Errors
///
/// The stream can't be written.
pub fn write_frame(out: &mut impl Write, message: &Value) -> io::Result<()> {
    let body = serde_json::to_vec(message)?;
    write!(out, "Content-Length: {}\r\n\r\n", body.len())?;
    out.write_all(&body)?;
    out.flush()
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn a_frame_names_its_length_in_bytes() {
        let mut out = Vec::new();
        write_frame(&mut out, &json!({ "a": "ü" })).unwrap();
        assert!(out.starts_with(b"Content-Length: 10\r\n\r\n"));
    }
}
