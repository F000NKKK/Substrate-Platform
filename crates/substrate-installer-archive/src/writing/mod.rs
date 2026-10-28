//! Putting entries on disk: where each lands, and the checks that keep it
//! inside the destination.

mod copy;
mod modes;
mod placement;
mod sink;
mod write_failed;

pub(crate) use sink::Sink;
