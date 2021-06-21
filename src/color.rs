//! [`Color`] functions for drawing.
//!
//! Each [`Color`] can be constructed with a [`ColorMode`]. The default mode and internal representation
//! is [`Rgb`] with values ranging from `0..=255` for red, green, blue, and alpha transparency. [`Hsb`] and [`Hsl`]
//! values range from `0.0..=360.0` for hue, `0.0..=100.0` for saturation and brightness/lightness
//! and `0.0..=1.0` for alpha transparency.
//!
//! There are convience macros for flexible construction: [`color!`], [`rgb!`], [`hsb!`] and [`hsl!`] that take 1-4
//! parameters. The number of parameters provided alter how they are interpreted:
//!
//! - Providing a single parameter constructs a grayscale color.
//! - Two parameters constructs a grayscale color with alpha transparency.
//! - Three parameters are used as RGB or HSB/HSL values.
//! - Four parameters are used as RGBB or HSb/HSL values with alpha transparency.
//!
//! If you're not picky about color, there are the [`random`](Color::random) and
//! [`random_alpha`](Color::random_alpha) methods.
//!
//! [`Color`] also implements [`FromStr`](std::str::FromStr) allowing conversion from a 3, 4, 6, or 8-digit hexidecimal
//! string.
//!
//! The [`Color`] instance stores which [`ColorMode`] it was created with, modifying how manipulation
//! methods are interprted such as [`set_alpha`](Color::set_alpha) taking a range of `0.0..=255.0` or `0.0..=1.0`.
//! The `ColorMode` can be changed any time to alter this behavior using [`set_mode`](Color::set_mode).
//!
//! There are also several named color [`constants`] available in the
//! [`prelude`](crate::prelude) matching the [SVG 1.0 Color
//! Keywords](https://www.w3.org/TR/SVG11/types.html#ColorKeywords).
//!
//! # Examples
//!
//! `Rgb` values range from `0..=255` for red, green, blue and alpha transparency.
//!
//! ```
//! use pix_engine::prelude::*;
//!
//! let c = rgb!(55); // Grayscale
//! assert_eq!(c.channels(), [55, 55, 55, 255]);
//!
//! let c = rgb!(55, 128); // Grayscale with alpha
//! assert_eq!(c.channels(), [55, 55, 55, 128]);
//!
//! let c = rgb!(128, 0, 55); // Red, Green, Blue
//! assert_eq!(c.channels(), [128, 0, 55, 255]);
//!
//! let c = rgb!(128, 0, 55, 128); // Red, Green, Blue, and Alpha
//! assert_eq!(c.channels(), [128, 0, 55, 128]);
//! ```
//!
//! `Hsb`/`Hsl` values range from `0.0..=360.0` for hue, `0.0..=100.0` for saturation and
//! brightness/lightness and `0.0..=1.0` for alpha transparency.
//!
//! ```
//! use pix_engine::prelude::*;
//!
//! let c = hsb!(50.0); // Gray
//! assert_eq!(c.channels(), [128, 128, 128, 255]);
//!
//! let c = hsb!(50.0, 0.5); // Gray with alpha
//! assert_eq!(c.channels(), [128, 128, 128, 128]);
//!
//! let c = hsb!(342.0, 100.0, 80.0); // Hue, Saturation, Brightness
//! assert_eq!(c.channels(), [204, 0, 61, 255]);
//!
//! let c = hsb!(342.0, 100.0, 80.0, 0.5); // Hue, Saturation, Brightness, Alpha
//! assert_eq!(c.channels(), [204, 0, 61, 128]);
//! ```
//!
//! # SVG 1.0 Named color constants
//!
//! ```
//! use pix_engine::prelude::*;
//!
//! let c = ALICE_BLUE;
//! assert_eq!(c.channels(), [240, 248, 255, 255]);
//!
//! let c = DARK_ORCHID;
//! assert_eq!(c.channels(), [153, 50, 204, 255]);
//! ```
//!
//! # From a hexidecimal string
//!
//! ```
//! use pix_engine::prelude::*;
//! use std::str::FromStr;
//!
//! let c = Color::from_str("#F0F")?; // 3-digit Hex string
//! assert_eq!(c.channels(), [255, 0, 255, 255]);
//!
//! let c = Color::from_str("#F0F5")?; // 4-digit Hex string
//! assert_eq!(c.channels(), [255, 0, 255, 85]);
//!
//! let c = Color::from_str("#F0F5BF")?; // 6-digit Hex string
//! assert_eq!(c.channels(), [240, 245, 191, 255]);
//!
//! let c = Color::from_str("#F0F5BF5F")?; // 8-digit Hex string
//! assert_eq!(c.channels(), [240, 245, 191, 95]);
//! # Ok::<(), ColorError<f64>>(())
//! ```

use crate::random;
use conversion::{calculate_channels, convert_levels, maxes, ColorError};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{
    array::IntoIter,
    borrow::Cow,
    fmt::{self, LowerHex, UpperHex},
    iter::FromIterator,
    ops::*,
};

pub mod constants;
pub mod conversion;

/// [`Color`] mode indicating level interpretation.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ColorMode {
    /// Red, Green, Blue, and Alpha
    Rgb,
    /// Hue, Saturation, Brightness, and Alpha
    Hsb,
    /// Hue, Saturation, Lightness, and Alpha
    Hsl,
}

use ColorMode::*;

