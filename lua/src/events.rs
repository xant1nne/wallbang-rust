// Config block:
// MAX_SCHEDULED = 16384
// MAX_DATABASE_KEYS = 65536
// MAX_DATABASE_DEPTH = 16
// MAX_DATABASE_BYTES = 16 * 1024 * 1024

use wallbang_config::collections::HashMap;

use mlua::RegistryKey;
use parking_lot::RwLock;

/// Events fired by the cheat itself. Any other name is a custom event.
pub const KNOWN_EVENTS: &[&str] = &[
    "render",
    "input_poll",
    "pre_movement",
    "post_movement",
    "createmove",
    "shutdown",
    "player_sound",
];

/// Thread-safe storage of Lua callbacks grouped by event name.
pub struct EventBus {
    callbacks: RwLock<HashMap<String, Vec<RegistryKey>>>,
}

impl EventBus {
    pub fn new() -> Self {
        Self {
            callbacks: RwLock::new(HashMap::default()),
        }
    }

    pub fn subscribe(&self, name: &str, key: RegistryKey) {
        let mut callbacks = self.callbacks.write();
        let list = callbacks.entry(name.to_owned()).or_default();
        if !list.contains(&key) {
            list.push(key);
        }
    }

    pub fn unsubscribe(&self, name: &str, key: RegistryKey) {
        let mut callbacks = self.callbacks.write();
        if let Some(list) = callbacks.get_mut(name) {
            list.retain(|current| current != &key);
            if list.is_empty() {
                callbacks.remove(name);
            }
        }
    }

    /// Removes all callbacks of the event and returns their registry keys.
    pub fn take(&self, name: &str) -> Vec<RegistryKey> {
        self.callbacks.write().remove(name).unwrap_or_default()
    }

    pub fn clear(&self) {
        self.callbacks.write().clear();
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}
