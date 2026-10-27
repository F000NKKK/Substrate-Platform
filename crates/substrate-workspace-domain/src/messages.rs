//! What the workspace says to the user (see [`substrate_message`]).

substrate_message::messages! {
    workspace {
        NOTHING_OPEN = "No project is open.",
        NOT_ON_DISK = "{path} doesn't resolve to anything on disk.",
        OUTSIDE = "{path} is outside any open project.",
    }
}