/// A color represented with a [`ColorMode`].
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Color {
    /// `Color` mode.
    mode: ColorMode,
    /// RGB values ranging `0.0..=1.0`.
    levels: [f64; 4],
    /// RGB values ranging from `0..=255`.
    channels: [u8; 4],
}

impl Color {
    /// Constructs a [`Rgb`] `Color` with max alpha.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let c = Color::new(0, 0, 128);
    /// assert_eq!(c.channels(), [0, 0, 128, 255]);
    /// ```
    pub fn new<T: Into<f64>>(r: T, g: T, b: T) -> Self {
        Self::rgb(r, g, b)
    }

    /// Constructs a [`Rgb`] `Color` with alpha.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let c = Color::new_alpha(0, 0, 128, 50);
    /// assert_eq!(c.channels(), [0, 0, 128, 50]);
    /// ```
    pub fn new_alpha<T: Into<f64>>(r: T, g: T, b: T, a: T) -> Self {
        Self::rgba(r, g, b, a)
    }

    /// Constructs a `Color` with the given [`ColorMode`] and max alpha.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let c = Color::with_mode(ColorMode::Rgb, 0.0, 0.0, 128.0);
    /// assert_eq!(c.channels(), [0, 0, 128, 255]);
    ///
    /// let c = Color::with_mode(ColorMode::Hsb, 126.0, 50.0, 100.0);
    /// assert_eq!(c.channels(), [128, 255, 140, 255]);
    /// ```
    pub fn with_mode<T: Into<f64>>(mode: ColorMode, v1: T, v2: T, v3: T) -> Self {
        // Normalize channels
        let [v1_max, v2_max, v3_max, _] = maxes(mode);
        let levels = [
            (v1.into() / v1_max).clamp(0.0, 1.0),
            (v2.into() / v2_max).clamp(0.0, 1.0),
            (v3.into() / v3_max).clamp(0.0, 1.0),
            1.0,
        ];

        // Convert to `Rgb`
        let levels = convert_levels(levels, mode, Rgb);
        let channels = calculate_channels(levels);

        Self {
            mode,
            levels,
            channels,
        }
    }

    /// Constructs a `Color` with the given [`ColorMode`] and alpha.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let c = Color::with_mode_alpha(ColorMode::Rgb, 0.0, 0.0, 128.0, 50.0);
    /// assert_eq!(c.channels(), [0, 0, 128, 50]);
    ///
    /// let c = Color::with_mode_alpha(ColorMode::Hsb, 126.0, 50.0, 100.0, 0.8);
    /// assert_eq!(c.channels(), [128, 255, 140, 204]);
    /// ```
    pub fn with_mode_alpha<T: Into<f64>>(mode: ColorMode, v1: T, v2: T, v3: T, alpha: T) -> Self {
        // Normalize channels
        let [v1_max, v2_max, v3_max, alpha_max] = maxes(mode);
        let levels = [
            (v1.into() / v1_max).clamp(0.0, 1.0),
            (v2.into() / v2_max).clamp(0.0, 1.0),
            (v3.into() / v3_max).clamp(0.0, 1.0),
            (alpha.into() / alpha_max).clamp(0.0, 1.0),
        ];

        // Convert to `Rgb`
        let levels = convert_levels(levels, mode, Rgb);
        let channels = calculate_channels(levels);

        Self {
            mode,
            levels,
            channels,
        }
    }

    /// Constructs a [`Rgb`] `Color` containing red, green, and blue with alpha of
    /// `255`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let c = Color::rgb(128, 64, 0);
    /// assert_eq!(c.channels(), [128, 64, 0, 255]);
    /// ```
    pub fn rgb<T: Into<f64>>(r: T, g: T, b: T) -> Self {
        Self::with_mode(Rgb, r, g, b)
    }

    /// Constructs a [`Rgb`] `Color` containing red, green, blue, and alpha.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let c = Color::rgba(128, 64, 128, 128);
    /// assert_eq!(c.channels(), [128, 64, 128, 128]);
    /// ```
    pub fn rgba<T: Into<f64>>(r: T, g: T, b: T, a: T) -> Self {
        Self::with_mode_alpha(Rgb, r, g, b, a)
    }

    /// Constructs a [`Hsb`] `Color` containing hue, saturation, and brightness
    /// with alpha of `1.0`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let c = Color::hsb(126.0, 80.0, 50.0);
    /// assert_eq!(c.channels(), [25, 128, 36, 255]);
    /// ```
    pub fn hsb<T: Into<f64>>(h: T, s: T, b: T) -> Self {
        Self::with_mode(Hsb, h, s, b)
    }

    /// Constructs a [`Hsb`] `Color` containing hue, saturation, brightness and
    /// alpha.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let c = Color::hsba(126.0, 80.0, 50.0, 0.5);
    /// assert_eq!(c.channels(), [25, 128, 36, 128]);
    /// ```
    pub fn hsba<T: Into<f64>>(h: T, s: T, b: T, a: T) -> Self {
        Self::with_mode_alpha(Hsb, h, s, b, a)
    }

    /// Constructs a [`Hsl`] `Color` containing hue, saturation, and lightness
    /// with alpha of `1.0`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let c = Color::hsl(126.0, 80.0, 50.0);
    /// assert_eq!(c.channels(), [25, 230, 46, 255]);
    /// ```
    pub fn hsl<T: Into<f64>>(h: T, s: T, l: T) -> Self {
        Self::with_mode(Hsl, h, s, l)
    }

