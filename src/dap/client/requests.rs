//! The requests a debug session makes, with their arguments typed. Results
//! are the body as the adapter sent it (JSON): the session that asked reads
//! it.

use std::path::Path;

use serde_json::{Value, json};

use crate::dap::{
    AdapterId, Client, Command, DapError, Debuggee, EvaluateContext, FrameId, ThreadId,
    VariablesReference,
};

impl Client {
    /// `initialize`: what this client is and can, and the adapter's
    /// capabilities in return. The adapter's `initialized` event follows
    /// (after `launch`/`attach` for most).
    ///
    /// # Errors
    ///
    /// The adapter has stopped, does not answer in time, or refuses.
    pub fn initialize(&self, adapter: &AdapterId) -> Result<Value, DapError> {
        self.request(
            &Command::Initialize,
            json!({
                "clientID": "substrate-platform",
                "clientName": "Substrate Platform",
                "adapterID": adapter,
                "locale": "en",
                "linesStartAt1": true,
                "columnsStartAt1": true,
                "pathFormat": "path",
                "supportsVariableType": true,
                "supportsRunInTerminalRequest": false,
                "supportsStartDebuggingRequest": false,
            }),
        )
    }

    /// `launch` with `arguments` (the adapter's own, see
    /// [`crate::dap::AdapterDefinition::launch_arguments`]). Many adapters answer
    /// only after `configurationDone`: call it from a thread of its own.
    ///
    /// # Errors
    ///
    /// The adapter has stopped, does not answer in time, or refuses.
    pub fn launch(&self, arguments: Value) -> Result<Value, DapError> {
        self.request(&Command::Launch, arguments)
    }

    /// `attach` with `arguments` (the adapter's own).
    ///
    /// # Errors
    ///
    /// The adapter has stopped, does not answer in time, or refuses.
    pub fn attach(&self, arguments: Value) -> Result<Value, DapError> {
        self.request(&Command::Attach, arguments)
    }

    /// `setBreakpoints`: the breakpoints on `lines` (1-based) of `path`
    /// replace the file's earlier ones. The body lists them as verified.
    ///
    /// # Errors
    ///
    /// The adapter has stopped, does not answer in time, or refuses.
    pub fn set_breakpoints(&self, path: &Path, lines: &[u32]) -> Result<Value, DapError> {
        let breakpoints: Vec<Value> = lines.iter().map(|line| json!({ "line": line })).collect();
        self.request(
            &Command::SetBreakpoints,
            json!({
                "source": { "path": path.to_string_lossy() },
                "breakpoints": breakpoints,
            }),
        )
    }

    /// `setExceptionBreakpoints`: stop on the exception kinds `filters` name
    /// (the adapter's `exceptionBreakpointFilters`).
    ///
    /// # Errors
    ///
    /// The adapter has stopped, does not answer in time, or refuses.
    pub fn set_exception_breakpoints(&self, filters: &[String]) -> Result<Value, DapError> {
        self.request(
            &Command::SetExceptionBreakpoints,
            json!({ "filters": filters }),
        )
    }

    /// `configurationDone`: the breakpoints are set, the debuggee may run.
    ///
    /// # Errors
    ///
    /// The adapter has stopped, does not answer in time, or refuses.
    pub fn configuration_done(&self) -> Result<Value, DapError> {
        self.request(&Command::ConfigurationDone, json!({}))
    }

    /// `threads`: the debuggee's threads.
    ///
    /// # Errors
    ///
    /// The adapter has stopped, does not answer in time, or refuses.
    pub fn threads(&self) -> Result<Value, DapError> {
        self.request(&Command::Threads, json!({}))
    }

    /// `stackTrace` of `thread`, at most `levels` frames from the top
    /// (`0`: all of them).
    ///
    /// # Errors
    ///
    /// The adapter has stopped, does not answer in time, or refuses.
    pub fn stack_trace(&self, thread: ThreadId, levels: u32) -> Result<Value, DapError> {
        self.request(
            &Command::StackTrace,
            json!({ "threadId": thread, "startFrame": 0, "levels": levels }),
        )
    }

    /// `scopes` of `frame`: its locals, arguments, registers…
    ///
    /// # Errors
    ///
    /// The adapter has stopped, does not answer in time, or refuses.
    pub fn scopes(&self, frame: FrameId) -> Result<Value, DapError> {
        self.request(&Command::Scopes, json!({ "frameId": frame }))
    }

    /// `variables` under `reference` (a scope or a structured variable).
    ///
    /// # Errors
    ///
    /// The adapter has stopped, does not answer in time, or refuses.
    pub fn variables(&self, reference: VariablesReference) -> Result<Value, DapError> {
        self.request(
            &Command::Variables,
            json!({ "variablesReference": reference }),
        )
    }

    /// `continue`: `thread` runs on.
    ///
    /// # Errors
    ///
    /// The adapter has stopped, does not answer in time, or refuses.
    pub fn resume(&self, thread: ThreadId) -> Result<Value, DapError> {
        self.request(&Command::Continue, json!({ "threadId": thread }))
    }

    /// `next`: `thread` steps over the line.
    ///
    /// # Errors
    ///
    /// The adapter has stopped, does not answer in time, or refuses.
    pub fn step_over(&self, thread: ThreadId) -> Result<Value, DapError> {
        self.request(&Command::Next, json!({ "threadId": thread }))
    }

    /// `stepIn`: `thread` steps into the call.
    ///
    /// # Errors
    ///
    /// The adapter has stopped, does not answer in time, or refuses.
    pub fn step_in(&self, thread: ThreadId) -> Result<Value, DapError> {
        self.request(&Command::StepIn, json!({ "threadId": thread }))
    }

    /// `stepOut`: `thread` runs until its function returns.
    ///
    /// # Errors
    ///
    /// The adapter has stopped, does not answer in time, or refuses.
    pub fn step_out(&self, thread: ThreadId) -> Result<Value, DapError> {
        self.request(&Command::StepOut, json!({ "threadId": thread }))
    }

    /// `pause`: `thread` stops where it is.
    ///
    /// # Errors
    ///
    /// The adapter has stopped, does not answer in time, or refuses.
    pub fn pause(&self, thread: ThreadId) -> Result<Value, DapError> {
        self.request(&Command::Pause, json!({ "threadId": thread }))
    }

    /// `evaluate` `expression` in `frame` (the debuggee's top frame if
    /// `None`), formatted for `context`.
    ///
    /// # Errors
    ///
    /// The adapter has stopped, does not answer in time, or refuses.
    pub fn evaluate(
        &self,
        expression: &str,
        frame: Option<FrameId>,
        context: EvaluateContext,
    ) -> Result<Value, DapError> {
        let mut arguments = json!({ "expression": expression, "context": context.as_str() });
        if let Some(frame) = frame {
            arguments["frameId"] = json!(frame);
        }
        self.request(&Command::Evaluate, arguments)
    }

    /// `disconnect`; `debuggee` says what becomes of the program.
    ///
    /// # Errors
    ///
    /// The adapter has stopped, does not answer in time, or refuses.
    pub fn disconnect(&self, debuggee: Debuggee) -> Result<Value, DapError> {
        self.request(
            &Command::Disconnect,
            json!({ "terminateDebuggee": debuggee == Debuggee::Terminate }),
        )
    }

    /// `terminate`: asks the debuggee to end itself (the adapter may not
    /// support it; [`Self::disconnect`] always works).
    ///
    /// # Errors
    ///
    /// The adapter has stopped, does not answer in time, or refuses.
    pub fn terminate(&self) -> Result<Value, DapError> {
        self.request(&Command::Terminate, json!({}))
    }
}
