//! Why a request or a transfer failed.

use std::path::PathBuf;

/// Why a request or a transfer failed. It is the `cause` the adapters of this
/// crate put into the ports' errors, so a caller that wants to tell a rate
/// limit from other failures asks [`is_rate_limited`].
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum HttpError {
    /// The server did not answer (no route, TLS, timeout, a broken line).
    #[error("{url} could not be reached")]
    Transport {
        /// What was asked.
        url: String,
        /// What went wrong.
        #[source]
        source: Box<ureq::Error>,
    },
    /// The body stopped arriving or could not be read.
    #[error("the answer of {url} could not be read")]
    Body {
        /// What was asked.
        url: String,
        /// What went wrong.
        #[source]
        source: std::io::Error,
    },
    /// The server refused to answer for now: too many requests.
    #[error("{url} is rate limited{}", retry_hint(*.retry_after_seconds))]
    RateLimited {
        /// What was asked.
        url: String,
        /// Seconds the server asks to wait, where it says.
        retry_after_seconds: Option<u64>,
    },
    /// The server answered with a status that is not a success.
    #[error("{url} answered {status}")]
    Status {
        /// What was asked.
        url: String,
        /// The HTTP status.
        status: u16,
    },
    /// The server cannot resume a download that was already handed on.
    #[error("{url} cannot resume the download; it has to start over")]
    ResumeUnsupported {
        /// What was asked.
        url: String,
    },
    /// A file of the transfer could not be read or written.
    #[error("{} could not be read or written", path.display())]
    Io {
        /// The file.
        path: PathBuf,
        /// What went wrong.
        #[source]
        source: std::io::Error,
    },
}

fn retry_hint(seconds: Option<u64>) -> String {
    seconds.map_or_else(String::new, |seconds| format!(" (retry in {seconds} s)"))
}

/// Whether `error`, or something it was caused by, is a rate limit: the
/// listing and download errors of the ports keep an [`HttpError`] as their
/// source, and a caller shows "try later" instead of "failed".
#[must_use]
pub fn is_rate_limited(error: &(dyn std::error::Error + 'static)) -> bool {
    let mut current = Some(error);
    while let Some(link) = current {
        if matches!(
            link.downcast_ref::<HttpError>(),
            Some(HttpError::RateLimited { .. })
        ) {
            return true;
        }
        current = link.source();
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_rate_limit_is_found_through_the_source_chain() {
        let limited = HttpError::RateLimited {
            url: "https://x.y".to_owned(),
            retry_after_seconds: Some(60),
        };
        assert_eq!(
            limited.to_string(),
            "https://x.y is rate limited (retry in 60 s)"
        );
        assert!(is_rate_limited(&limited));
        let other = HttpError::Status {
            url: "https://x.y".to_owned(),
            status: 500,
        };
        assert!(!is_rate_limited(&other));
    }
}
