//! What a terminal says to the user (see [`substrate_message`]).

substrate_message::messages! {
    terminal {
        CANT_START = "Couldn't start the shell: {error}",
        FAILED = "The terminal failed: {error}",
    }
}
