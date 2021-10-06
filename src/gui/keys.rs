//! Keyboard state management.

use crate::prelude::*;
use std::collections::HashSet;

/// Keep track of key states between frames
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub(crate) struct KeyState {
    pub(crate) entered: Option<Key>,
    pub(crate) pressed: HashSet<Key>,
    pub(crate) mods_pressed: HashSet<KeyMod>,
}

impl KeyState {
    /// Returns if any [Key] is currently being held.
    #[inline]
    pub(crate) fn is_pressed(&self) -> bool {
        !self.pressed.is_empty()
    }

    /// Returns if a specific [Key] is currently being held.
    #[inline]
    pub(crate) fn is_down(&self, key: Key) -> bool {
        self.pressed.contains(&key)
    }

    /// Returns if a [Key] was entered last frame.
    #[inline]
    pub(crate) fn was_entered(&self, key: Key) -> bool {
        matches!(self.entered, Some(k) if k == key)
    }

    /// Returns if a specific [KeyMod] is currently being held.
    #[inline]
    pub(crate) fn mod_down(&self, keymod: KeyMod) -> bool {
        self.mods_pressed.contains(&keymod)
    }

    /// Store a pressed [Key].
    #[inline]
    pub(crate) fn press(&mut self, key: Key, keymod: KeyMod) {
        self.entered = Some(key);
        self.pressed.insert(key);
        self.mods_pressed.insert(keymod);
    }

    /// Remove a pressed [Key].
    #[inline]
    pub(crate) fn release(&mut self, key: Key, keymod: KeyMod) {
        self.pressed.remove(&key);
        self.mods_pressed.remove(&keymod);
    }
}
