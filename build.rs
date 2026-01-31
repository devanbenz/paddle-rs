fn main() {
    capnpc::CompilerCommand::new()
        .src_prefix("schema")
        .file("schema/append_entries.capnp")
        .run()
        .expect("schema compiler command");
}
