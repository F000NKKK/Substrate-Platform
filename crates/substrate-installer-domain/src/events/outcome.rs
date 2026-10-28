//! How a plan ended.

/// How a plan ended.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Outcome {
    /// Every change was made.
    Completed,
    /// The user cancelled it.
    Cancelled,
    /// A change failed; the plan stopped there.
    Failed {
        /// Why, rendered once for a person from the error.
        reason: String,
    },
}

impl Outcome {
    /// Whether every change was made.
    #[must_use]
    pub fn is_completed(&self) -> bool {
        matches!(self, Self::Completed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_a_completed_plan_is_completed() {
        assert!(Outcome::Completed.is_completed());
        assert!(!Outcome::Cancelled.is_completed());
        assert!(!Outcome::Failed { reason: "x".into() }.is_completed());
    }
}
