//! A request's command.

use std::fmt;

/// The command of a request to the adapter. The ones Substrate speaks
/// itself are named; any other (`setVariable`, `source`, `completions`…)
/// passes through as [`Command::Other`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    /// `initialize`.
    Initialize,
    /// `launch`.
    Launch,
    /// `attach`.
    Attach,
    /// `setBreakpoints`.
    SetBreakpoints,
    /// `setExceptionBreakpoints`.
    SetExceptionBreakpoints,
    /// `configurationDone`.
    ConfigurationDone,
    /// `threads`.
    Threads,
    /// `stackTrace`.
    StackTrace,
    /// `scopes`.
    Scopes,
    /// `variables`.
    Variables,
    /// `continue`.
    Continue,
    /// `next`.
    Next,
    /// `stepIn`.
    StepIn,
    /// `stepOut`.
    StepOut,
    /// `pause`.
    Pause,
    /// `evaluate`.
    Evaluate,
    /// `disconnect`.
    Disconnect,
    /// `terminate`.
    Terminate,
    /// Any other command, by its name.
    Other(String),
}

impl Command {
    /// The command as the protocol names it.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Initialize => "initialize",
            Self::Launch => "launch",
            Self::Attach => "attach",
            Self::SetBreakpoints => "setBreakpoints",
            Self::SetExceptionBreakpoints => "setExceptionBreakpoints",
            Self::ConfigurationDone => "configurationDone",
            Self::Threads => "threads",
            Self::StackTrace => "stackTrace",
            Self::Scopes => "scopes",
            Self::Variables => "variables",
            Self::Continue => "continue",
            Self::Next => "next",
            Self::StepIn => "stepIn",
            Self::StepOut => "stepOut",
            Self::Pause => "pause",
            Self::Evaluate => "evaluate",
            Self::Disconnect => "disconnect",
            Self::Terminate => "terminate",
            Self::Other(name) => name,
        }
    }
}

impl From<&str> for Command {
    fn from(name: &str) -> Self {
        const NAMED: [Command; 18] = [
            Command::Initialize,
            Command::Launch,
            Command::Attach,
            Command::SetBreakpoints,
            Command::SetExceptionBreakpoints,
            Command::ConfigurationDone,
            Command::Threads,
            Command::StackTrace,
            Command::Scopes,
            Command::Variables,
            Command::Continue,
            Command::Next,
            Command::StepIn,
            Command::StepOut,
            Command::Pause,
            Command::Evaluate,
            Command::Disconnect,
            Command::Terminate,
        ];
        NAMED
            .into_iter()
            .find(|command| command.as_str() == name)
            .unwrap_or_else(|| Self::Other(name.to_string()))
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn commands_are_named_as_the_protocol_does() {
        assert_eq!(Command::from("stackTrace"), Command::StackTrace);
        assert_eq!(Command::from("setVariable").as_str(), "setVariable");
        assert_eq!(
            Command::from("setVariable"),
            Command::Other("setVariable".into())
        );
        assert_eq!(Command::StepOut.to_string(), "stepOut");
    }

    #[test]
    fn every_named_command_round_trips() {
        for name in [
            "initialize",
            "launch",
            "attach",
            "setBreakpoints",
            "setExceptionBreakpoints",
            "configurationDone",
            "threads",
            "stackTrace",
            "scopes",
            "variables",
            "continue",
            "next",
            "stepIn",
            "stepOut",
            "pause",
            "evaluate",
            "disconnect",
            "terminate",
        ] {
            let command = Command::from(name);
            assert!(!matches!(command, Command::Other(_)), "{name}");
            assert_eq!(command.as_str(), name);
        }
    }
}
