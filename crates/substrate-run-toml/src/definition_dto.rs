//! One workflow as the file writes it.

use serde::Deserialize;
use substrate_run_domain::workflow::Definition;

use crate::step_dto::StepDto;

/// One workflow as the file writes it.
#[derive(Deserialize)]
pub(crate) struct DefinitionDto {
    description: Option<String>,
    steps: Vec<StepDto>,
}

impl From<DefinitionDto> for Definition {
    fn from(definition: DefinitionDto) -> Self {
        Self {
            description: definition.description,
            steps: definition.steps.into_iter().map(Into::into).collect(),
        }
    }
}
