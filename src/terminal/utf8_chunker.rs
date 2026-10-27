//! Bytes from a terminal, as text: a character split across reads stays whole.

/// Turns a stream of arbitrarily-split byte chunks into valid UTF-8 strings
/// — an incomplete multi-byte sequence at the end of one chunk is carried
/// over and completed by the next, instead of each half being decoded (and
/// garbled into U+FFFD) separately. Genuinely invalid bytes still become
/// U+FFFD, same as [`String::from_utf8_lossy`].
#[derive(Debug, Default)]
pub struct Utf8Chunker {
    pending: Vec<u8>,
}

impl Utf8Chunker {
    /// A chunker with nothing held back.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Feeds the next chunk; returns everything decodable so far (possibly
    /// empty, if `bytes` only extended an incomplete trailing character).
    pub fn push(&mut self, mut bytes: &[u8]) -> String {
        let mut out = String::with_capacity(self.pending.len() + bytes.len());
        // Complete the held-back character byte by byte (at most 3), then
        // decode the rest of the chunk in place — no copy of the whole chunk.
        while !self.pending.is_empty() {
            let Some((&next, rest)) = bytes.split_first() else {
                return out;
            };
            self.pending.push(next);
            bytes = rest;
            if std::str::from_utf8(&self.pending).is_err_and(|e| e.error_len().is_none()) {
                continue; // still truncated
            }
            let pending = std::mem::take(&mut self.pending);
            self.pending = decode_into(&pending, &mut out).to_vec();
        }
        let tail = decode_into(bytes, &mut out);
        self.pending.extend_from_slice(tail);
        out
    }

    /// Flushes whatever incomplete sequence is still held back (as U+FFFD) —
    /// call once the stream has ended.
    pub fn finish(&mut self) -> String {
        let out = String::from_utf8_lossy(&self.pending).into_owned();
        self.pending.clear();
        out
    }
}

/// Appends everything decodable in `bytes` to `out` (invalid sequences as
/// U+FFFD) and returns the truncated multi-byte sequence left at the end, if any.
fn decode_into<'a>(mut bytes: &'a [u8], out: &mut String) -> &'a [u8] {
    loop {
        match std::str::from_utf8(bytes) {
            Ok(valid) => {
                out.push_str(valid);
                return &[];
            }
            Err(err) => {
                let (valid, after) = bytes.split_at(err.valid_up_to());
                // `valid_up_to` guarantees this prefix is valid UTF-8.
                out.push_str(std::str::from_utf8(valid).unwrap_or_default());
                match err.error_len() {
                    // Invalid sequence in the middle — replace it and keep going.
                    Some(len) => {
                        out.push(char::REPLACEMENT_CHARACTER);
                        bytes = &after[len..];
                    }
                    // Truncated sequence at the very end — wait for more bytes.
                    None => return after,
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn utf8_chunker_carries_split_characters_to_the_next_chunk() {
        let text = "héllo → 世界 🦀";
        let bytes = text.as_bytes();
        // Every possible single split point, including mid-character ones.
        for split in 0..=bytes.len() {
            let mut chunker = Utf8Chunker::new();
            let mut out = chunker.push(&bytes[..split]);
            out.push_str(&chunker.push(&bytes[split..]));
            out.push_str(&chunker.finish());
            assert_eq!(out, text, "split at {split}");
        }
    }

    #[test]
    fn utf8_chunker_byte_at_a_time() {
        let text = "a€b🦀c";
        let mut chunker = Utf8Chunker::new();
        let out: String = text
            .as_bytes()
            .iter()
            .map(|b| chunker.push(&[*b]))
            .collect();
        assert_eq!(out, text);
    }

    #[test]
    fn utf8_chunker_replaces_invalid_bytes() {
        let mut chunker = Utf8Chunker::new();
        assert_eq!(chunker.push(b"a\xffb"), "a\u{FFFD}b");
        // A truncated sequence is held back, then flushed lossily at the end.
        assert_eq!(chunker.push(b"c\xe2\x82"), "c");
        assert_eq!(chunker.finish(), "\u{FFFD}");
        assert_eq!(chunker.finish(), "");
    }
}
