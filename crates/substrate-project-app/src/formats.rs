//! The formats a product registers, and opening a file with the one that reads it.

use std::fmt;
use std::path::Path;

use substrate_project_domain::{Project, ProjectError, Solution};

use crate::ports::{ProjectFormat, SolutionFormat};

/// The formats a product knows: a file opens with the first that reads it.
#[derive(Default)]
pub struct Formats {
    solutions: Vec<Box<dyn SolutionFormat>>,
    projects: Vec<Box<dyn ProjectFormat>>,
}

impl fmt::Debug for Formats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Formats")
            .field("solutions", &self.solutions.len())
            .field("projects", &self.projects.len())
            .finish_non_exhaustive()
    }
}

impl Formats {
    /// These formats and `format`, tried after them.
    #[must_use]
    pub fn with_solution(mut self, format: impl SolutionFormat + 'static) -> Self {
        self.solutions.push(Box::new(format));
        self
    }

    /// These formats and `format`, tried after them.
    #[must_use]
    pub fn with_project(mut self, format: impl ProjectFormat + 'static) -> Self {
        self.projects.push(Box::new(format));
        self
    }

    /// The first solution format that reads `path`.
    #[must_use]
    pub fn solution_format(&self, path: &Path) -> Option<&dyn SolutionFormat> {
        self.solutions
            .iter()
            .map(Box::as_ref)
            .find(|f| f.reads(path))
    }

    /// The first project format that reads `path`.
    #[must_use]
    pub fn project_format(&self, path: &Path) -> Option<&dyn ProjectFormat> {
        self.projects
            .iter()
            .map(Box::as_ref)
            .find(|f| f.reads(path))
    }

    /// The solution in `path`, read by its format.
    ///
    /// # Errors
    ///
    /// No format reads `path`, or what the format refuses.
    pub fn load_solution(&self, path: &Path) -> Result<Solution, ProjectError> {
        self.solution_format(path)
            .ok_or_else(|| no_format(path))?
            .load(path)
            .map_err(ProjectError::Format)
    }

    /// Writes `solution` to `path` in its format.
    ///
    /// # Errors
    ///
    /// No format reads `path`, or what the format refuses.
    pub fn save_solution(&self, solution: &Solution, path: &Path) -> Result<(), ProjectError> {
        self.solution_format(path)
            .ok_or_else(|| no_format(path))?
            .save(solution, path)
            .map_err(ProjectError::Format)
    }

    /// The project in `path`, read by its format.
    ///
    /// # Errors
    ///
    /// No format reads `path`, or what the format refuses.
    pub fn load_project(&self, path: &Path) -> Result<Project, ProjectError> {
        self.project_format(path)
            .ok_or_else(|| no_format(path))?
            .load(path)
            .map_err(ProjectError::Format)
    }

    /// Writes `project` to `path` in its format.
    ///
    /// # Errors
    ///
    /// No format reads `path`, or what the format refuses.
    pub fn save_project(&self, project: &Project, path: &Path) -> Result<(), ProjectError> {
        self.project_format(path)
            .ok_or_else(|| no_format(path))?
            .save(project, path)
            .map_err(ProjectError::Format)
    }
}

/// The message for a file no format reads.
fn no_format(path: &Path) -> ProjectError {
    ProjectError::NoFormat {
        path: path.to_path_buf(),
    }
}

#[cfg(test)]
mod tests {
    use substrate_message::Message;
    use substrate_project_domain::messages;

    use super::*;

    /// A format reading files of one extension, into an empty solution named by it.
    struct ByExtension(&'static str);

    impl SolutionFormat for ByExtension {
        fn name(&self) -> &str {
            self.0
        }

        fn reads(&self, path: &Path) -> bool {
            path.extension().is_some_and(|x| x == self.0)
        }

        fn load(&self, _path: &Path) -> Result<Solution, Message> {
            Ok(Solution {
                name: self.0.into(),
                ..Default::default()
            })
        }

        fn save(&self, _solution: &Solution, _path: &Path) -> Result<(), Message> {
            Ok(())
        }
    }

    #[test]
    fn a_file_opens_with_the_format_that_reads_it() {
        let formats = Formats::default()
            .with_solution(ByExtension("sln"))
            .with_solution(ByExtension("slnx"));
        assert_eq!(
            formats.load_solution(Path::new("App.slnx")).unwrap().name,
            "slnx"
        );
        assert_eq!(
            formats.load_solution(Path::new("App.sln")).unwrap().name,
            "sln"
        );
        let error = formats.load_solution(Path::new("App.toml")).unwrap_err();
        assert!(matches!(error, ProjectError::NoFormat { .. }));
        assert_eq!(Message::from(error).text, messages::NO_FORMAT);
    }
}
