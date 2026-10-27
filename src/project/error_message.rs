//! A [`ProjectError`] as the user sees it.

use substrate_message::Message;

use super::{ProjectError, messages};

impl From<ProjectError> for Message {
    fn from(error: ProjectError) -> Self {
        match error {
            ProjectError::NoFormat { path } => {
                Message::new(messages::NO_FORMAT).arg("path", path.display())
            }
            ProjectError::DependencyCycle { project } => {
                Message::new(messages::DEPENDENCY_CYCLE).arg("project", project)
            }
            ProjectError::UnknownDependency {
                project,
                dependency,
            } => Message::new(messages::UNKNOWN_DEPENDENCY)
                .arg("project", project)
                .arg("dependency", dependency),
            ProjectError::Format(message) => message,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_formats_own_words_pass_through() {
        let own = Message::verbatim("bad csproj");
        assert_eq!(Message::from(ProjectError::Format(own.clone())), own);
        let message = Message::from(ProjectError::UnknownDependency {
            project: "app".into(),
            dependency: "lib".into(),
        });
        assert_eq!(message.text, messages::UNKNOWN_DEPENDENCY);
        assert_eq!(message.args["dependency"], "lib");
    }
}
