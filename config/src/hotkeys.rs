// Config block: no numeric constants; VK_INSERT (0x2D) comes from
// MenuSettings::default() when an invalid (>0xFF) key code is finalized.

use crate::Settings;

/// Edge state of a key after an update.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KeyState {
    Up,
    Pressed,
    Down,
    Released,
}

/// Tracks the previous state of one virtual key code.
#[derive(Clone, Debug)]
pub struct Keybind {
    pub key: u32,
    prev: bool,
}

impl Keybind {
    /// Creates a released keybind for `key`.
    pub fn new(key: u32) -> Self {
        Self { key, prev: false }
    }

    /// Feeds the current physical state and returns the resulting edge.
    pub fn update(&mut self, down: bool) -> KeyState {
        let state = match (self.prev, down) {
            (false, false) => KeyState::Up,
            (false, true) => KeyState::Pressed,
            (true, true) => KeyState::Down,
            (true, false) => KeyState::Released,
        };
        self.prev = down;
        state
    }

    /// True while the key is held down.
    pub fn is_down(&self) -> bool {
        self.prev
    }
}

/// Resets invalid (>0xFF) key codes in `settings` back to their defaults.
pub fn finalize_binds(settings: &mut Settings) {
    let defaults = Settings::default();
    if settings.menu.toggle_key > 0xFF {
        settings.menu.toggle_key = defaults.menu.toggle_key;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn key_edges() {
        let mut bind = Keybind::new(0x2D);
        assert_eq!(bind.update(false), KeyState::Up);
        assert_eq!(bind.update(true), KeyState::Pressed);
        assert!(bind.is_down());
        assert_eq!(bind.update(true), KeyState::Down);
        assert_eq!(bind.update(false), KeyState::Released);
        assert!(!bind.is_down());
    }

    #[test]
    fn invalid_binds_are_reset() {
        let mut settings = Settings::default();
        settings.menu.toggle_key = 0x1234;
        finalize_binds(&mut settings);
        assert_eq!(settings.menu.toggle_key, Settings::default().menu.toggle_key);
    }
}
