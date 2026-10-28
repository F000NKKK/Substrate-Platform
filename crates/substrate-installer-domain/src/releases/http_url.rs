//! A link.

use crate::texts::define_text;

define_text! {
    /// An `http` or `https` link: a download, a release's notes page.
    HttpUrl, HttpUrl, http_url
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_is_a_web_link() {
        assert!(HttpUrl::new("https://github.com/o/r").is_ok());
        assert!(HttpUrl::new("file:///etc").is_err());
    }
}
