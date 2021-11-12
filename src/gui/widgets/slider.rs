//! Slider and drag widget rendering methods.
//!
//! Provided [PixState] methods:
//!
//! - [PixState::drag]
//! - [PixState::advanced_drag]
//!
//! # Example
//!
//! ```
//! # use pix_engine::prelude::*;
//! # struct App { drag: i32, advanced_drag: f32};
//! # impl AppState for App {
//! fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
//!     s.drag("Drag", &mut self.drag, 1)?;
//!     s.advanced_drag(
//!         "Advanced Drag",
//!         &mut self.advanced_drag,
//!         0.005,
//!         0.0,
//!         1.0,
//!         Some(|val| format!("{:.3}", val).into()),
//!     )?;
//!     Ok(())
//! }
//! # }
//! ```

use crate::{
    gui::{scroll::THUMB_MIN, MOD_CTRL},
    prelude::*,
};
use num_traits::{clamp, Bounded, NumCast};
use std::{borrow::Cow, error::Error, fmt, str::FromStr};

impl PixState {
    /// Draw a draggable number widget to the current canvas.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App { int: i32, float: f32};
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     s.drag("Drag Int", &mut self.int, 1)?;
    ///     s.drag("Drag Float", &mut self.float, 0.005)?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    pub fn drag<T, L>(&mut self, label: L, value: &mut T, speed: T) -> PixResult<bool>
    where
        T: Num + NumCast + Bounded + fmt::Display,
        L: AsRef<str>,
    {
        self.advanced_drag(label, value, speed, T::min_value(), T::max_value(), None)
    }

    /// Draw an advanced draggable number widget to the current canvas.
    ///
    /// # Example
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App { advanced_drag: f32};
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     s.advanced_drag(
    ///         "Advanced Drag",
    ///         &mut self.advanced_drag,
    ///         0.005,
    ///         0.0,
    ///         1.0,
    ///         Some(|val| format!("{:.3}", val).into()),
    ///     )?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    pub fn advanced_drag<'a, T, L>(
        &mut self,
        label: L,
        value: &mut T,
        speed: T,
        min: T,
        max: T,
        formatter: Option<fn(&T) -> Cow<'a, str>>,
    ) -> PixResult<bool>
    where
        T: Num + NumCast + fmt::Display,
        L: AsRef<str>,
    {
        let label = label.as_ref();
        let s = self;
        let id = s.ui.get_id(&label);
        let label = label.split('#').next().unwrap_or("");
        let pos = s.cursor_pos();
        let font_size = s.theme.font_sizes.body as i32;
        let style = s.theme.style;
        let fpad = style.frame_pad;
        let ipad = style.item_pad;

        // Calculate drag rect
        let width =
            s.ui.next_width
                .take()
                .unwrap_or_else(|| s.width().unwrap_or(100) - 2 * fpad.x() as u32);
        let mut drag = rect![pos, width as i32, font_size + 2 * ipad.y()];
        let (lwidth, lheight) = s.size_of(label)?;
        if !label.is_empty() {
            drag.offset_x(lwidth as i32 + ipad.x());
        }

        // Check hover/active/keyboard focus
        let hovered = s.ui.try_hover(id, drag);
        let focused = s.ui.try_focus(id);
        let disabled = s.ui.disabled;
        let active = s.ui.is_active(id);

        s.push();
        s.ui.push_cursor();

        // Render
        s.rect_mode(RectMode::Corner);

        // Label
        if !label.is_empty() {
            s.set_cursor_pos([pos.x(), pos.y() + drag.height() / 2 - lheight as i32 / 2]);
            s.text(label)?;
        }

        // Rect
        s.push();
        if focused || active {
            s.stroke(s.highlight_color());
        } else {
            s.stroke(s.muted_color());
        }
        if active {
            s.frame_cursor(Cursor::hand())?;
            s.fill(s.highlight_color());
        } else if disabled {
            s.fill(s.primary_color() / 2);
        } else if hovered {
            s.frame_cursor(Cursor::hand())?;
            s.fill(s.secondary_color());
        } else {
            s.fill(s.primary_color());
        }
        s.rect(drag)?;
        s.pop();

        // Value
        let text = if let Some(formatter) = formatter {
            formatter(value)
        } else {
            format!("{}", value).into()
        };
        let (vw, vh) = s.size_of(&text)?;
        let center = drag.center();
        let x = center.x() - vw as i32 / 2;
        let y = center.y() - vh as i32 / 2;
        s.set_cursor_pos([x, y]);
        s.text(&text)?;

        s.ui.pop_cursor();
        s.pop();

        // Process drag
        let mut changed = false;
        let mut new_value = *value;
        if active {
            let delta = s.mouse_pos().x() - s.pmouse_pos().x();
            let mut delta: T = NumCast::from(delta).expect("valid i32 cast");
            if s.keymod_down(KeyMod::ALT) {
                delta /= NumCast::from(100).expect("valid number cast");
            } else if s.keymod_down(KeyMod::SHIFT) {
                delta *= NumCast::from(10).expect("valid number cast");
            }
            new_value = clamp(new_value + (delta * speed), min, max);
        }
        if new_value != *value {
            *value = new_value;
            changed = true;
        }
        s.ui.handle_events(id);
        s.advance_cursor(rect![pos, drag.right() - pos.x(), drag.height()]);

        Ok(changed)
    }