    /// Constructs a [`Hsl`] `Color` containing hue, saturation, lightness and
    /// alpha.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let c = Color::hsla(126.0, 80.0, 50.0, 0.5);
    /// assert_eq!(c.channels(), [25, 230, 46, 128]);
    /// ```
    pub fn hsla<T: Into<f64>>(h: T, s: T, l: T, a: T) -> Self {
        Self::with_mode_alpha(Hsl, h, s, l, a)
    }

    /// Constructs a raw `Color` with the given [`ColorMode`] and alpha using the levels passed in
    /// as-is without normalizing them.
    ///
    /// # Safety
    ///
    /// This may result in unexpected behavior if values are outside the range `0.0..=1.0`. It is
    /// up to the responsibility of the caller to hold this invariant.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let c = unsafe { Color::from_raw(ColorMode::Rgb, 0.4, 0.5, 1.0, 0.8) };
    /// assert_eq!(c.channels(), [102, 128, 255, 204]);
    /// ```
    pub unsafe fn from_raw<T: Into<f64>>(mode: ColorMode, v1: T, v2: T, v3: T, alpha: T) -> Self {
        let levels = [v1.into(), v2.into(), v3.into(), alpha.into()];
        Self {
            mode,
            levels,
            channels: calculate_channels(levels),
        }
    }

    /// Constructs a `Color` from a [`slice`] of 1-4 values. The number of values
    /// provided alter how they are interpreted similar to the [`color!`], [`rgb!`], [`hsb!`], and
    /// [`hsl!`] macros.
    ///
    /// # Errors
    ///
    /// If the [`slice`] is empty or has more than 4 values, an error is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let vals: Vec<f64> = vec![128.0, 64.0, 0.0];
    /// let c = Color::from_slice(ColorMode::Rgb, &vals)?; // RGB Vec
    /// assert_eq!(c.channels(), [128, 64, 0, 255]);
    ///
    /// let vals: [f64; 4] = [128.0, 64.0, 0.0, 128.0];
    /// let c = Color::from_slice(ColorMode::Rgb, &vals[..])?; // RGBA slice
    /// assert_eq!(c.channels(), [128, 64, 0, 128]);
    /// # Ok::<(), ColorError<f64>>(())
    /// ```
    pub fn from_slice<T: Into<f64>>(mode: ColorMode, slice: &[T]) -> Result<Self, ColorError<T>>
    where
        T: fmt::Debug + Copy + Clone,
    {
        let result = match *slice {
            [gray] => Self::with_mode(mode, gray, gray, gray),
            [gray, a] => Self::with_mode_alpha(mode, gray, gray, gray, a),
            [v1, v2, v3] => Self::with_mode(mode, v1, v2, v3),
            [v1, v2, v3, a] => Self::with_mode_alpha(mode, v1, v2, v3, a),
            _ => return Err(ColorError::InvalidSlice(Cow::from(slice.to_owned()))),
        };
        Ok(result)
    }
    /// Constructs a random [`Rgb`] `Color` with max alpha.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # #[allow(unused_variables)]
    /// let c = Color::random();
    /// // `c.channels()` will return something like:
    /// // [207, 12, 217, 255]
    /// ```
    pub fn random() -> Self {
        Self::new(random!(255), random!(255), random!(255))
    }

    /// Constructs a random [`Rgb`] `Color` with alpha.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # #[allow(unused_variables)]
    /// let c = Color::random_alpha();
    /// // `c.channels()` will return something like:
    /// // [132, 159, 233, 76]
    /// ```
    pub fn random_alpha() -> Self {
        Self::new_alpha(random!(255), random!(255), random!(255), random!(255))
    }

    /// Constructs a [`Rgb`] `Color` from a [`u32`] RGBA hexadecimal value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let c = Color::from_hex(0xF0FF00FF);
    /// assert_eq!(c.channels(), [240, 255, 0, 255]);
    ///
    /// let c = Color::from_hex(0xF0FF0080);
    /// assert_eq!(c.channels(), [240, 255, 0, 128]);
    /// ```
    pub fn from_hex(hex: u32) -> Self {
        let [r, g, b, a] = hex.to_be_bytes();
        Self::rgba(r, g, b, a)
    }

    /// Returns a list of max values for each color channel based on [`ColorMode`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let c = Color::rgb(0, 0, 0);
    /// assert_eq!(c.maxes(), [255.0, 255.0, 255.0, 255.0]);
    ///
    /// let c = Color::hsb(0.0, 0.0, 0.0);
    /// assert_eq!(c.maxes(), [360.0, 100.0, 100.0, 1.0]);
    ///
    /// let c = Color::hsl(0.0, 0.0, 0.0);
    /// assert_eq!(c.maxes(), [360.0, 100.0, 100.0, 1.0]);
    /// ```
    #[inline]
    pub const fn maxes(&self) -> [f64; 4] {
        maxes(self.mode)
    }

    /// Returns the `Color` levels which range from `0.0..=1.0`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let c = Color::rgba(128, 64, 128, 128);
    /// assert_eq!(c.channels(), [128, 64, 128, 128]);
    /// ```
    #[inline]
    pub const fn levels(&self) -> [f64; 4] {
        self.levels
    }

    /// Returns the [`Rgb`] `Color` channels which range from `0..=255`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let c = Color::rgba(128, 64, 128, 128);
    /// assert_eq!(c.channels(), [128, 64, 128, 128]);
    /// ```
    #[inline]
    pub const fn channels(&self) -> [u8; 4] {
        self.channels
    }

