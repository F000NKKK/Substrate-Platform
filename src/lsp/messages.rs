//! What a language server says to the user (see [`crate::message`]).

crate::messages! {
    lsp {
        INVALID_LIST = "The list of languages is invalid: {error}",
        NO_SERVER = "There's no language server for {language}.",
        CANT_START = "Couldn't start the language server {server}: {error}",
        FAILED = "The language server {server} failed: {error}",
        STOPPED = "The language server {server} has stopped.",
        TIMED_OUT = "{server} didn't answer {method} in time.",
        ERROR = "{server} couldn't do {method}: {error}",
    }
}
