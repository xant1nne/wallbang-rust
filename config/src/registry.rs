// Config block: no numeric constants; the registry flattens Settings defaults into
// FNV-1a-hashed fields keyed by dotted names ("a.b.c").

use std::collections::BTreeMap;
use std::sync::OnceLock;

use parking_lot::RwLock;
use serde_json::{Map, Value};

use crate::Settings;

static GLOBAL_REGISTRY: OnceLock<RwLock<FieldRegistry>> = OnceLock::new();

/// Lazily-created, thread-safe registry shared by menu and feature code.
pub fn global_registry() -> &'static RwLock<FieldRegistry> {
    GLOBAL_REGISTRY.get_or_init(|| RwLock::new(FieldRegistry::new()))
}

/// A single flattened configuration field with its hashed key and default value.
#[derive(Clone, Debug)]
pub struct Field {
    pub key: u32,
    pub name: String,
    pub default_json: Value,
}

/// Registry of every configuration field, indexed by its FNV-1a key.
#[derive(Clone, Debug)]
pub struct FieldRegistry {
    fields: BTreeMap<u32, Field>,
}

impl Default for FieldRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl FieldRegistry {
    /// Builds a registry containing every leaf of `Settings::default()`.
    pub fn new() -> Self {
        let mut registry = Self {
            fields: BTreeMap::new(),
        };
        let defaults = serde_json::to_value(Settings::default())
            .expect("Settings must always be serializable");
        flatten("", &defaults, &mut registry);
        registry
    }

    /// Registers (or replaces) a field under its FNV-1a key.
    pub fn register(&mut self, name: &str, default: Value) {
        let key = fnv1a(name);
        self.fields.insert(
            key,
            Field {
                key,
                name: name.to_owned(),
                default_json: default,
            },
        );
    }

    /// Returns all defaults as a JSON object.
    ///
    /// Keys are the eight-digit lowercase hexadecimal field hash (e.g. `a1b2c3d4`,
    /// matching the original C++ `%08x` lookup) and the dotted field name
    /// (e.g. `combat.fov`) for readability. Both map to the same default value.
    pub fn defaults(&self) -> Value {
        let mut map = Map::new();
        for (key, field) in &self.fields {
            let value = field.default_json.clone();
            map.insert(format!("{key:08x}"), value.clone());
            map.insert(field.name.clone(), value);
        }
        Value::Object(map)
    }

    /// Looks up a field by its FNV-1a key.
    pub fn get(&self, key: u32) -> Option<&Field> {
        self.fields.get(&key)
    }

    /// Number of registered fields.
    pub fn len(&self) -> usize {
        self.fields.len()
    }

    /// True when no fields are registered.
    pub fn is_empty(&self) -> bool {
        self.fields.is_empty()
    }
}

/// Deterministic 32-bit FNV-1a hash.
pub fn fnv1a(s: &str) -> u32 {
    let mut hash = 0x811c_9dc5u32;
    for byte in s.as_bytes() {
        hash ^= u32::from(*byte);
        hash = hash.wrapping_mul(0x0100_0193);
    }
    hash
}

/// Recursively expands JSON objects and arrays into dotted leaf names.
fn flatten(prefix: &str, value: &Value, registry: &mut FieldRegistry) {
    match value {
        Value::Object(map) => {
            for (name, child) in map {
                let full_name = if prefix.is_empty() {
                    name.clone()
                } else {
                    format!("{prefix}.{name}")
                };
                flatten(&full_name, child, registry);
            }
        }
        Value::Array(items) => {
            for (index, child) in items.iter().enumerate() {
                flatten(&format!("{prefix}.{index}"), child, registry);
            }
        }
        _ => registry.register(prefix, value.clone()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_covers_all_settings_leaves() {
        let registry = FieldRegistry::new();
        assert!(!registry.is_empty());
        assert!(registry.get(fnv1a("combat.ragebot.groups.0.max_fov")).is_some());
        assert!(registry.get(fnv1a("misc.allow_writes")).is_some());
        assert!(registry.get(fnv1a("misc.motion_blur.enabled")).is_some());
        assert!(registry.get(fnv1a("misc.motion_blur.mode")).is_some());
        assert!(registry.get(fnv1a("visual.chams.enemy.primary.material")).is_some());
        assert!(registry.get(fnv1a("ui.hit_logs")).is_some());
        assert!(registry.get(fnv1a("esp.box")).is_some());
        assert!(registry.get(fnv1a("esp.weapon")).is_some());
        assert!(registry.get(fnv1a("esp.ammo")).is_some());
        assert!(registry.get(fnv1a("esp.r#box")).is_none());
    }

    #[test]
    fn fnv1a_known_vectors() {
        assert_eq!(fnv1a(""), 0x811c9dc5);
        assert_eq!(fnv1a("a"), 0xe40c292c);
        assert_eq!(fnv1a("foobar"), 0xbf9cf968);
    }
}
