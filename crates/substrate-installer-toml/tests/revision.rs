//! Taking a catalog only when it is newer.

use substrate_installer_toml::{CatalogFileError, newer_revision};

const FILE: &str = "revision = 5\n[[group]]\nid = \"g\"\nname = \"G\"\n";

#[test]
fn a_newer_catalog_is_read() {
    let catalog = newer_revision(FILE, 4).unwrap().unwrap();
    assert_eq!((catalog.revision(), catalog.groups().len()), (5, 1));
}

#[test]
fn an_equal_or_older_catalog_is_left_unread() {
    assert!(newer_revision(FILE, 5).unwrap().is_none());
    assert!(newer_revision(FILE, 9).unwrap().is_none());
}

#[test]
fn an_older_catalog_is_not_checked() {
    let broken = "revision = 1\n[[group]]\nid = \"BAD\"\nname = \"G\"\n";
    assert!(newer_revision(broken, 2).unwrap().is_none());
    assert!(matches!(
        newer_revision(broken, 0),
        Err(CatalogFileError::Invalid(_))
    ));
}

#[test]
fn a_file_of_the_wrong_shape_is_refused_either_way() {
    assert!(matches!(
        newer_revision("revision = \"x\"", 9),
        Err(CatalogFileError::Syntax { .. })
    ));
}

#[test]
fn a_missing_revision_is_the_oldest() {
    assert!(newer_revision("", 0).unwrap().is_none());
    assert!(newer_revision("", 0).is_ok());
}
