//! [PixState] functions for the [PixEngine] and [AppState].

use crate::{
    prelude::*,
    renderer::{Error as RendererError, Renderer, WindowRenderer},
};
use environment::Environment;
use settings::Settings;
use std::{
    borrow::Cow,
    collections::{HashMap, HashSet},
    error, fmt, io,
    time::Instant,
};

pub mod environment;
pub mod settings;

/// Represents all state and methods for updating and interacting with the [PixEngine].
#[non_exhaustive]
#[derive(Debug)]
pub struct PixState {
    pub(crate) renderer: Renderer,
    pub(crate) env: Environment,
    pub(crate) settings: Settings,
    pub(crate) mouse: MouseState,
    pub(crate) pmouse: MouseState,
    pub(crate) keys: KeyState,
    pub(crate) setting_stack: Vec<Settings>,
}

impl PixState {
    /// Get the current window title.
    #[inline]
    pub fn title(&self) -> &str {
        self.renderer.title()
    }

    /// Set the current window title.
    #[inline]
    pub fn set_title<S: AsRef<str>>(&mut self, title: S) -> PixResult<()> {
        Ok(self.renderer.set_title(title.as_ref())?)
    }

    /// Returns the current mouse position coordinates as `(x, y)`.
    #[inline]
    pub fn mouse_pos(&self) -> PointI2 {
        self.mouse.pos
    }

    /// Returns the previous mouse position coordinates last frame as `(x, y)`.
    #[inline]
    pub fn pmouse_pos(&self) -> PointI2 {
        self.pmouse.pos
    }

    /// Returns if any [Mouse] button is currently being held.
    #[inline]
    pub fn mouse_pressed(&self) -> bool {
        self.mouse.is_pressed()
    }

    /// Returns if a specific [Mouse] button is currently being held.
    #[inline]
    pub fn mouse_down(&self, btn: Mouse) -> bool {
        self.mouse.is_down(btn)
    }

    /// Returns the a list of the current mouse buttons being held.
    #[inline]
    pub fn mouse_buttons(&self) -> &HashSet<Mouse> {
        &self.mouse.pressed
    }

    /// Returns the a list of the current keys being held.
    #[inline]
    pub fn keys(&self) -> &HashSet<Key> {
        &self.keys.pressed
    }

    /// Returns if any [Key] is currently being held.
    #[inline]
    pub fn key_pressed(&self) -> bool {
        self.keys.is_pressed()
    }

    /// Returns if a specific [Key] is currently being held.
    #[inline]
    pub fn key_down(&self, key: Key) -> bool {
        self.keys.is_down(key)
    }
}

impl PixState {
    /// Constructs `PixState` with a given `Renderer`.
    #[inline]
    pub(crate) fn new(renderer: Renderer) -> Self {
        Self {
            renderer,
            env: Environment::default(),
            settings: Settings::default(),
            mouse: MouseState::default(),
            pmouse: MouseState::default(),
            keys: KeyState::default(),
            setting_stack: Vec::new(),
        }
    }

    /// Handle state changes this frame prior to calling [AppState::on_update].
    #[inline]
    pub(crate) fn pre_update(&mut self) {
        self.renderer
            .cursor(self.settings.cursor.as_ref())
            .expect("valid cursor");
    }

    /// Handle state changes this frame after calling [AppState::on_update].
    #[inline]
    pub(crate) fn post_update(&mut self) {
        self.mouse.clear();
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub(crate) struct MouseState {
    pos: PointI2,
    pressed: HashSet<Mouse>,
    clicked: HashSet<Mouse>,
    last_clicked: HashMap<Mouse, Instant>,
}

impl MouseState {
    /// Clear transient [Mouse] state.
    #[inline]
    pub(crate) fn clear(&mut self) {
        self.clicked.clear();
    }

    /// Current [Mouse] position.
    #[inline]
    pub(crate) fn pos(&self) -> PointI2 {
        self.pos
    }

    /// Set current [Mouse] position.
    #[inline]
    pub(crate) fn set_pos(&mut self, pos: PointI2) {
        self.pos = pos;
    }

    /// Whether any [Mouse] buttons are pressed.
    #[inline]
    pub(crate) fn is_pressed(&self) -> bool {
        !self.pressed.is_empty()
    }

    /// Returns if a specific [Mouse] button is currently being held.
    #[inline]
    pub(crate) fn is_down(&self, btn: Mouse) -> bool {
        self.pressed.contains(&btn)
    }

    /// Store a pressed [Mouse] button.
    #[inline]
    pub(crate) fn press(&mut self, btn: Mouse) {
        self.pressed.insert(btn);
    }

    /// Remove a pressed [Mouse] button.
    #[inline]
    pub(crate) fn release(&mut self, btn: &Mouse) {
        self.pressed.remove(btn);
    }

    /// Store last time a [Mouse] button was clicked.
    #[inline]
    pub(crate) fn click(&mut self, btn: Mouse, time: Instant) {
        self.clicked.insert(btn);
        self.last_clicked.insert(btn, time);
    }

    /// Returns if [Mouse] button was clicked last frame.
    #[inline]
    pub(crate) fn was_clicked(&self, btn: &Mouse) -> bool {
        self.clicked.contains(btn)
    }

    /// Returns last time a [Mouse] button was clicked.
    #[inline]
    pub(crate) fn last_clicked(&self, btn: &Mouse) -> Option<&Instant> {
        self.last_clicked.get(btn)
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub(crate) struct KeyState {
    pressed: HashSet<Key>,
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

    /// Store a pressed [Key].
    #[inline]
    pub(crate) fn press(&mut self, key: Key) {
        self.pressed.insert(key);
    }

    /// Remove a pressed [Key].
    #[inline]
    pub(crate) fn release(&mut self, key: &Key) {
        self.pressed.remove(key);
    }
}

/// The error type for [PixState] operations.
#[non_exhaustive]
#[derive(Debug)]
pub enum Error {
    /// IO specific errors.
    IoError(io::Error),
    /// Renderer specific errors.
    RendererError(RendererError),
    /// Unknown errors.
    Other(Cow<'static, str>),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Error::*;
        match self {
            Other(err) => write!(f, "image error: {}", err),
            err => err.fmt(f),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        use Error::*;
        match self {
            IoError(err) => err.source(),
            RendererError(err) => err.source(),
            _ => None,
        }
    }
}

impl From<Error> for PixError {
    fn from(err: Error) -> Self {
        Self::StateError(err)
    }
}