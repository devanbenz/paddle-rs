// TODO: Remove allow dead_code
#![allow(dead_code)]
capnp::generated_code!(mod append_entries_capnp);
use std::io::{BufReader, BufWriter};

use append_entries_capnp::append_entries_request;
use capnp::message::ReaderOptions;
use capnp::serialize_packed;

#[derive(Default, Debug)]
pub(crate) struct LogEntry {
    term: u64,
    command: String,
}

impl LogEntry {
    pub(crate) fn new(term: u64, command: String) -> Self {
        Self { term, command }
    }
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

#[derive(Debug, Default)]
pub(crate) struct ApppendEntries {
    term: u64,
    leader_id: u64,
    prev_log_index: u64,
    prev_log_term: u64,
    leader_commit: u64,
    entries: Vec<LogEntry>,
}

impl ApppendEntries {
    pub fn new(
        term: u64,
        leader_id: u64,
        prev_log_index: u64,
        prev_log_term: u64,
        leader_commit: u64,
        entries: Vec<LogEntry>,
    ) -> Self {
        Self {
            term,
            leader_id,
            prev_log_index,
            prev_log_term,
            leader_commit,
            entries,
        }
    }

    pub fn serialize(&self, buffer: &mut [u8]) -> capnp::Result<()> {
        let mut builder = ::capnp::message::Builder::new_default();
        let mut append_entries = builder.init_root::<append_entries_request::Builder>();
        append_entries.set_term(self.term);
        append_entries.set_leader_id(self.leader_id);
        append_entries.set_prev_log_index(self.prev_log_index);
        append_entries.set_prev_log_term(self.prev_log_term);
        append_entries.set_leader_commit(self.leader_commit);
        let mut entries = append_entries.init_entries(self.entries.len() as u32);

        for (i, entry) in self.entries.iter().enumerate() {
            let mut new_entry = entries.reborrow().get(i as u32);
            new_entry.set_term(entry.term);
            new_entry.set_command(entry.command.as_str());
        }

        let writer = BufWriter::new(buffer);
        serialize_packed::write_message(writer, &builder)?;
        Ok(())
    }

    pub fn deserialize(&mut self, buffer: &[u8]) -> capnp::Result<()> {
        let reader = BufReader::new(buffer);
        let messsage = serialize_packed::read_message(reader, ReaderOptions::default())?;
        assert!(messsage.size_in_words() > 0);
        let data = messsage.get_root::<append_entries_request::Reader>()?;
        self.term = data.get_term();
        self.leader_id = data.get_leader_id();
        self.prev_log_index = data.get_prev_log_index();
        self.prev_log_term = data.get_prev_log_term();
        self.leader_commit = data.get_leader_commit();

        for entry in data.get_entries()? {
            let entry_string = entry.get_command()?.to_string()?;
            let entry_term = entry.get_term();
            self.entries.push(LogEntry::new(entry_term, entry_string));
        }

        Ok(())
    }
}

#[derive(Debug)]
pub(crate) struct RequestVote {}

#[derive(Debug, Default, PartialEq)]
pub(crate) enum NodeState {
    #[default]
    Follower,
    Leader,
    Candidate,
}

#[derive(Default, Debug)]
pub struct Node {
    log: Log,
    state: NodeState,
    append_entries_buffer: Vec<ApppendEntries>,
    request_vote_buffer: Vec<RequestVote>,
}

impl Node {
    pub fn new() -> Self {
        Self {
            log: Log::default(),
            state: NodeState::default(),
            append_entries_buffer: vec![],
            request_vote_buffer: vec![],
        }
    }

    pub fn receive_message(&self, _buffer: &[u8]) -> capnp::Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn append_entries_test() -> capnp::Result<()> {
        let entry = ApppendEntries::new(
            4,
            2,
            3,
            2,
            3,
            vec![
                LogEntry::new(0, String::from("hello")),
                LogEntry::new(1, String::from("tadashi & mizu")),
            ],
        );

        let mut serialized_buffer = [0; 1024];
        entry.serialize(&mut serialized_buffer)?;

        let mut entry2 = ApppendEntries::default();
        entry2.deserialize(&serialized_buffer)?;

        assert_eq!(entry.term, entry2.term);
        assert_eq!(entry.leader_id, entry2.leader_id);
        assert_eq!(entry.prev_log_term, entry2.prev_log_term);
        assert_eq!(entry.prev_log_index, entry2.prev_log_index);
        assert_eq!(entry.leader_commit, entry2.leader_commit);

        for (i, e) in entry.entries.iter().enumerate() {
            assert_eq!(e.term, entry2.entries[i].term);
            assert_eq!(e.command, entry2.entries[i].command);
        }

        Ok(())
    }

    #[test]
    fn node_test() {
        let new_node = Node::new();
        assert_eq!(new_node.state, NodeState::Follower);
    }
}
