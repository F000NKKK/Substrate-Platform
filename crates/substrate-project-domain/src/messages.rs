//! What projects and solutions say to the user (see [`substrate_message`]).
//! A format's own failures are the product's to word.

substrate_message::messages! {
    project {
        INVALID_STANDARDS = "The list of project standards is invalid: {error}",
        NO_FORMAT = "No format this app knows reads {path}.",
        DEPENDENCY_CYCLE = "{project} depends on itself through its build dependencies.",
        UNKNOWN_DEPENDENCY = "{project} depends on {dependency}, which isn't in the solution.",
    }
}