    /// Returns the current [`ColorMode`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let c = Color::rgb(100, 0, 0);
    /// assert_eq!(c.mode(), ColorMode::Rgb);
    ///
    /// let c = Color::hsb(100.0, 0.0, 0.0);
    /// assert_eq!(c.mode(), ColorMode::Hsb);
    /// ```
    #[inline]
    pub const fn mode(&self) -> ColorMode {
        self.mode
    }

    /// Set the [`ColorMode`].
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let mut c = Color::rgb(100, 0, 0);
    /// c.set_mode(ColorMode::Hsb);
    /// assert_eq!(c.mode(), ColorMode::Hsb);
    /// ```
    #[inline]
    pub fn set_mode(&mut self, mode: ColorMode) {
        self.mode = mode;
    }

    /// Returns the red `Color` channel ranging from `0..=255`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let c = Color::rgb(100, 0, 0);
    /// assert_eq!(c.red(), 100);
    /// ```
    #[inline]
    pub const fn red(&self) -> u8 {
        self.channels[0]
    }

    /// Set the red `Color` channel ranging from `0..=255`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let mut c = Color::default();
    /// assert_eq!(c.channels(), [0, 0, 0, 255]);
    /// c.set_red(100);
    /// assert_eq!(c.channels(), [100, 0, 0, 255]);
    /// ```
    #[inline]
    pub fn set_red(&mut self, r: impl Into<f64>) {
        let maxes = maxes(Rgb);
        self.levels[0] = r.into() / maxes[0];
        self.calculate_channels();
    }

    /// Returns the green `Color` channel ranging from `0..=255`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let c = Color::rgb(0, 100, 0);
    /// assert_eq!(c.green(), 100);
    /// ```
    #[inline]
    pub const fn green(&self) -> u8 {
        self.channels[1]
    }

    /// Set the green `Color` channel ranging from `0..=255`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let mut c = Color::default();
    /// assert_eq!(c.channels(), [0, 0, 0, 255]);
    /// c.set_green(100);
    /// assert_eq!(c.channels(), [0, 100, 0, 255]);
    /// ```
    #[inline]
    pub fn set_green(&mut self, g: impl Into<f64>) {
        let maxes = maxes(Rgb);
        self.levels[1] = g.into() / maxes[1];
        self.calculate_channels();
    }

    /// Returns the blue `Color` channel ranging from `0..=255`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let c = Color::rgb(0, 0, 100);
    /// assert_eq!(c.blue(), 100);
    /// ```
    #[inline]
    pub const fn blue(&self) -> u8 {
        self.channels[2]
    }

    /// Set the blue `Color` channel ranging from `0..=255`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let mut c = Color::default();
    /// assert_eq!(c.channels(), [0, 0, 0, 255]);
    /// c.set_blue(100);
    /// assert_eq!(c.channels(), [0, 0, 100, 255]);
    /// ```
    #[inline]
    pub fn set_blue(&mut self, b: impl Into<f64>) {
        let maxes = maxes(Rgb);
        self.levels[2] = b.into() / maxes[2];
        self.calculate_channels();
    }

    /// Returns the alpha `Color` channel ranging from `0.0..=255.0` or `0.0..=1.0` depending on
    /// current [`ColorMode`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let c = Color::rgba(0, 0, 0, 100);
    /// assert_eq!(c.alpha(), 100.0);
    ///
    /// let c = Color::hsba(0.0, 0.0, 0.0, 0.8);
    /// assert_eq!(c.alpha(), 0.8);
    /// ```
    #[inline]
    pub fn alpha(&self) -> f64 {
        let maxes = self.maxes();
        self.levels[3] * maxes[3]
    }

    /// Set the alpha `Color` channel ranging from `0..=255` or `0.0..=1.0` depending on current
    /// [`ColorMode`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let mut c = Color::default();
    /// assert_eq!(c.channels(), [0, 0, 0, 255]);
    /// c.set_alpha(100);
    /// assert_eq!(c.channels(), [0, 0, 0, 100]);
    ///
    /// let mut c = Color::hsb(0.0, 0.0, 0.0);
    /// assert_eq!(c.channels(), [0, 0, 0, 255]);
    /// c.set_alpha(0.8);
    /// assert_eq!(c.channels(), [0, 0, 0, 204]);
    /// ```
    #[inline]
    pub fn set_alpha(&mut self, a: impl Into<f64>) {
        let maxes = self.maxes();
        self.levels[3] = a.into() / maxes[3];
        self.calculate_channels();
    }

    /// Returns the hue `Color` channel ranging from `0.0..=360.0`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let c = Color::rgb(0, 100, 0);
    /// assert_eq!(c.hue(), 120.0);
    /// ```
    #[inline]
    pub fn hue(&self) -> f64 {
        let maxes = maxes(Hsb);
        let levels = convert_levels(self.levels, Rgb, Hsb);
        levels[0] * maxes[0]
    }

    /// Set the hue `Color` channel ranging from `0.0..=360.0`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let mut c = Color::rgb(128, 0, 0);
    /// assert_eq!(c.channels(), [128, 0, 0, 255]);
    /// c.set_hue(100.0);
    /// assert_eq!(c.channels(), [43, 128, 0, 255]);
    /// ```
    #[inline]
    pub fn set_hue(&mut self, h: impl Into<f64>) {
        let maxes = maxes(Hsb);
        let mut levels = convert_levels(self.levels, Rgb, Hsb);
        levels[0] = h.into() / maxes[0];
        self.levels = convert_levels(levels, Hsb, Rgb);
        self.calculate_channels();
    }

