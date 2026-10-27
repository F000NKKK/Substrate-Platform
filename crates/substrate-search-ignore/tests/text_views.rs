//! A kind of file the product reads through a text view.

use std::path::Path;

use substrate_platform::text_view::{TextView, TextViews};
use substrate_search_app::ports::{Decoding, DocumentSource};
use substrate_search_ignore::DiskDocuments;

/// A kind of file kept reversed on disk, read and written through its view.
#[test]
fn a_registered_view_is_what_search_reads() {
    fn read(p: &Path) -> Option<String> {
        Some(std::fs::read_to_string(p).ok()?.chars().rev().collect())
    }
    fn write(p: &Path, text: &str) -> std::io::Result<()> {
        std::fs::write(p, text.chars().rev().collect::<String>())
    }
    let views = TextViews::default().with("reversed", TextView { read, write });
    let dir = std::env::temp_dir().join(format!("substrate-views-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let file = dir.join("a.reversed");
    std::fs::write(&file, "olleh").unwrap();
    let documents = DiskDocuments::new(views);
    assert_eq!(
        documents.read_text(&file, Decoding::Strict).as_deref(),
        Some("hello")
    );
    documents.write_text(&file, "bye").unwrap();
    assert_eq!(std::fs::read_to_string(&file).unwrap(), "eyb");
    std::fs::remove_dir_all(&dir).unwrap();
}
