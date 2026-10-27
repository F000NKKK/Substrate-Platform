//! What a debug adapter says to the user (see [`crate::message`]).

crate::messages! {
    dap {
        INVALID_LIST = "The list of debug adapters is invalid: {error}",
        NO_ADAPTER = "There's no debug adapter {adapter}.",
        CANT_START = "Couldn't start the debug adapter {adapter}: {error}",
        FAILED = "The debug adapter {adapter} failed: {error}",
        STOPPED = "The debug adapter {adapter} has stopped.",
        TIMED_OUT = "{adapter} didn't answer {command} in time.",
        ERROR = "{adapter} couldn't do {command}: {error}",
    }
}