    /// Returns the saturation `Color` channel ranging from `0.0..=100.0`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let c = Color::rgb(0, 100, 0);
    /// assert_eq!(c.saturation(), 100.0);
    /// ```
    #[inline]
    pub fn saturation(&self) -> f64 {
        let maxes = maxes(Hsb);
        let levels = convert_levels(self.levels, Rgb, Hsb);
        levels[1] * maxes[1]
    }

    /// Set the saturation `Color` channel ranging from `0.0..=100.0`. Defaults to [`Hsb`] if the
    /// current mode is not [`Hsb`] or [`Hsl`] already.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let mut c = Color::rgb(128, 0, 0);
    /// assert_eq!(c.channels(), [128, 0, 0, 255]);
    /// c.set_saturation(50.0);
    /// assert_eq!(c.channels(), [128, 64, 64, 255]);
    ///
    /// let mut c = Color::rgb(128, 0, 0);
    /// c.set_mode(ColorMode::Hsl);
    /// assert_eq!(c.channels(), [128, 0, 0, 255]);
    /// c.set_saturation(50.0);
    /// assert_eq!(c.channels(), [96, 32, 32, 255]);
    /// ```
    #[inline]
    pub fn set_saturation(&mut self, s: impl Into<f64>) {
        let mode = match self.mode {
            Hsb | Hsl => self.mode,
            _ => Hsb,
        };
        let maxes = maxes(mode);
        let mut levels = convert_levels(self.levels, Rgb, mode);
        levels[1] = s.into() / maxes[1];
        self.levels = convert_levels(levels, mode, Rgb);
        self.calculate_channels();
    }

    /// Returns the brightness `Color` channel ranging from `0.0..=100.0`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let c = Color::rgb(0, 102, 0);
    /// assert_eq!(c.brightness(), 40.0);
    /// ```
    #[inline]
    pub fn brightness(&self) -> f64 {
        let maxes = maxes(Hsb);
        let levels = convert_levels(self.levels, Rgb, Hsb);
        levels[2] * maxes[2]
    }

    /// Set the brightness `Color` channel ranging from `0.0..=100.0`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let mut c = Color::rgb(128, 0, 0);
    /// assert_eq!(c.channels(), [128, 0, 0, 255]);
    /// c.set_brightness(90.0);
    /// assert_eq!(c.channels(), [230, 0, 0, 255]);
    /// ```
    #[inline]
    pub fn set_brightness(&mut self, b: impl Into<f64>) {
        let maxes = maxes(Hsb);
        let mut levels = convert_levels(self.levels, Rgb, Hsb);
        levels[2] = b.into() / maxes[2];
        self.levels = convert_levels(levels, Hsb, Rgb);
        self.calculate_channels();
    }

    /// Returns the lightness `Color` channel ranging from `0.0..=100.0`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let c = Color::rgb(0, 102, 0);
    /// assert_eq!(c.lightness(), 20.0);
    /// ```
    #[inline]
    pub fn lightness(&self) -> f64 {
        let maxes = maxes(Hsl);
        let levels = convert_levels(self.levels, Rgb, Hsl);
        levels[2] * maxes[2]
    }

    /// Set the lightness `Color` channel ranging from `0.0..=100.0`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let mut c = Color::rgb(128, 0, 0);
    /// assert_eq!(c.channels(), [128, 0, 0, 255]);
    /// c.set_lightness(90.0);
    /// assert_eq!(c.channels(), [255, 204, 204, 255]);
    /// ```
    #[inline]
    pub fn set_lightness(&mut self, l: impl Into<f64>) {
        let maxes = maxes(Hsl);
        let mut levels = convert_levels(self.levels, Rgb, Hsl);
        levels[2] = l.into() / maxes[2];
        self.levels = convert_levels(levels, Hsl, Rgb);
        self.calculate_channels();
    }

    /// Returns an itereator over the `Color` RGBA channels `[r, g, b, a]`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let c = color!(100, 200, 50);
    /// let mut iterator = c.iter();
    ///
    /// assert_eq!(iterator.next(), Some(100));
    /// assert_eq!(iterator.next(), Some(200));
    /// assert_eq!(iterator.next(), Some(50));
    /// assert_eq!(iterator.next(), Some(255));
    /// assert_eq!(iterator.next(), None);
    /// ```
    pub fn iter(&self) -> Iter {
        Iter::new(self)
    }
}

/// # Constructs a [`Rgb`] [`Color`].
///
/// Alias for [`rgb!`].
#[macro_export]
macro_rules! color {
    ($gray:expr) => {
        rgb!($gray)
    };
    ($gray:expr, $a:expr$(,)?) => {
        rgb!($gray, $a)
    };
    ($r:expr, $g:expr, $b:expr$(,)?) => {
        rgb!($r, $g, $b)
    };
    ($r:expr, $g:expr, $b:expr, $a:expr$(,)?) => {
        rgb!($r, $g, $b, $a)
    };
}

