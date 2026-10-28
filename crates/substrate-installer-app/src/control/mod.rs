//! Cancelling and pausing an operation from another thread.

mod cancel_token;
mod cancelled;

pub use cancel_token::CancelToken;
pub use cancelled::Cancelled;
