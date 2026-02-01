@0xc366562dd5c738b8;

struct AppendEntriesRequest {
  term @0 :UInt64;
  leaderId @1 :UInt64;
  prevLogIndex @2 :UInt64;
  prevLogTerm @3 :UInt64;
  leaderCommit @4 :UInt64;
  entries @5 :List(LogEntry);

  struct LogEntry {
    term @0 :UInt64;
    command @1 :Text;
  }
 }