/// # Constructs a [`Rgb`] [`Color`].
///
/// # Examples
///
/// ```
/// use pix_engine::prelude::*;
///
/// let c = rgb!(128); // Gray
/// assert_eq!(c.channels(), [128, 128, 128, 255]);
///
/// let c = rgb!(128, 64); // Gray with alpha
/// assert_eq!(c.channels(), [128, 128, 128, 64]);
///
/// let c = rgb!(128, 64, 0); // Red, Green, Blue
/// assert_eq!(c.channels(), [128, 64, 0, 255]);
///
/// let c = rgb!(128, 64, 128, 128); // Red, Green, Blue, Alpha
/// assert_eq!(c.channels(), [128, 64, 128, 128]);
/// ```
#[macro_export]
macro_rules! rgb {
    ($gray:expr) => {
        rgb!($gray, $gray, $gray)
    };
    ($gray:expr, $a:expr$(,)?) => {
        rgb!($gray, $gray, $gray, $a)
    };
    ($r:expr, $g:expr, $b:expr$(,)?) => {
        rgb!($r, $g, $b, 255)
    };
    ($r:expr, $g:expr, $b:expr, $a:expr$(,)?) => {
        $crate::color::Color::rgba($r, $g, $b, $a)
    };
}

/// # Constructs a [`Hsb`] [`Color`].
///
/// # Examples
///
/// ```
/// use pix_engine::prelude::*;
///
/// let c = hsb!(50.0); // Gray
/// assert_eq!(c.channels(), [128, 128, 128, 255]);
///
/// let c = hsb!(50.0, 0.5); // Gray with alpha
/// assert_eq!(c.channels(), [128, 128, 128, 128]);
///
/// let c = hsb!(342.0, 100.0, 80.0); // Hue, Saturation, Brightness
/// assert_eq!(c.channels(), [204, 0, 61, 255]);
///
/// let c = hsb!(342.0, 100.0, 80.0, 0.5); // Hue, Saturation, Brightness, Alpha
/// assert_eq!(c.channels(), [204, 0, 61, 128]);
/// ```
#[macro_export]
macro_rules! hsb {
    ($gray:expr) => {
        hsb!(0.0, 0.0, $gray)
    };
    ($gray:expr, $a:expr$(,)?) => {
        hsb!(0.0, 0.0, $gray, $a)
    };
    ($h:expr, $s:expr, $b:expr$(,)?) => {
        hsb!($h, $s, $b, 1.0)
    };
    ($h:expr, $s:expr, $b:expr, $a:expr$(,)?) => {
        $crate::color::Color::hsba($h, $s, $b, $a)
    };
}

/// # Constructs a [`Hsl`] [`Color`].
///
/// # Examples
///
/// ```
/// use pix_engine::prelude::*;
///
/// let c = hsl!(50.0); // Gray
/// assert_eq!(c.channels(), [128, 128, 128, 255]);
///
/// let c = hsl!(50.0, 0.5); // Gray with alpha
/// assert_eq!(c.channels(), [128, 128, 128, 128]);
///
/// let c = hsl!(342.0, 100.0, 80.0); // Hue, Saturation, Lightness
/// assert_eq!(c.channels(), [255, 153, 184, 255]);
///
/// let c = hsl!(342.0, 100.0, 80.0, 0.5); // Hue, Saturation, Lightness, Alpha
/// assert_eq!(c.channels(), [255, 153, 184, 128]);
/// ```
#[macro_export]
macro_rules! hsl {
    ($gray:expr) => {
        hsl!(0.0, 0.0, $gray)
    };
    ($gray:expr, $a:expr$(,)?) => {
        hsl!(0.0, 0.0, $gray, $a)
    };
    ($h:expr, $s:expr, $l:expr$(,)?) => {
        hsl!($h, $s, $l, 1.0)
    };
    ($h:expr, $s:expr, $l:expr, $a:expr$(,)?) => {
        $crate::color::Color::hsla($h, $s, $l, $a)
    };
}

impl Default for Color {
    fn default() -> Self {
        Self::rgb(0, 0, 0)
    }
}

/// Display [`Color`] as "[r, g, b, a]".
impl fmt::Display for Color {
    #[allow(clippy::many_single_char_names)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let [r, g, b, a] = self.channels();
        write!(f, "[{}, {}, {}, {}]", r, g, b, a)
    }
}

impl LowerHex for Color {
    #[allow(clippy::many_single_char_names)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let [r, g, b, a] = self.channels();
        write!(f, "#{:x}{:x}{:x}{:x}", r, g, b, a)
    }
}

impl UpperHex for Color {
    #[allow(clippy::many_single_char_names)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let [r, g, b, a] = self.channels();
        write!(f, "#{:X}{:X}{:X}{:X}", r, g, b, a)
    }
}

// Operations

impl Index<usize> for Color {
    type Output = f64;
    fn index(&self, idx: usize) -> &Self::Output {
        match idx {
            i if i < 4 => self.levels.get(i).unwrap(),
            _ => panic!("index out of bounds: the len is 4 but the index is {}", idx),
        }
    }
}

impl Add for Color {
    type Output = Self;
    fn add(self, other: Color) -> Self::Output {
        let [v1, v2, v3, a] = self.levels;
        let [ov1, ov2, ov3, ova] = convert_levels(other.levels, other.mode, self.mode);
        let levels = [v1 + ov1, v2 + ov2, v3 + ov3, a + ova];
        let channels = calculate_channels(levels);
        Self {
            mode: self.mode,
            levels,
            channels,
        }
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, other: Color) {
        let [v1, v2, v3, a] = self.levels;
        let [ov1, ov2, ov3, ova] = convert_levels(other.levels, other.mode, self.mode);
        self.levels = [v1 + ov1, v2 + ov2, v3 + ov3, a + ova];
        for level in &mut self.levels {
            *level = level.clamp(0.0, 1.0);
        }
        self.calculate_channels();
    }
}

