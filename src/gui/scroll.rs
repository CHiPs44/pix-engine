//! UI scrollbar rendering functions.

use super::{state::ElementId, Direction};
use crate::prelude::*;
use std::cmp;

pub(crate) const THUMB_MIN: i32 = 10;
pub(crate) const SCROLL_SIZE: i32 = 12;
pub(crate) const SCROLL_SPEED: i32 = 3;

impl PixState {
    /// Handles mouse wheel scroll for `hovered` elements.
    pub(crate) fn scroll(
        &mut self,
        id: ElementId,
        mut rect: Rect<i32>,
        width: i32,
        height: i32,
    ) -> PixResult<Rect<i32>> {
        use cmp::{max, min};
        let s = self;

        let mut scroll = s.ui.scroll(id);
        let xmax = width - rect.width();
        let ymax = height - rect.height();
        if s.ui.is_hovered(id) {
            let speed = 3;
            if s.ui.mouse.xrel != 0 {
                scroll.set_x(max(0, min(xmax, scroll.x() - speed * s.ui.mouse.xrel)));
                s.ui.set_scroll(id, scroll);
            }
            if s.ui.mouse.yrel != 0 {
                scroll.set_y(max(0, min(ymax, scroll.y() - speed * s.ui.mouse.yrel)));
                s.ui.set_scroll(id, scroll);
            }
        }

        // Vertical scroll
        if height > rect.height() {
            let mut scroll_y = scroll.y();
            let scrolled = s.scrollbar(
                rect![rect.right() + 1, rect.top(), SCROLL_SIZE, rect.height()],
                ymax as u32,
                &mut scroll_y,
                Direction::Vertical,
            )?;
            if scrolled {
                scroll.set_y(scroll_y);
                s.ui.set_scroll(id, scroll);
            }
        }

        // Horizontal scroll
        if width > rect.width() {
            let mut scroll_x = scroll.x();
            let scrolled = s.scrollbar(
                rect![
                    rect.left(),
                    rect.bottom() + 1,
                    rect.width() - SCROLL_SIZE,
                    SCROLL_SIZE
                ],
                xmax as u32,
                &mut scroll_x,
                Direction::Horizontal,
            )?;
            if scrolled {
                scroll.set_x(scroll_x);
                s.ui.set_scroll(id, scroll);
            }
        }

        rect.offset_size([SCROLL_SIZE, SCROLL_SIZE]);
        Ok(rect)
    }

    fn scrollbar(
        &mut self,
        rect: Rect<i32>,
        max: u32,
        value: &mut i32,
        dir: Direction,
    ) -> PixResult<bool> {
        use Direction::*;

        let s = self;
        let id = s.ui.get_id(&rect);

        // Check hover/active/keyboard focus
        let hovered = s.ui.try_hover(id, rect);
        let focused = s.ui.try_focus(id);
        let disabled = s.ui.disabled;
        let active = s.ui.is_active(id);

        s.push();

        // Clamp value
        let max = max as i32;
        *value = cmp::max(0, cmp::min(max, *value));

        // Scroll region
        s.no_stroke();
        s.fill(s.background_color());
        s.rect(rect)?;

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
        let thumb_w = match dir {
            Horizontal => {
                let w = rect.width() as f32;
                let w = ((w / (max as f32 + w)) * w) as i32;
                w.max(THUMB_MIN).min(w)
            }
            Vertical => rect.width(),
        };
        let thumb_h = match dir {
            Horizontal => rect.height(),
            Vertical => {
                let h = rect.height() as f32;
                let h = ((h / (max as f32 + h)) * h) as i32;
                h.max(THUMB_MIN).min(h)
            }
        };
        match dir {
            Horizontal => {
                let thumb_x = ((rect.width() - thumb_w) * *value) / max;
                s.rect([rect.x() + thumb_x, rect.y(), thumb_w, thumb_h])?
            }
            Vertical => {
                let thumb_y = ((rect.height() - thumb_h) * *value) / max;
                s.rect([rect.x(), rect.y() + thumb_y, thumb_w, thumb_h])?
            }
        }

        s.pop();

        // Process keyboard input
        let mut new_value = *value;
        if focused {
            if let Some(key) = s.ui.key_entered() {
                match key {
                    Key::Up if dir == Vertical => {
                        new_value = value.saturating_sub(SCROLL_SPEED).max(0);
                    }
                    Key::Down if dir == Vertical => {
                        new_value = value.saturating_add(SCROLL_SPEED).min(max);
                    }
                    Key::Left if dir == Horizontal => {
                        new_value = value.saturating_sub(SCROLL_SPEED).max(0);
                    }
                    Key::Right if dir == Horizontal => {
                        new_value = value.saturating_add(SCROLL_SPEED).min(max);
                    }
                    _ => (),
                }
            }
        }

        // Process mouse wheel
        if hovered {
            match dir {
                Vertical if s.ui.mouse.yrel != 0 => {
                    new_value -= SCROLL_SPEED * s.ui.mouse.yrel;
                }
                Horizontal if s.ui.mouse.xrel != 0 => {
                    new_value -= SCROLL_SPEED * s.ui.mouse.xrel;
                }
                _ => (),
            };
        }
        // Process mouse input
        if active {
            new_value = match dir {
                Vertical => {
                    let my = (s.mouse_pos().y() - rect.y()).clamp(0, rect.height());
                    (my * max) / rect.height()
                }
                Horizontal => {
                    let mx = (s.mouse_pos().x() - rect.x()).clamp(0, rect.width());
                    (mx * max) / rect.width()
                }
            };
        }
        s.ui.handle_events(id);

        if new_value != *value {
            *value = new_value;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
