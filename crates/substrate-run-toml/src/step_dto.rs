//! A step as the file writes it.

use std::collections::HashMap;

use serde::Deserialize;
use substrate_run_domain::workflow::Step;

/// A step as the file writes it: a command, or a workflow to compose.
#[derive(Deserialize)]
#[serde(untagged)]
pub(crate) enum StepDto {
    Run {
        run: String,
        #[serde(default)]
        args: Vec<String>,
        cwd: Option<String>,
        #[serde(default)]
        env: HashMap<String, String>,
    },
    Compose {
        workflow: String,
    },
}

impl From<StepDto> for Step {
    fn from(step: StepDto) -> Self {
        match step {
            StepDto::Run {
                run,
                args,
                cwd,
                env,
            } => Step::Run {
                run,
                args,
                cwd,
                env,
            },
            StepDto::Compose { workflow } => Step::Compose { workflow },
        }
    }
}