impl Sub for Color {
    type Output = Self;
    fn sub(self, other: Color) -> Self::Output {
        let [v1, v2, v3, a] = self.levels;
        let [ov1, ov2, ov3, ova] = convert_levels(other.levels, other.mode, self.mode);
        let levels = [v1 - ov1, v2 - ov2, v3 - ov3, a - ova];
        let channels = calculate_channels(levels);
        Self {
            mode: self.mode,
            levels,
            channels,
        }
    }
}

impl SubAssign for Color {
    fn sub_assign(&mut self, other: Color) {
        let [v1, v2, v3, a] = self.levels;
        let [ov1, ov2, ov3, ova] = convert_levels(other.levels, other.mode, self.mode);
        self.levels = [v1 - ov1, v2 - ov2, v3 - ov3, a - ova];
        for level in &mut self.levels {
            *level = level.clamp(0.0, 1.0);
        }
        self.calculate_channels();
    }
}

impl ExactSizeIterator for Iter {}

impl<T: Into<f64>> FromIterator<T> for Color {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        let mut rgba = [0.0, 0.0, 0.0, 255.0];
        for (i, v) in iter.into_iter().enumerate() {
            rgba[i] = v.into();
        }
        let [r, g, b, a] = rgba;
        Self::rgba(r, g, b, a)
    }
}

impl IntoIterator for Color {
    type Item = u8;
    type IntoIter = IntoIter<Self::Item, 4>;

    /// Owned `Color` iterator over `[r, g, b, a]`.
    ///
    /// This struct is created by the [`into_iter`](Color::into_iter) method on [`Color`]s.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let c = color!(100, 200, 50);
    /// let mut iterator = c.into_iter();
    ///
    /// assert_eq!(iterator.next(), Some(100));
    /// assert_eq!(iterator.next(), Some(200));
    /// assert_eq!(iterator.next(), Some(50));
    /// assert_eq!(iterator.next(), Some(255));
    /// assert_eq!(iterator.next(), None);
    /// ```
    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self.channels())
    }
}

/// Immutable `Color` iterator over `[r, g, b, a]`.
///
/// This struct is created by the [`iter`](Color::iter) method on [`Color`]s.
///
/// # Example
///
/// ```
/// # use pix_engine::prelude::*;
/// let c: Color = color!(100, 200, 50);
/// let mut iterator = c.iter();
///
/// assert_eq!(iterator.next(), Some(100));
/// assert_eq!(iterator.next(), Some(200));
/// assert_eq!(iterator.next(), Some(50));
/// assert_eq!(iterator.next(), Some(255));
/// assert_eq!(iterator.next(), None);
/// ```
#[derive(Debug, Clone)]
pub struct Iter {
    inner: [u8; 4],
    current: usize,
}

impl Iter {
    #[inline]
    fn new(color: &Color) -> Self {
        Self {
            inner: color.channels(),
            current: 0,
        }
    }
}

impl Iterator for Iter {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current > 3 {
            return None;
        }
        let next = self.inner[self.current];
        self.current += 1;
        Some(next)
    }
}

