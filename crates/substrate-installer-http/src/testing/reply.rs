//! What the mock server answers.

/// A response: a status, headers and a body.
#[derive(Debug, Clone)]
pub(crate) struct Reply {
    status: u16,
    headers: Vec<(String, String)>,
    body: Vec<u8>,
}

impl Reply {
    pub(crate) fn status(status: u16) -> Self {
        Self {
            status,
            headers: Vec::new(),
            body: Vec::new(),
        }
    }

    pub(crate) fn ok(body: impl Into<Vec<u8>>) -> Self {
        Self::status(200).body(body)
    }

    pub(crate) fn header(mut self, name: &str, value: &str) -> Self {
        self.headers.push((name.to_owned(), value.to_owned()));
        self
    }

    pub(crate) fn body(mut self, body: impl Into<Vec<u8>>) -> Self {
        self.body = body.into();
        self
    }

    /// The bytes on the wire; the connection closes after them.
    pub(crate) fn to_bytes(&self) -> Vec<u8> {
        let mut lines = vec![format!("HTTP/1.1 {} X", self.status)];
        lines.extend(
            self.headers
                .iter()
                .map(|(name, value)| format!("{name}: {value}")),
        );
        lines.push(format!("Content-Length: {}", self.body.len()));
        lines.push("Connection: close".to_owned());
        let head = lines.join("\r\n") + "\r\n\r\n";
        let mut bytes = head.into_bytes();
        bytes.extend_from_slice(&self.body);
        bytes
    }
}
