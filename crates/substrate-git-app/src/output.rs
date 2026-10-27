//! Where a long run's lines go.

use substrate_message::Message;

/// Where a long run's lines go (the host's Output window): git's own as
/// they are ([`Message::verbatim`]), ours to translate.
pub type Output<'a> = &'a (dyn Fn(Message) + Sync);
