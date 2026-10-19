//! What a terminal says to the user (see [`crate::message`]).

crate::messages! {
    terminal {
        CANT_START = "Couldn't start the shell: {error}",
        FAILED = "The terminal failed: {error}",
    }
}
