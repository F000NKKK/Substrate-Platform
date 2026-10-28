//! A local HTTP server for the tests, on a `TcpListener` of the standard library.

mod mock_server;
mod reply;

pub(crate) use mock_server::{MockServer, Request};
pub(crate) use reply::Reply;
