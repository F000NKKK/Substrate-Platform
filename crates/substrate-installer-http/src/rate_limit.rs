//! Telling a rate limit from other refusals.

use ureq::http::HeaderMap;

/// A status that always means "too many requests".
const TOO_MANY_REQUESTS: u16 = 429;
/// GitHub answers a rate limit with this status too.
const FORBIDDEN: u16 = 403;

/// Whether the answer means "too many requests": a 429, or a 403 that says
/// the quota is used up or asks to retry (a plain 403 is a refusal).
pub(crate) fn is_rate_limited(status: u16, headers: &HeaderMap) -> bool {
    match status {
        TOO_MANY_REQUESTS => true,
        FORBIDDEN => quota_used_up(headers) || headers.contains_key("retry-after"),
        _ => false,
    }
}

/// Seconds the server asks to wait.
pub(crate) fn retry_after_seconds(headers: &HeaderMap) -> Option<u64> {
    headers
        .get("retry-after")?
        .to_str()
        .ok()?
        .trim()
        .parse()
        .ok()
}

fn quota_used_up(headers: &HeaderMap) -> bool {
    headers
        .get("x-ratelimit-remaining")
        .and_then(|value| value.to_str().ok())
        .is_some_and(|value| value.trim() == "0")
}

#[cfg(test)]
mod tests {
    use ureq::http::HeaderValue;

    use super::*;

    fn headers(name: &'static str, value: &'static str) -> HeaderMap {
        let mut map = HeaderMap::new();
        map.insert(name, HeaderValue::from_static(value));
        map
    }

    #[test]
    fn only_a_spent_quota_makes_a_403_a_rate_limit() {
        assert!(is_rate_limited(429, &HeaderMap::new()));
        assert!(!is_rate_limited(403, &HeaderMap::new()));
        assert!(is_rate_limited(403, &headers("x-ratelimit-remaining", "0")));
        assert!(!is_rate_limited(
            403,
            &headers("x-ratelimit-remaining", "7")
        ));
        assert!(is_rate_limited(403, &headers("retry-after", "30")));
        assert!(!is_rate_limited(500, &HeaderMap::new()));
    }

    #[test]
    fn the_wait_is_read_where_the_server_says() {
        assert_eq!(
            retry_after_seconds(&headers("retry-after", " 30 ")),
            Some(30)
        );
        assert_eq!(retry_after_seconds(&headers("retry-after", "soon")), None);
        assert_eq!(retry_after_seconds(&HeaderMap::new()), None);
    }
}
