//! A local HTTP server for the tests, on a `TcpListener` of the standard library.

mod mock_server;
mod reply;
mod request;

pub(crate) use mock_server::MockServer;
pub(crate) use reply::Reply;
pub(crate) use request::Request;
