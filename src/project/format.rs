//! Solution and project formats: the product's own, registered in [`Formats`].

use std::path::Path;

use super::messages;
use super::model::{Project, Solution};
use crate::message::Message;

/// A solution file format: `.sln`, `.slnx`, a product's own.
pub trait SolutionFormat: Send + Sync {
    /// Its name, for messages: `sln`, `slnx`.
    fn name(&self) -> &str;
    /// Whether `path` is a file of this format (by its name).
    fn reads(&self, path: &Path) -> bool;
    /// The solution in `path`; its project paths resolved against `path`'s folder.
    fn load(&self, path: &Path) -> Result<Solution, Message>;
    fn save(&self, solution: &Solution, path: &Path) -> Result<(), Message>;
}

/// A project file format: `.csproj`, `Cargo.toml`, a product's own.
pub trait ProjectFormat: Send + Sync {
    fn name(&self) -> &str;
    fn reads(&self, path: &Path) -> bool;
    fn load(&self, path: &Path) -> Result<Project, Message>;
    fn save(&self, project: &Project, path: &Path) -> Result<(), Message>;
}

/// The formats a product knows: a file opens with the first that reads it.
#[derive(Default)]
pub struct Formats {
    solutions: Vec<Box<dyn SolutionFormat>>,
    projects: Vec<Box<dyn ProjectFormat>>,
}

impl Formats {
    pub fn with_solution(mut self, format: impl SolutionFormat + 'static) -> Self {
        self.solutions.push(Box::new(format));
        self
    }

    pub fn with_project(mut self, format: impl ProjectFormat + 'static) -> Self {
        self.projects.push(Box::new(format));
        self
    }

    pub fn solution_format(&self, path: &Path) -> Option<&dyn SolutionFormat> {
        self.solutions.iter().map(Box::as_ref).find(|f| f.reads(path))
    }

    pub fn project_format(&self, path: &Path) -> Option<&dyn ProjectFormat> {
        self.projects.iter().map(Box::as_ref).find(|f| f.reads(path))
    }

    pub fn load_solution(&self, path: &Path) -> Result<Solution, Message> {
        self.solution_format(path).ok_or_else(|| no_format(path))?.load(path)
    }

    pub fn save_solution(&self, solution: &Solution, path: &Path) -> Result<(), Message> {
        self.solution_format(path).ok_or_else(|| no_format(path))?.save(solution, path)
    }

    pub fn load_project(&self, path: &Path) -> Result<Project, Message> {
        self.project_format(path).ok_or_else(|| no_format(path))?.load(path)
    }

    pub fn save_project(&self, project: &Project, path: &Path) -> Result<(), Message> {
        self.project_format(path).ok_or_else(|| no_format(path))?.save(project, path)
    }
}

fn no_format(path: &Path) -> Message {
    Message::new(messages::NO_FORMAT).arg("path", path.display())
}

#[cfg(test)]
mod tests {
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
            Ok(Solution { name: self.0.into(), ..Default::default() })
        }

        fn save(&self, _solution: &Solution, _path: &Path) -> Result<(), Message> {
            Ok(())
        }
    }

    #[test]
    fn a_file_opens_with_the_format_that_reads_it() {
        let formats = Formats::default().with_solution(ByExtension("sln")).with_solution(ByExtension("slnx"));
        assert_eq!(formats.load_solution(Path::new("App.slnx")).unwrap().name, "slnx");
        assert_eq!(formats.load_solution(Path::new("App.sln")).unwrap().name, "sln");
        assert_eq!(formats.load_solution(Path::new("App.toml")).unwrap_err().text, messages::NO_FORMAT);
    }
}
