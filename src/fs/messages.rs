//! What file operations say to the user (see [`substrate_message`]).

substrate_message::messages! {
    fs {
        NOT_A_FILE = "{path} isn't a file.",
        ALREADY_EXISTS = "{path} already exists.",
        INTO_ITSELF = "A folder can't be copied into itself.",
        FAILED = "{path}: {error}",
    }
}
