//! A solution, and what an IDE asks of it: its projects and folders by id, the
//! tree they make, which configuration of a project a solution
//! configuration builds, and the order to build them in.

use std::collections::BTreeSet;

use crate::{Configuration, ProjectEntry, ProjectError, Section, SolutionFolder};

/// A solution: its projects and how they're built together.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Solution {
    /// Its name.
    pub name: String,
    /// Its configurations.
    pub configurations: Vec<Configuration>,
    /// Its solution folders.
    pub folders: Vec<SolutionFolder>,
    /// Its projects, as it lists them.
    pub projects: Vec<ProjectEntry>,
    /// The format's header settings, in order (a `.sln`'s format and
    /// Visual Studio versions).
    pub properties: Vec<(String, String)>,
    /// What else the format keeps, in order — the model's own parts
    /// (configurations, nesting) stand where the file had them.
    pub sections: Vec<Section>,
}

impl Solution {
    /// The project of `id`.
    #[must_use]
    pub fn project(&self, id: &str) -> Option<&ProjectEntry> {
        self.projects.iter().find(|p| p.id == id)
    }

    /// The solution folder of `id`.
    #[must_use]
    pub fn folder(&self, id: &str) -> Option<&SolutionFolder> {
        self.folders.iter().find(|f| f.id == id)
    }

    /// The folders right under `parent` (`None`: the solution itself).
    pub fn folders_in<'s>(
        &'s self,
        parent: Option<&'s str>,
    ) -> impl Iterator<Item = &'s SolutionFolder> {
        self.folders
            .iter()
            .filter(move |f| f.parent.as_deref() == parent)
    }

    /// The projects right under `folder` (`None`: the solution itself).
    pub fn projects_in<'s>(
        &'s self,
        folder: Option<&'s str>,
    ) -> impl Iterator<Item = &'s ProjectEntry> {
        self.projects
            .iter()
            .filter(move |p| p.folder.as_deref() == folder)
    }

    /// The projects `configuration` builds, each after its dependencies.
    ///
    /// # Errors
    ///
    /// A dependency cycle, or a dependency on a project the solution lacks.
    pub fn build_order(
        &self,
        configuration: &Configuration,
    ) -> Result<Vec<&ProjectEntry>, ProjectError> {
        let mut order = Vec::new();
        let (mut done, mut visiting) = (BTreeSet::new(), BTreeSet::new());
        for project in &self.projects {
            self.visit(project, &mut done, &mut visiting, &mut order)?;
        }
        order.retain(|p| p.mapping(configuration).build);
        Ok(order)
    }

    /// `project` into `order` after its dependencies (depth first).
    fn visit<'s>(
        &'s self,
        project: &'s ProjectEntry,
        done: &mut BTreeSet<&'s str>,
        visiting: &mut BTreeSet<&'s str>,
        order: &mut Vec<&'s ProjectEntry>,
    ) -> Result<(), ProjectError> {
        if done.contains(project.id.as_str()) {
            return Ok(());
        }
        if !visiting.insert(project.id.as_str()) {
            return Err(ProjectError::DependencyCycle {
                project: project.name.clone(),
            });
        }
        for dependency in &project.dependencies {
            let Some(next) = self.project(dependency) else {
                return Err(ProjectError::UnknownDependency {
                    project: project.name.clone(),
                    dependency: dependency.clone(),
                });
            };
            self.visit(next, done, visiting, order)?;
        }
        visiting.remove(project.id.as_str());
        done.insert(project.id.as_str());
        order.push(project);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ConfigurationMapping;

    fn project(id: &str, dependencies: &[&str]) -> ProjectEntry {
        ProjectEntry {
            id: id.into(),
            name: id.into(),
            dependencies: dependencies.iter().map(ToString::to_string).collect(),
            ..Default::default()
        }
    }

    fn debug() -> Configuration {
        Configuration {
            name: "Debug".into(),
            platform: "Any CPU".into(),
        }
    }

    #[test]
    fn projects_build_after_their_dependencies() {
        let solution = Solution {
            projects: vec![
                project("app", &["lib"]),
                project("lib", &["core"]),
                project("core", &[]),
            ],
            ..Default::default()
        };
        let order: Vec<&str> = solution
            .build_order(&debug())
            .unwrap()
            .iter()
            .map(|p| p.id.as_str())
            .collect();
        assert_eq!(order, ["core", "lib", "app"]);
    }

    #[test]
    fn a_cycle_is_refused() {
        let solution = Solution {
            projects: vec![project("a", &["b"]), project("b", &["a"])],
            ..Default::default()
        };
        let error = solution.build_order(&debug()).unwrap_err();
        assert!(matches!(error, ProjectError::DependencyCycle { .. }));
        assert_eq!(
            substrate_message::Message::from(error).text,
            crate::messages::DEPENDENCY_CYCLE
        );
    }

    #[test]
    fn a_configuration_skips_what_it_doesnt_build() {
        let mut skipped = project("tests", &[]);
        skipped.configurations.push(ConfigurationMapping {
            solution: debug(),
            project: debug(),
            build: false,
            deploy: false,
        });
        let solution = Solution {
            projects: vec![project("app", &[]), skipped],
            ..Default::default()
        };
        let order: Vec<&str> = solution
            .build_order(&debug())
            .unwrap()
            .iter()
            .map(|p| p.id.as_str())
            .collect();
        assert_eq!(order, ["app"]);
    }

    #[test]
    fn the_tree_follows_folders() {
        let solution = Solution {
            folders: vec![SolutionFolder {
                id: "src".into(),
                name: "src".into(),
                ..Default::default()
            }],
            projects: vec![
                ProjectEntry {
                    folder: Some("src".into()),
                    ..project("app", &[])
                },
                project("readme", &[]),
            ],
            ..Default::default()
        };
        assert_eq!(solution.folders_in(None).count(), 1);
        assert_eq!(
            solution
                .projects_in(Some("src"))
                .map(|p| p.id.as_str())
                .collect::<Vec<_>>(),
            ["app"]
        );
    }
}
