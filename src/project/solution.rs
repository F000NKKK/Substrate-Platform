//! What an IDE asks of a [`Solution`]: its projects and folders by id, the
//! tree they make, which configuration of a project a solution
//! configuration builds, and the order to build them in.

use std::collections::BTreeSet;

use super::messages;
use super::model::{Configuration, ConfigurationMapping, ProjectEntry, Solution, SolutionFolder};
use crate::message::Message;

impl Solution {
    pub fn project(&self, id: &str) -> Option<&ProjectEntry> {
        self.projects.iter().find(|p| p.id == id)
    }

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

    /// What `configuration` does with `project`: its own mapping, else the
    /// project's configuration of the same name, built.
    pub fn mapping(
        &self,
        project: &ProjectEntry,
        configuration: &Configuration,
    ) -> ConfigurationMapping {
        project
            .configurations
            .iter()
            .find(|m| &m.solution == configuration)
            .cloned()
            .unwrap_or_else(|| ConfigurationMapping {
                solution: configuration.clone(),
                project: configuration.clone(),
                build: true,
                deploy: false,
            })
    }

    /// The projects `configuration` builds, each after its dependencies.
    pub fn build_order(
        &self,
        configuration: &Configuration,
    ) -> Result<Vec<&ProjectEntry>, Message> {
        let mut order = Vec::new();
        let (mut done, mut visiting) = (BTreeSet::new(), BTreeSet::new());
        for project in &self.projects {
            self.visit(project, &mut done, &mut visiting, &mut order)?;
        }
        order.retain(|p| self.mapping(p, configuration).build);
        Ok(order)
    }

    /// `project` into `order` after its dependencies (depth first).
    fn visit<'s>(
        &'s self,
        project: &'s ProjectEntry,
        done: &mut BTreeSet<&'s str>,
        visiting: &mut BTreeSet<&'s str>,
        order: &mut Vec<&'s ProjectEntry>,
    ) -> Result<(), Message> {
        if done.contains(project.id.as_str()) {
            return Ok(());
        }
        if !visiting.insert(project.id.as_str()) {
            return Err(Message::new(messages::DEPENDENCY_CYCLE).arg("project", &project.name));
        }
        for dependency in &project.dependencies {
            let Some(next) = self.project(dependency) else {
                return Err(Message::new(messages::UNKNOWN_DEPENDENCY)
                    .arg("project", &project.name)
                    .arg("dependency", dependency));
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

    fn project(id: &str, dependencies: &[&str]) -> ProjectEntry {
        ProjectEntry {
            id: id.into(),
            name: id.into(),
            dependencies: dependencies.iter().map(|d| d.to_string()).collect(),
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
        assert_eq!(
            solution.build_order(&debug()).unwrap_err().text,
            messages::DEPENDENCY_CYCLE
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
