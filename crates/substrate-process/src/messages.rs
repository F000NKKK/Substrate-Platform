//! What starting programs says to the user (see [`substrate_message`]).

substrate_message::messages! {
    process {
        NOT_A_LINK = "{url} isn't a web or mail link.",
        CANT_START = "Couldn't start {program}: {error}",
    }
}
