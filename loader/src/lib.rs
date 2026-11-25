// Config block: library facade for the loader binary. The GUI loader in
// `src/bin/wallbang_gui.rs` is the only binary; these modules implement the PE
// manual-map pipeline it runs.
pub mod cleanup;
pub mod entry_call;
pub mod find_process;
pub mod loadlib;
pub mod map;
