// Config block: local bounded caches use a deterministic hasher. Creating
// std::RandomState accesses compiler static TLS, unavailable in the current
// manual-map runtime (a dynamic TlsAlloc slot is not a PE static TLS slot).
use std::collections::hash_map::DefaultHasher;
use std::hash::BuildHasherDefault;

pub type HashMap<K, V> = std::collections::HashMap<K, V, BuildHasherDefault<DefaultHasher>>;
pub type HashSet<K> = std::collections::HashSet<K, BuildHasherDefault<DefaultHasher>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_preserve_profile_format_and_sets_deduplicate() {
        let mut map = HashMap::default();
        map.insert(7i16, (1usize, 0.12f32, 421i32, true));
        let json = serde_json::to_vec(&map).unwrap();
        let restored: HashMap<i16, (usize, f32, i32, bool)> = serde_json::from_slice(&json).unwrap();
        assert_eq!(restored, map);
        let mut set = HashSet::default();
        assert!(set.insert("popup"));
        assert!(!set.insert("popup"));
    }
}
