//! Records and stores for the tests.

use std::path::PathBuf;

use substrate_data::{Map, Value};
use substrate_installer_domain::ids::{ComponentId, ItemId, ProductId};
use substrate_installer_domain::inventory::{Installed, InstalledSet};
use substrate_installer_domain::versions::Version;

use crate::BinaryInstalledStore;

pub(super) fn record(item: ItemId, version: &str, path: &str) -> Installed {
    Installed {
        id: item,
        version: Version::new(version).unwrap(),
        path: PathBuf::from(path),
    }
}

pub(super) fn component(id: &str) -> ItemId {
    ItemId::Component(ComponentId::new(id).unwrap())
}

pub(super) fn product(id: &str) -> ItemId {
    ItemId::Product(ProductId::new(id).unwrap())
}

pub(super) fn sample() -> InstalledSet {
    InstalledSet::new(vec![
        record(component("jdk"), "21.0.5+11", "/h/components/jdk/21.0.5+11"),
        record(component("jdk"), "17", "/h/components/jdk/17"),
        record(
            product("yog-idle"),
            "2.0.0",
            "/h/products/yog-idle/2.0.0/yog-idle",
        ),
    ])
}

pub(super) fn store_in(dir: &tempfile::TempDir) -> BinaryInstalledStore {
    BinaryInstalledStore::new(dir.path().join("state/installed.sdat"))
}

/// A map value of `pairs`, in order.
pub(super) fn map<const N: usize>(pairs: [(&str, Value); N]) -> Value {
    let mut map = Map::new();
    for (key, value) in pairs {
        map.insert(key, value);
    }
    Value::Map(map)
}
