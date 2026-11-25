// Config block:
// MAX_SCHEDULED = 16384
// MAX_DATABASE_KEYS = 65536
// MAX_DATABASE_DEPTH = 16
// MAX_DATABASE_BYTES = 16 * 1024 * 1024

/// Maximum number of pending `time.Schedule` calls.
pub const MAX_SCHEDULED: usize = 16384;
/// Maximum number of keys in `database`.
pub const MAX_DATABASE_KEYS: usize = 65536;
/// Maximum nesting depth of a database value.
pub const MAX_DATABASE_DEPTH: usize = 16;
/// Maximum total size of all database keys and values, in bytes.
pub const MAX_DATABASE_BYTES: usize = 16 * 1024 * 1024;

pub mod api;
pub mod bindings;
pub mod events;
pub mod runtime;

pub use runtime::Runtime;
