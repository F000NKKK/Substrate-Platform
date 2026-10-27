//! Parsing a workflow file.

use substrate_run_domain::workflow::File;

use crate::FileError;
use crate::file_dto::FileDto;

/// Parses a workflow file's text.
///
/// # Errors
///
/// The TOML error of text that is not a workflow file.
pub fn parse_workflow(text: &str) -> Result<File, FileError> {
    Ok(toml::from_str::<FileDto>(text)?.into())
}

#[cfg(test)]
mod tests {
    use substrate_run_domain::workflow::Step;

    use super::*;

    const SAMPLE: &str = r#"
[workflow.build]
description = "Compile"
steps = [
{ run = "cargo", args = ["build", "--target-dir", "${out}"], cwd = "crates/${name}", env = { MODE = "${mode}" } },
]

[workflow.ci]
steps = [{ workflow = "build" }, { run = "cargo", args = ["test"] }]
"#;

    #[test]
    fn run_and_compose_steps_are_read() {
        let file = parse_workflow(SAMPLE).unwrap();

        let build = &file.workflow["build"];
        assert_eq!(build.description.as_deref(), Some("Compile"));
        let Step::Run {
            run,
            args,
            cwd,
            env,
        } = &build.steps[0]
        else {
            panic!("expected a run step");
        };
        assert_eq!(run, "cargo");
        assert_eq!(args, &["build", "--target-dir", "${out}"]);
        assert_eq!(cwd.as_deref(), Some("crates/${name}"));
        assert_eq!(env["MODE"], "${mode}");

        let ci = &file.workflow["ci"];
        assert!(ci.description.is_none());
        assert!(matches!(&ci.steps[0], Step::Compose { workflow } if workflow == "build"));
        assert!(
            matches!(&ci.steps[1], Step::Run { args: a, cwd: c, env: e, .. } if a == &["test"] && c.is_none() && e.is_empty())
        );
    }

    #[test]
    fn invalid_toml_is_rejected() {
        assert!(parse_workflow("[workflow.x]\nsteps = 3").is_err());
    }
}