    /// Draw a slider widget to the current canvas.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App { int: i32, float: f32};
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     s.slider("Slider Int", &mut self.int, -5, 5)?;
    ///     s.slider("Slider Float", &mut self.float, 0.0, 1.0)?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    pub fn slider<T, L>(&mut self, label: L, value: &mut T, min: T, max: T) -> PixResult<bool>
    where
        T: Num + NumCast + fmt::Display + FromStr,
        <T as FromStr>::Err: Error + Sync + Send + 'static,
        L: AsRef<str>,
    {
        self.advanced_slider(label, value, min, max, None)
    }

    /// Draw an advanced slider widget to the current canvas.
    ///
    /// # Example
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App { advanced_slider: f32};
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     s.advanced_slider(
    ///         "Advanced Slider",
    ///         &mut self.advanced_slider,
    ///         0.0,
    ///         1.0,
    ///         Some(|val| format!("ratio = {:.3}", val).into()),
    ///     )?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    pub fn advanced_slider<'a, T, L>(
        &mut self,
        label: L,
        value: &mut T,
        min: T,
        max: T,
        formatter: Option<fn(&T) -> Cow<'a, str>>,
    ) -> PixResult<bool>
    where
        T: Num + NumCast + fmt::Display + FromStr,
        <T as FromStr>::Err: Error + Sync + Send + 'static,
        L: AsRef<str>,
    {
        let label = label.as_ref();
        let s = self;
        let id = s.ui.get_id(&label);
        let label = label.split('#').next().unwrap_or("");
        let pos = s.cursor_pos();
        let font_size = s.theme.font_sizes.body as i32;
        let style = s.theme.style;
        let fpad = style.frame_pad;
        let ipad = style.item_pad;

        // If editing, render editable text field instead
        let editing = s.ui.is_editing(id);
        let disabled = s.ui.disabled;
        if editing {
            if disabled {
                s.ui.end_edit();
            } else {
                let mut text = s.ui.text_edit(id, value.to_string());
                let changed = s.advanced_text_field(
                    label,
                    "",
                    &mut text,
                    Some(|c| c.is_ascii_digit() || c == '.' || c == '-'),
                )?;
                s.ui.set_text_edit(id, text);

                if let Some(Key::Return | Key::Escape) = s.ui.key_entered() {
                    s.ui.end_edit();
                }
                return Ok(changed);
            }
        }
        *value = clamp(s.ui.parse_text_edit(id, *value)?, min, max);

        // Calculate slider rect
        let width =
            s.ui.next_width
                .take()
                .unwrap_or_else(|| s.width().unwrap_or(100) - 2 * fpad.x() as u32);
        let mut slider = rect![pos, width as i32, font_size + 2 * ipad.y()];
        let (lwidth, lheight) = s.size_of(label)?;
        if !label.is_empty() {
            slider.offset_x(lwidth as i32 + ipad.x());
        }

        // Check hover/active/keyboard focus
        let hovered = s.ui.try_hover(id, slider);
        let focused = s.ui.try_focus(id);
        let active = s.ui.is_active(id);

        s.push();
        s.ui.push_cursor();

        // Render
        s.rect_mode(RectMode::Corner);

        // Label
        if !label.is_empty() {
            s.set_cursor_pos([pos.x(), pos.y() + slider.height() / 2 - lheight as i32 / 2]);
            s.text(label)?;
        }

        s.push();

        // Slider region
        s.stroke(s.muted_color());
        if disabled {
            s.fill(s.muted_color() / 4);
        } else {
            s.fill(s.muted_color() / 2);
        }
        s.rect(slider)?;

        // Scroll thumb
        if hovered {
            s.frame_cursor(Cursor::hand())?;
        }
        if hovered || active || focused {
            s.fill(s.highlight_color());
        } else if disabled {
            s.fill(s.muted_color() / 2);
        } else {
            s.fill(s.muted_color());
        }
        let slider_w = slider.width() as Scalar;
        let vmin: Scalar = NumCast::from(min).expect("valid number cast");
        let vmax: Scalar = NumCast::from(max).expect("valid number cast");
        let val: Scalar = NumCast::from(*value).expect("valid number cast");
        let thumb_w = if vmax - vmin > 1.0 {
            slider_w / (vmax - vmin)
        } else {
            THUMB_MIN as Scalar
        };
        let thumb_w = thumb_w.min(slider_w);
        let offset = ((val - vmin) / (vmax - vmin)) * (slider_w - thumb_w);
        let x = slider.x() + offset as i32;
        let thumb = rect![x, slider.y(), thumb_w as i32, slider.height()];
        s.rect(thumb)?;

        s.pop();

        // Value
        let text = if let Some(formatter) = formatter {
            formatter(value)
        } else {
            format!("{}", value).into()
        };
        let (vw, vh) = s.size_of(&text)?;
        let center = slider.center();
        let x = center.x() - vw as i32 / 2;
        let y = center.y() - vh as i32 / 2;
        s.set_cursor_pos([x, y]);
        s.text(&text)?;

        s.ui.pop_cursor();
        s.pop();

        let mut new_value = *value;
        if active && s.keymod_down(MOD_CTRL) {
            // Process keyboard input
            s.ui.begin_edit(id);
        } else {
            // Process mouse input
            if active {
                let mx = (s.mouse_pos().x() - slider.x()).clamp(0, slider.width()) as Scalar
                    / slider.width() as Scalar;
                new_value = NumCast::from(mx * (vmax - vmin) + vmin).unwrap();
            }
        }
        s.ui.handle_events(id);
        s.advance_cursor(rect![pos, slider.right() - pos.x(), slider.height()]);

        if new_value != *value {
            *value = new_value;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
