// TODO: Remove allow dead_code
#![allow(dead_code)]
capnp::generated_code!(mod append_entries_capnp);
use append_entries_capnp::append_entries_request;

#[derive(Default, Debug)]
pub(crate) struct LogEntry {
    term: u64,
    command: Vec<u8>,
}

#[derive(Default, Debug)]
pub(crate) struct Log {
    entries: Vec<LogEntry>,
}

impl Log {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub(crate) struct ApppendEntries {
    term: u64,
}

impl ApppendEntries {
    pub fn new() -> Self {
        Self {}
    }

    pub fn serialize(&self) {
        let mut builder = ::capnp::message::Builder::new_default();
        let append_entries = builder.init_root::<append_entries_request::Builder>();
        append_entries.set_term(value);
        append_entries.init_entries()
    }
}

#[derive(Debug)]
pub(crate) struct RequestVote {}

#[derive(Default, Debug)]
pub(crate) struct Node {
    log: Log,
    append_entries_buffer: Vec<ApppendEntries>,
    request_vote_buffer: Vec<RequestVote>,
}

impl Node {
    pub fn new() -> Self {
        Self {
            log: Log::default(),
            append_entries_buffer: vec![],
            request_vote_buffer: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn node_test() {}
}