impl IntoIterator for &Color {
    type Item = u8;
    type IntoIter = Iter;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

macro_rules! impl_ops {
    ($($target:ty),*) => {
        $(
            impl Mul<$target> for Color where $target: Into<f64> {
                type Output = Self;
                fn mul(self, s: $target) -> Self::Output {
                    let [v1, v2, v3, a] = self.levels;
                    let s = f64::from(s);
                    let levels = [v1 * s, v2 * s, v3 * s, a * s];
                    let channels = calculate_channels(levels);
                    Self {
                        mode: self.mode,
                        levels,
                        channels,
                    }
                }
            }

            impl Mul<Color> for $target where $target: Into<f64> {
                type Output = Color;
                fn mul(self, c: Color) -> Self::Output {
                    let [v1, v2, v3, a] = c.levels();
                    let s = f64::from(self);
                    let levels = [v1 * s, v2 * s, v3 * s, a * s];
                    let channels = calculate_channels(levels);
                    Color {
                        mode: c.mode,
                        levels,
                        channels,
                    }
                }
            }

            impl MulAssign<$target> for Color where $target: Into<f64> {
                fn mul_assign(&mut self, s: $target) {
                    let [v1, v2, v3, a] = self.levels;
                    let s = f64::from(s);
                    self.levels = [v1 * s, v2 * s, v3 * s, a * s];
                    for level in &mut self.levels {
                        *level = level.clamp(0.0, 1.0);
                    }
                    self.calculate_channels();
                }
            }

            impl Div<$target> for Color where $target: Into<f64> {
                type Output = Self;
                fn div(self, s: $target) -> Self::Output {
                    let [v1, v2, v3, a] = self.levels;
                    let s = f64::from(s);
                    let levels = [v1 / s, v2 / s, v3 / s, a / s];
                    let channels = calculate_channels(levels);
                    Self {
                        mode: self.mode,
                        levels,
                        channels,
                    }
                }
            }

            impl DivAssign<$target> for Color where $target: Into<f64> {
                fn div_assign(&mut self, s: $target) {
                    let [v1, v2, v3, a] = self.levels;
                    let s = f64::from(s);
                    self.levels = [v1 / s, v2 / s, v3 / s, a / s];
                    for level in &mut self.levels {
                        *level = level.clamp(0.0, 1.0);
                    }
                    self.calculate_channels();
                }
            }
        )*
    };
}

macro_rules! impl_as_ops {
    ($($target:ty),*) => {
        $(
            impl Mul<$target> for Color {
                type Output = Self;
                fn mul(self, s: $target) -> Self::Output {
                    let [v1, v2, v3, a] = self.levels;
                    let s = s as f64;
                    let levels = [v1 * s, v2 * s, v3 * s, a * s];
                    let channels = calculate_channels(levels);
                    Self {
                        mode: self.mode,
                        levels,
                        channels,
                    }
                }
            }

            impl Mul<Color> for $target {
                type Output = Color;
                fn mul(self, c: Color) -> Self::Output {
                    let [v1, v2, v3, a] = c.levels();
                    let s = self as f64;
                    let levels = [v1 * s, v2 * s, v3 * s, a * s];
                    let channels = calculate_channels(levels);
                    Color {
                        mode: c.mode,
                        levels,
                        channels,
                    }
                }
            }

            impl MulAssign<$target> for Color {
                fn mul_assign(&mut self, s: $target) {
                    let [v1, v2, v3, a] = self.levels;
                    let s = s as f64;
                    self.levels = [v1 * s, v2 * s, v3 * s, a * s];
                    for level in &mut self.levels {
                        *level = level.clamp(0.0, 1.0);
                    }
                    self.calculate_channels();
                }
            }

            impl Div<$target> for Color {
                type Output = Self;
                fn div(self, s: $target) -> Self::Output {
                    let [v1, v2, v3, a] = self.levels;
                    let s = s as f64;
                    let levels = [v1 / s, v2 / s, v3 / s, a / s];
                    let channels = calculate_channels(levels);
                    Self {
                        mode: self.mode,
                        levels,
                        channels,
                    }
                }
            }

            impl DivAssign<$target> for Color {
                fn div_assign(&mut self, s: $target) {
                    let [v1, v2, v3, a] = self.levels;
                    let s = s as f64;
                    self.levels = [v1 / s, v2 / s, v3 / s, a / s];
                    for level in &mut self.levels {
                        *level = level.clamp(0.0, 1.0);
                    }
                    self.calculate_channels();
                }
            }
        )*
    }
}

impl_ops!(i8, u8, i16, u16, i32, u32, f32, f64);
impl_as_ops!(isize, usize, i64, u64, i128, u128);

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_ops {
        ($($val: expr),*) => {
            $(
                // Mul<T> for Color
                let c = color!(200, 50, 10, 100) * $val;
                assert_eq!(c.channels(), [255, 100, 20, 200]);

                // Mul<Color> for T
                let c: Color = $val * color!(200, 50, 10, 100);
                assert_eq!(c.channels(), [255, 100, 20, 200]);

                // MulAssign<T> for Color
                let mut c = color!(200, 50, 10, 100);
                c *= $val;
                assert_eq!(c.channels(), [255, 100, 20, 200]);

                // Div<T> for Color
                let c: Color = color!(100, 255, 0, 100) / $val;
                assert_eq!(c.channels(), [50, 128, 0, 50]);

                // DivAssign<T> for Color
                let mut c = color!(200, 50, 10, 100);
                c /= $val;
                assert_eq!(c.channels(), [100, 25, 5, 50]);
            )*
        };
    }

    #[test]
    fn test_ops() {
        // Add
        let c1 = color!(200, 50, 10, 100);
        let c2 = color!(100, 50, 10, 100);
        let c3 = c1 + c2;
        assert_eq!(c3.channels(), [255, 100, 20, 200]);

        // AddAssign
        let mut c1 = color!(200, 50, 10, 100);
        let c2 = color!(100, 50, 10, 100);
        c1 += c2;
        assert_eq!(c1.channels(), [255, 100, 20, 200]);

        // Sub
        let c1 = color!(200, 100, 20, 200);
        let c2 = color!(100, 50, 30, 100);
        let c3 = c1 - c2;
        assert_eq!(c3.channels(), [100, 50, 0, 100]);

        // SubAssign
        let mut c1 = color!(200, 100, 20, 200);
        let c2 = color!(100, 50, 30, 100);
        c1 -= c2;
        assert_eq!(c1.channels(), [100, 50, 0, 100]);

        test_ops!(2i8, 2u8, 2i16, 2u16, 2i32, 2u32, 2f32, 2f64);
    }

    #[test]
    fn test_constructors() {
        let expected = |mode| Color {
            mode,
            levels: [0.0, 0.0, 0.0, 1.0],
            channels: [0, 0, 0, 255],
        };
        assert_eq!(Color::with_mode(Rgb, 0.0, 0.0, 0.0), expected(Rgb));
        assert_eq!(
            Color::with_mode_alpha(Rgb, 0.0, 0.0, 0.0, 255.0),
            expected(Rgb)
        );
        assert_eq!(Color::with_mode(Hsb, 0.0, 0.0, 0.0), expected(Hsb));
        assert_eq!(
            Color::with_mode_alpha(Hsb, 0.0, 0.0, 0.0, 1.0),
            expected(Hsb)
        );
        assert_eq!(Color::with_mode(Hsl, 0.0, 0.0, 0.0), expected(Hsl));
        assert_eq!(
            Color::with_mode_alpha(Hsl, 0.0, 0.0, 0.0, 1.0),
            expected(Hsl)
        );
    }
}
