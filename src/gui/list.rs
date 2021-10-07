//! Immediate-GUI functions related to rendering and interacting with lists and select boxes.

use num_traits::AsPrimitive;

use super::get_hash;
use crate::{prelude::*, renderer::Rendering};

impl PixState {
    /// Draw a select list to the current canvas with a scrollable region.
    pub fn select_list<R, S, I, T>(
        &mut self,
        rect: R,
        label: S,
        items: &[I],
        item_height: T,
        selected: &mut Option<usize>,
    ) -> PixResult<()>
    where
        R: Into<Rect<i32>>,
        S: AsRef<str>,
        I: AsRef<str>,
        T: AsPrimitive<u32>,
    {
        let rect = self.get_rect(rect);
        self._select_list(rect, label.as_ref(), items, item_height.as_(), selected)
    }

    fn _select_list<S>(
        &mut self,
        rect: Rect<i32>,
        label: &str,
        items: &[S],
        item_height: u32,
        selected: &mut Option<usize>,
    ) -> PixResult<()>
    where
        S: AsRef<str>,
    {
        let s = self;
        let id = get_hash(&rect);

        s.push();

        let pad = s.theme.padding;
        let radius = 3;
        let scroll_width = 16;

        let mut border = rect;
        if !label.is_empty() {
            let (_, h) = s.size_of(&label)?;
            border.set_y(border.y() + h as i32 + pad); // Push border down past label
        }
        let mut content = Rect::resized(border, -radius);
        let line_height = item_height as i32 + pad * 2;
        let total_height = items.len() as i32 * line_height;
        let mut scroll = s.ui_state.scroll(id);
        let skip_count = (scroll.y() / line_height) as usize;
        let displayed_count = (content.height() / line_height) as usize;

        let scroll_enabled = total_height > content.height();
        if scroll_enabled {
            content.set_width(content.width() - scroll_width);
        }

        // Check hover/active/keyboard focus
        if content.contains_point(s.mouse_pos()) {
            s.ui_state.hover(id);
        }
        s.ui_state.try_capture(id);

        // Render
        s.rect_mode(RectMode::Corner);
        s.renderer.font_family(&s.theme.fonts.body)?;

        // Label
        if !label.is_empty() {
            s.fill(s.text_color());
            s.text([rect.x(), rect.y()], label)?;
        }

        // Background
        s.fill(s.primary_color());
        s.rounded_rect(border, radius)?;

        // Contents
        // TODO: Move this to ElementState (requires migrating back to textureId references)
        let mut texture =
            s.create_texture(content.width() as u32, content.height() as u32, None)?;
        s.with_texture(&mut texture, |s: &mut PixState| -> PixResult<()> {
            // Because x/y is now relative to this texture and scroll, offset the mouse
            let mut mouse = s.mouse_pos();
            mouse.offset(-content.top_left());
            s.background(s.primary_color())?;

            let x = pad;
            let mut y = -scroll.y() + (skip_count as i32 * line_height);
            for (i, item) in items
                .iter()
                .enumerate()
                .skip(skip_count)
                .take(displayed_count + 2)
            {
                let item_rect = rect!(0, y, content.width(), line_height);
                let clickable = item_rect.bottom() > 0 || item_rect.top() < content.height();
                if clickable {
                    let mut click_area = item_rect;
                    if click_area.top() < 0 {
                        click_area.set_height(click_area.height() + click_area.y());
                        click_area.set_top(0);
                    }
                    if click_area.bottom() > content.height() {
                        click_area.set_height(content.height() - click_area.top());
                    }
                    if click_area.contains_point(mouse) {
                        s.frame_cursor(&Cursor::hand())?;
                        if s.ui_state.is_active(id) && s.mouse_down(Mouse::Left) {
                            *selected = Some(i);
                        }
                    }
                }
                if matches!(*selected, Some(el) if el == i) {
                    s.no_stroke();
                    s.fill(s.highlight_color());
                    s.rounded_rect(item_rect, radius)?;
                    s.fill(BLACK);
                } else {
                    s.fill(WHITE);
                }
                s.text([x, y + pad], item)?;
                y += line_height;
            }
            Ok(())
        })?;
        s.clip(content)?;
        s.texture(&mut texture, None, content)?;
        s.no_clip()?;

        // Process input
        let focused = s.ui_state.is_focused(id);
        if focused {
            if let Some(key) = s.ui_state.key_entered() {
                let changed_selection = match key {
                    Key::Up => {
                        *selected = selected.map(|s| s.saturating_sub(1)).or(Some(0));
                        true
                    }
                    Key::Down => {
                        *selected = selected
                            .map(|s| {
                                let mut s = s.saturating_add(1);
                                if s >= items.len() {
                                    s = items.len() - 1;
                                }
                                s
                            })
                            .or(Some(0));
                        true
                    }
                    _ => false,
                };
                if changed_selection {
                    let sel_y = selected.unwrap_or(0) as i32 * line_height;
                    // Snap scroll to top of the window
                    if sel_y < scroll.y() {
                        scroll.set_y(sel_y);
                        s.ui_state.set_scroll(id, scroll);
                    } else if sel_y + line_height > scroll.y() + content.height() {
                        // Snap scroll to bottom of the window
                        scroll.set_y(sel_y - (content.height() - line_height));
                        s.ui_state.set_scroll(id, scroll);
                    }
                }
            }
        }
        s.ui_state.handle_input(id);

        // Scrollbar
        let max_scroll = total_height - content.height();
        if scroll_enabled
            && s.slider(
                [
                    border.right() - scroll_width,
                    border.top(),
                    scroll_width,
                    border.height(),
                ],
                max_scroll,
                &mut scroll.y_mut(),
            )?
        {
            s.ui_state.set_scroll(id, scroll);
        }

        // Border
        s.no_fill();
        if focused {
            s.stroke(s.secondary_color());
        } else {
            s.stroke(s.muted_color());
        }
        s.rounded_rect(border, radius)?;

        s.pop();

        Ok(())
    }
}
