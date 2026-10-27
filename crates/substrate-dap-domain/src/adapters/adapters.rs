//! The adapters a product knows.

use crate::{Adapter, AdapterId};

/// A product's debug adapters, in the order of their ids.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Adapters {
    adapters: Vec<Adapter>,
}

impl Adapters {
    /// These adapters, in this order.
    #[must_use]
    pub fn new(adapters: Vec<Adapter>) -> Self {
        Self { adapters }
    }

    /// Every adapter.
    #[must_use]
    pub fn all(&self) -> &[Adapter] {
        &self.adapters
    }

    /// The adapter `id`.
    #[must_use]
    pub fn find(&self, id: &AdapterId) -> Option<&Adapter> {
        self.adapters.iter().find(|a| &a.id == id)
    }

    /// The first adapter that debugs `language`.
    #[must_use]
    pub fn for_language(&self, language: &str) -> Option<&Adapter> {
        self.adapters.iter().find(|a| a.debugs(language))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::AdapterDefinition;

    fn adapter(id: &str, languages: &[&str]) -> Adapter {
        Adapter {
            id: AdapterId::new(id),
            languages: languages.iter().map(ToString::to_string).collect(),
            definition: AdapterDefinition {
                command: id.into(),
                args: Vec::new(),
                env: Default::default(),
                launch_defaults: None,
            },
        }
    }

    #[test]
    fn adapters_are_found_by_id_and_language() {
        let adapters = Adapters::new(vec![
            adapter("lldb", &["rust", "c", "cpp"]),
            adapter("python", &["python"]),
        ]);
        assert_eq!(adapters.all().len(), 2);
        let lldb = adapters.find(&AdapterId::new("lldb")).unwrap();
        assert!(lldb.debugs("c"));
        assert_eq!(adapters.for_language("rust").unwrap().id.as_str(), "lldb");
        assert_eq!(
            adapters.for_language("python").unwrap().id.as_str(),
            "python"
        );
        assert!(adapters.for_language("go").is_none());
        assert!(adapters.find(&AdapterId::new("gdb")).is_none());
    }
}
