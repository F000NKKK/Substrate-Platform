//! What projects and solutions say to the user (see [`crate::message`]).
//! A format's own failures are the product's to word.

crate::messages! {
    project {
        NO_FORMAT = "No format this app knows reads {path}.",
        DEPENDENCY_CYCLE = "{project} depends on itself through its build dependencies.",
        UNKNOWN_DEPENDENCY = "{project} depends on {dependency}, which isn't in the solution.",
    }
}
