//! The tool runner over real processes: shell scripts stand in for rustup and
//! cargo, in a folder of their own.
#![cfg(unix)]

mod bootstrap;
mod control;
mod running;
mod support;
