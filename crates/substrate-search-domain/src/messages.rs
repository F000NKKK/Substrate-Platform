//! What a search says to the user (see [`substrate_message`]).

substrate_message::messages! {
    search {
        EMPTY = "The search text is empty.",
        BAD_PATTERN = "Invalid regular expression: {error}",
        BAD_MASK = "Invalid file mask {mask}: {error}",
        WRITE_FAILED = "{path}: {error}",
    }
}
