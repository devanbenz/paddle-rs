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
    leader_id: u64,
    prev_log_index: u64,
    prev_log_term: u64,
    leader_commit: u64,
    entries: Vec<LogEntry>,
}

impl ApppendEntries {
    pub fn new() -> Self {
        Self {
            term: 0,
            leader_id: 0,
            prev_log_index: 0,
            prev_log_term: 0,
            leader_commit: 0,
            entries: vec![],
        }
    }

    pub fn serialize(&self) {
        let mut builder = ::capnp::message::Builder::new_default();
        let mut append_entries = builder.init_root::<append_entries_request::Builder>();
        append_entries.set_term(self.term);
        append_entries.set_leader_id(self.leader_id);
        append_entries.set_prev_log_index(self.prev_log_index);
        append_entries.set_prev_log_term(self.prev_log_term);
        append_entries.set_leader_commit(self.leader_commit);
        let mut entries = append_entries.init_entries(self.entries.len() as u32);

        for (i, entry) in self.entries.iter().enumerate() {
            let mut v = entries
                .reborrow()
                .get(i as u32)
                .expect("get entries buffer");
            let s = match str::from_utf8(&entry.command) {
                Ok(v) => v,
                Err(e) => panic!("Invalid UTF-8: {}", e),
            };

            v.push_str(s);
        }
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
