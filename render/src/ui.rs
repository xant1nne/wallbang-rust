// Config block: 1:1 порт xui поверх DrawList (без доступа к памяти игры).
use crate::draw::{Color, DrawList};
use std::collections::{HashMap, HashSet};
use wallbang_core::input::{cursor_position, key_down};
use wallbang_core::math::Vec2;

pub const POPUP_W: f32 = 220.0;
pub const WINDOW_PAD: f32 = 12.0;
pub const ITEM_SPACING_Y: f32 = 8.0;
pub const ITEM_SPACING_X: f32 = 8.0;
pub const ROUNDING: f32 = 4.0;
pub const WIDGET_ROUNDING: f32 = 3.0;

#[derive(Clone, Copy, Debug)]
pub struct Style {
    pub accent: Color,
    pub dark: Color,
    pub card: Color,
    pub bg: Color,
    pub elevated: Color,
    pub hovered: Color,
    pub active: Color,
    pub border: Color,
    pub slider_fill: Color,
    pub text: Color,
    pub text_dim: Color,
    /// Toggle/checkbox tokens copied from the C++ xui style.
    pub checkbox_bg: Color,
    pub checkbox_border: Color,
    pub checkbox_mark: Color,
    pub checkbox_mark_icon: Color,
}

impl Default for Style {
    fn default() -> Self {
        Self::new([164, 204, 232, 255])
    }
}

impl Style {
    /// Exact theme tokens matching the C++ screenshots (#08080A pitch dark, #141416 elevated, #1E1E22 border).
    pub fn new(accent: Color) -> Self {
        Self {
            accent: [accent[0], accent[1], accent[2], 255],
            dark: [8, 8, 10, 255],
            card: [14, 14, 16, 255],
            bg: [8, 8, 10, 255],
            elevated: [20, 20, 22, 255],
            hovered: [26, 26, 29, 255],
            active: [40, 40, 44, 255],
            border: [28, 28, 31, 255],
            slider_fill: [174, 174, 174, 255],
            text: [232, 232, 236, 255],
            text_dim: [130, 132, 138, 255],
            checkbox_bg: [17, 17, 17, 82],
            checkbox_border: [0, 0, 0, 0],
            checkbox_mark: [173, 192, 255, 255],
            checkbox_mark_icon: [17, 17, 17, 255],
        }
    }
}

#[inline]
pub fn display_label(label: &str) -> &str {
    match label.find("##") {
        Some(idx) => &label[..idx],
        None => label,
    }
}

#[inline]
pub fn lerp_color(a: Color, b: Color, t: f32) -> Color {
    let t = t.clamp(0.0, 1.0);
    [
        (a[0] as f32 + (b[0] as f32 - a[0] as f32) * t) as u8,
        (a[1] as f32 + (b[1] as f32 - a[1] as f32) * t) as u8,
        (a[2] as f32 + (b[2] as f32 - a[2] as f32) * t) as u8,
        (a[3] as f32 + (b[3] as f32 - a[3] as f32) * t) as u8,
    ]
}

pub struct Ui {
    mouse: Vec2,
    prev_mouse: Vec2,
    down: bool,
    prev_down: bool,
    clicked: bool,
    right_clicked: bool,
    prev_right: bool,
    cursor: Vec2,
    row_w: f32,
    child_bottom: f32,
    last_rect: (f32, f32, f32, f32),
    last_id: String,
    same_line_next: bool,
    active_slider: Option<String>,
    active_text: Option<String>,
    open_list: Option<String>,
    open_popup: Option<String>,
    open_sub_popup: Option<String>,
    open_color: Option<String>,
    popup_owners: HashSet<String>,
    in_popup: bool,
    dragging_window: bool,
    drag_offset: Vec2,
    prev_keys: [bool; 256],
    scroll: HashMap<u64, f32>,
    scroll_max: HashMap<u64, f32>,
    wheel: f32,
}

impl Default for Ui {
    fn default() -> Self {
        Self::new()
    }
}

impl Ui {
    pub fn new() -> Self {
        Self {
            mouse: Vec2::default(),
            prev_mouse: Vec2::default(),
            down: false,
            prev_down: false,
            clicked: false,
            right_clicked: false,
            prev_right: false,
            cursor: Vec2::default(),
            row_w: 220.0,
            child_bottom: 10_000.0,
            last_rect: (0.0, 0.0, 0.0, 0.0),
            last_id: String::new(),
            same_line_next: false,
            active_slider: None,
            active_text: None,
            open_list: None,
            open_popup: None,
            open_sub_popup: None,
            open_color: None,
            popup_owners: HashSet::new(),
            in_popup: false,
            dragging_window: false,
            drag_offset: Vec2::default(),
            prev_keys: [false; 256],
            scroll: HashMap::new(),
            scroll_max: HashMap::new(),
            wheel: 0.0,
        }
    }

    pub fn begin_frame(&mut self) {
        self.prev_mouse = self.mouse;
        if let Some(p) = cursor_position() {
            self.mouse = Vec2::new(p.x as f32, p.y as f32);
        }
        self.down = key_down(0x01);
        self.clicked = self.down && !self.prev_down;
        if !self.down {
            self.active_slider = None;
            self.dragging_window = false;
        }
        self.prev_down = self.down;
        let right = key_down(0x02);
        self.right_clicked = right && !self.prev_right;
        self.prev_right = right;
        self.same_line_next = false;
        self.in_popup = false;
        self.wheel = crate::hooks::take_wheel();
    }

    /// Injects synthetic mouse state for offline preview rendering.
    pub fn set_synthetic_mouse(&mut self, x: f32, y: f32) {
        self.prev_mouse = Vec2::new(x, y);
        self.mouse = Vec2::new(x, y);
        self.down = false;
        self.clicked = false;
        self.right_clicked = false;
    }

    #[inline]
    pub fn mouse(&self) -> Vec2 {
        self.mouse
    }

    #[inline]
    pub fn mouse_delta(&self) -> Vec2 {
        Vec2::new(self.mouse.x - self.prev_mouse.x, self.mouse.y - self.prev_mouse.y)
    }

    #[inline]
    pub fn mouse_down(&self) -> bool {
        self.down
    }

    #[inline]
    pub fn mouse_clicked(&self) -> bool {
        self.clicked
    }

    #[inline]
    pub fn right_clicked(&self) -> bool {
        self.right_clicked
    }

    #[inline]
    pub fn overlay_blocking(&self) -> bool {
        !self.in_popup
            && (self.open_popup.is_some()
                || self.open_list.is_some()
                || self.open_color.is_some())
    }

    pub fn close_overlays(&mut self) {
        self.open_popup = None;
        self.open_sub_popup = None;
        self.open_list = None;
        self.open_color = None;
        self.active_slider = None;
        self.active_text = None;
    }

    #[inline]
    pub fn set_cursor(&mut self, x: f32, y: f32) {
        self.cursor = Vec2::new(x, y);
        self.same_line_next = false;
    }

    #[inline]
    pub fn cursor(&self) -> Vec2 {
        self.cursor
    }

    #[inline]
    pub fn avail_w(&self) -> f32 {
        self.row_w
    }

    #[inline]
    pub fn same_line(&mut self) {
        self.same_line_next = true;
    }

    #[inline]
    pub fn advance(&mut self, dy: f32) {
        self.cursor.y += dy;
    }

    #[inline]
    pub fn inside(&self, x: f32, y: f32, w: f32, h: f32) -> bool {
        self.mouse.x >= x && self.mouse.x <= x + w && self.mouse.y >= y && self.mouse.y <= y + h
    }

    #[inline]
    fn next_pos(&mut self) -> (f32, f32) {
        if self.same_line_next {
            self.same_line_next = false;
            (self.last_rect.0 + self.last_rect.2 + ITEM_SPACING_X, self.last_rect.1)
        } else {
            (self.cursor.x, self.cursor.y)
        }
    }

    #[inline]
    fn mark_at(&mut self, x: f32, y: f32, w: f32, h: f32, id: &str) {
        self.last_rect = (x, y, w, h);
        self.last_id.clear();
        self.last_id.push_str(id);
        let next_y = y + h + ITEM_SPACING_Y;
        if next_y > self.cursor.y {
            self.cursor.y = next_y;
        }
    }

    /// Handles dragging the main menu window by its sidebar brand header.
    pub fn handle_window_drag(
        &mut self,
        mx: &mut f32,
        my: &mut f32,
        drag_w: f32,
        drag_h: f32,
        screen_w: f32,
        screen_h: f32,
        mw: f32,
        mh: f32,
    ) {
        if self.clicked && !self.overlay_blocking() && self.inside(*mx, *my, drag_w, drag_h) {
            self.dragging_window = true;
            self.drag_offset = Vec2::new(self.mouse.x - *mx, self.mouse.y - *my);
        }
        if self.dragging_window && self.down {
            *mx = (self.mouse.x - self.drag_offset.x).clamp(0.0, (screen_w - mw).max(0.0));
            *my = (self.mouse.y - self.drag_offset.y).clamp(0.0, (screen_h - mh).max(0.0));
        }
    }

    pub fn panel(&self, dl: &mut DrawList, x: f32, y: f32, w: f32, h: f32, s: &Style) {
        dl.rect_rounded(x, y, w, h, ROUNDING, s.dark);
        dl.outline_rounded(x, y, w, h, ROUNDING, 1.0, s.border);
    }

    #[inline]
    fn child_key(x: f32, y: f32, w: f32, h: f32) -> u64 {
        let x = (x * 4.0) as i64 as u64;
        let y = (y * 4.0) as i64 as u64;
        let w = (w * 4.0) as i64 as u64;
        let h = (h * 4.0) as i64 as u64;
        x.wrapping_mul(0x9E37_79B9_7F4A_7C15)
            ^ y.wrapping_mul(0xC2B2_AE3D_27D4_EB4F)
            ^ w.wrapping_mul(0x1656_67B1_9E37_79F9)
            ^ h
    }

    /// Child group box matching `xui::begin_child(..., scrollable)`.
    pub fn child<R>(
        &mut self,
        dl: &mut DrawList,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        border: bool,
        s: &Style,
        f: impl FnOnce(&mut Ui, &mut DrawList) -> R,
    ) -> R {
        if border {
            dl.rect_rounded(x, y, w, h, ROUNDING, s.dark);
            dl.outline_rounded(x, y, w, h, ROUNDING, 1.0, s.border);
        }
        let key = Self::child_key(x, y, w, h);
        let previous_max = self.scroll_max.get(&key).copied().unwrap_or(0.0);
        if previous_max > 0.0 && self.wheel != 0.0 && self.inside(x, y, w, h) {
            let current = self.scroll.entry(key).or_insert(0.0);
            *current = (*current - self.wheel * 48.0).clamp(0.0, previous_max);
            self.wheel = 0.0;
        }
        let offset = self
            .scroll
            .get(&key)
            .copied()
            .unwrap_or(0.0)
            .clamp(0.0, previous_max);

        let saved_cursor = self.cursor;
        let saved_w = self.row_w;
        let saved_bottom = self.child_bottom;
        let pad = if border { WINDOW_PAD } else { 0.0 };
        self.cursor = Vec2::new(x + pad, y + pad - offset);
        self.row_w = (w - pad * 2.0).max(1.0);
        self.child_bottom = y + h - 4.0;
        dl.push_clip(x + 1.0, y + 1.0, (w - 2.0).max(0.0), (h - 2.0).max(0.0));
        let out = f(self, dl);
        let content_h = (self.cursor.y + offset - y + pad).max(0.0);
        let max_scroll = (content_h - h + 2.0).max(0.0);
        self.scroll_max.insert(key, max_scroll);
        let current = self.scroll.entry(key).or_insert(0.0);
        if *current > max_scroll {
            *current = max_scroll;
        }
        if max_scroll > 0.5 {
            let track_x = x + w - 5.0;
            let track_y = y + 4.0;
            let track_h = (h - 8.0).max(1.0);
            let ratio = (h / content_h.max(1.0)).clamp(0.0, 1.0);
            let thumb_h = (track_h * ratio).max(18.0);
            let travel = (track_h - thumb_h).max(0.0);
            let thumb_y = track_y + travel * (*current / max_scroll).clamp(0.0, 1.0);
            dl.rect_rounded(track_x, track_y, 3.0, track_h, 1.5, [22, 22, 25, 180]);
            dl.rect_rounded(track_x, thumb_y, 3.0, thumb_h, 1.5, [92, 92, 100, 220]);
        }
        dl.pop_clip();
        self.cursor = saved_cursor;
        self.row_w = saved_w;
        self.child_bottom = saved_bottom;
        out
    }

    pub fn text(&mut self, dl: &mut DrawList, text: &str, c: Color, scale: f32) {
        let (x, y) = self.next_pos();
        let h = dl.text_height(scale);
        dl.text(x, y, text, c, scale);
        self.mark_at(x, y, dl.text_width(text, scale), h + 2.0, text);
    }

    pub fn separator(&mut self, dl: &mut DrawList, s: &Style) {
        let (x, y) = (self.cursor.x, self.cursor.y);
        dl.rect(x, y, self.row_w, 1.0, s.border);
        self.cursor.y += ITEM_SPACING_Y;
    }

    pub fn button_at(
        &mut self,
        dl: &mut DrawList,
        label: &str,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        s: &Style,
    ) -> bool {
        let shown = display_label(label);
        let hover = !self.overlay_blocking() && self.inside(x, y, w, h);
        let active = hover && self.down;
        let bg = if active {
            s.active
        } else if hover {
            s.hovered
        } else {
            s.elevated
        };
        dl.rect_rounded(x, y, w, h, WIDGET_ROUNDING, bg);
        dl.outline_rounded(x, y, w, h, WIDGET_ROUNDING, 1.0, s.border);
        let tw = dl.text_width(shown, 1.0);
        let th = dl.text_height(1.0);
        dl.text(
            (x + (w - tw) * 0.5).floor(),
            (y + (h - th) * 0.5).floor(),
            shown,
            if hover { s.text } else { s.text_dim },
            1.0,
        );
        hover && self.clicked
    }

    pub fn button(&mut self, dl: &mut DrawList, label: &str, w: f32, h: f32, s: &Style) -> bool {
        let (x, y) = self.next_pos();
        let bw = if w <= 0.0 { self.row_w } else { w.min(self.row_w) };
        let clicked = self.button_at(dl, label, x, y, bw, h, s);
        self.mark_at(x, y, bw, h, label);
        clicked
    }

    /// 12px square checkbox used by the multicombo list (C++ draw_minimal_checkbox).
    fn minimal_checkbox(&self, dl: &mut DrawList, x: f32, y: f32, size: f32, on: bool, s: &Style) {
        dl.rect_rounded(x, y, size, size, 4.0, [17, 17, 17, 255]);
        if on {
            dl.rect_rounded(x, y, size, size, 4.0, s.checkbox_mark);
            let ink = s.checkbox_mark_icon;
            dl.line(
                x + size * 0.25,
                y + size * 0.5,
                x + size * 0.43,
                y + size * 0.68,
                1.5,
                ink,
            );
            dl.line(
                x + size * 0.43,
                y + size * 0.68,
                x + size * 0.76,
                y + size * 0.31,
                1.5,
                ink,
            );
        }
    }

    /// Toggle switch copied 1:1 from the C++ `draw_setting_toggle` (28x16, knob 10, inset 3).
    pub fn checkbox(&mut self, dl: &mut DrawList, label: &str, value: &mut bool, s: &Style) -> bool {
        let (x, y) = self.next_pos();
        let w = self.row_w;
        let row_h = 24.0;
        let sw_w = 28.0;
        let sw_h = 16.0;
        let sw_x = x + w - sw_w;
        let sw_y = y + 4.0;
        let hover = !self.overlay_blocking() && self.inside(x, y, w, row_h);

        let shown = display_label(label);
        let th = dl.text_height(1.0);
        let t = if *value { 1.0 } else { 0.0 };
        let text_col = if *value || hover { s.text } else { s.text_dim };
        dl.text(x, (y + (row_h - th) * 0.5).floor(), shown, text_col, 1.0);

        let track = lerp_color(s.checkbox_bg, s.checkbox_mark, t);
        dl.rect_rounded(sw_x, sw_y, sw_w, sw_h, 3.0, track);
        if s.checkbox_border[3] != 0 {
            dl.outline_rounded(sw_x, sw_y, sw_w, sw_h, 3.0, 1.0, s.checkbox_border);
        }
        let knob_x = sw_x + 3.0 + (sw_w - 10.0 - 6.0) * t;
        let knob = lerp_color(s.text_dim, s.checkbox_mark_icon, t);
        dl.rect_rounded(knob_x, sw_y + 3.0, 10.0, 10.0, 2.0, knob);

        // Subtle dark hairline divider under the row (as in the C++ menu).
        dl.rect(x, y + row_h + 3.0, w, 1.0, [18, 18, 18, 255]);

        let clicked = hover && self.clicked;
        if clicked {
            *value = !*value;
        }
        self.mark_at(x, y, w, row_h, label);
        clicked
    }

    pub fn slider_f32(
        &mut self,
        dl: &mut DrawList,
        label: &str,
        value: &mut f32,
        min: f32,
        max: f32,
        fmt_decimals: usize,
        suffix: &str,
        s: &Style,
    ) -> bool {
        let (x, y) = self.next_pos();
        let w = self.row_w;
        let shown_label = display_label(label);
        let shown_val = match fmt_decimals {
            0 => format!("{:.0}{}", *value, suffix),
            1 => format!("{:.1}{}", *value, suffix),
            2 => format!("{:.2}{}", *value, suffix),
            3 => format!("{:.3}{}", *value, suffix),
            _ => format!("{:.4}{}", *value, suffix),
        };
        let th = dl.text_height(1.0);
        dl.text(x, y, shown_label, s.text_dim, 1.0);
        let vw = dl.text_width(&shown_val, 1.0);
        dl.text(x + w - vw, y, &shown_val, s.text, 1.0);

        let track_y = y + th + 5.0;
        let track_h = 6.0;
        dl.rect_rounded(x, track_y, w, track_h, 3.0, s.elevated);
        dl.outline_rounded(x, track_y, w, track_h, 3.0, 1.0, s.border);

        let ratio = if max > min {
            ((*value - min) / (max - min)).clamp(0.0, 1.0)
        } else {
            0.0
        };
        let fill_w = (w * ratio).max(0.0);
        if fill_w > 1.0 {
            dl.rect_rounded(x, track_y, fill_w, track_h, 3.0, s.slider_fill);
        }

        let total_h = th + 5.0 + track_h + 2.0;
        let hover = !self.overlay_blocking() && self.inside(x, y, w, total_h);
        if hover && self.clicked {
            self.active_slider = Some(label.to_string());
        }
        let mut changed = false;
        if self.active_slider.as_deref() == Some(label) && self.down {
            let px = ((self.mouse.x - x) / w).clamp(0.0, 1.0);
            let next = min + (max - min) * px;
            if (next - *value).abs() > f32::EPSILON {
                *value = next;
                changed = true;
            }
        }
        self.mark_at(x, y, w, total_h, label);
        changed
    }

    pub fn slider_i32(
        &mut self,
        dl: &mut DrawList,
        label: &str,
        value: &mut i32,
        min: i32,
        max: i32,
        suffix: &str,
        s: &Style,
    ) -> bool {
        let mut float = *value as f32;
        let changed = self.slider_f32(dl, label, &mut float, min as f32, max as f32, 0, suffix, s);
        if changed {
            *value = float.round().clamp(min as f32, max as f32) as i32;
        }
        changed
    }

    pub fn combo(
        &mut self,
        dl: &mut DrawList,
        label: &str,
        value: &mut usize,
        items: &[&str],
        s: &Style,
    ) -> bool {
        let (x, y) = self.next_pos();
        let w = self.row_w;
        let shown = display_label(label);
        let th = dl.text_height(1.0);
        let box_y = if shown.is_empty() {
            y
        } else {
            dl.text(x, y, shown, s.text_dim, 1.0);
            y + th + 4.0
        };
        let box_h = 22.0;
        let is_open = self.open_list.as_deref() == Some(label);
        let hover = (!self.overlay_blocking() || is_open) && self.inside(x, box_y, w, box_h);

        let bg = if hover || is_open { s.hovered } else { s.elevated };
        dl.rect_rounded(x, box_y, w, box_h, WIDGET_ROUNDING, bg);
        dl.outline_rounded(x, box_y, w, box_h, WIDGET_ROUNDING, 1.0, s.border);

        let current = items.get(*value).copied().unwrap_or("-");
        let trunc = dl.truncate(current, w - 24.0, 1.0);
        dl.text(x + 8.0, (box_y + (box_h - th) * 0.5).floor(), &trunc, s.text, 1.0);

        // Combo chevron arrow
        let ax = x + w - 12.0;
        let ay = box_y + box_h * 0.5;
        if is_open {
            dl.line(ax - 3.0, ay + 1.0, ax, ay - 2.0, 1.2, s.text_dim);
            dl.line(ax, ay - 2.0, ax + 3.0, ay + 1.0, 1.2, s.text_dim);
        } else {
            dl.line(ax - 3.0, ay - 1.0, ax, ay + 2.0, 1.2, s.text_dim);
            dl.line(ax, ay + 2.0, ax + 3.0, ay - 1.0, 1.2, s.text_dim);
        }

        if hover && self.clicked {
            self.open_list = if is_open { None } else { Some(label.to_string()) };
        }

        let total_h = (box_y - y) + box_h;
        self.mark_at(x, y, w, total_h, label);

        if self.open_list.as_deref() != Some(label) {
            return false;
        }

        let mut changed = false;
        let item_h = 22.0;
        let pop_y = box_y + box_h + 2.0;
        let pop_h = items.len() as f32 * item_h + 4.0;
        dl.begin_top();
        dl.rect_rounded(x, pop_y, w, pop_h, WIDGET_ROUNDING, s.dark);
        dl.outline_rounded(x, pop_y, w, pop_h, WIDGET_ROUNDING, 1.0, s.border);
        for (index, item) in items.iter().enumerate() {
            let iy = pop_y + 2.0 + index as f32 * item_h;
            let ihover = self.inside(x + 2.0, iy, w - 4.0, item_h);
            let isel = *value == index;
            if isel || ihover {
                dl.rect_rounded(
                    x + 2.0,
                    iy,
                    w - 4.0,
                    item_h,
                    2.0,
                    if isel { s.active } else { s.hovered },
                );
            }
            dl.text(
                x + 8.0,
                (iy + (item_h - th) * 0.5).floor(),
                item,
                if isel || ihover { s.text } else { s.text_dim },
                1.0,
            );
            if ihover && self.clicked {
                *value = index;
                changed = true;
                self.open_list = None;
            }
        }
        dl.end_top();
        if self.clicked && !self.inside(x, box_y, w, box_h) && !self.inside(x, pop_y, w, pop_h) {
            self.open_list = None;
        }
        changed
    }

    pub fn multicombo(
        &mut self,
        dl: &mut DrawList,
        label: &str,
        values: &mut [bool],
        items: &[&str],
        s: &Style,
    ) -> bool {
        let (x, y) = self.next_pos();
        let w = self.row_w;
        let shown = display_label(label);
        let th = dl.text_height(1.0);
        let box_y = if shown.is_empty() {
            y
        } else {
            dl.text(x, y, shown, s.text_dim, 1.0);
            y + th + 4.0
        };
        let box_h = 22.0;
        let is_open = self.open_list.as_deref() == Some(label);
        let hover = (!self.overlay_blocking() || is_open) && self.inside(x, box_y, w, box_h);

        let bg = if hover || is_open { s.hovered } else { s.elevated };
        dl.rect_rounded(x, box_y, w, box_h, WIDGET_ROUNDING, bg);
        dl.outline_rounded(x, box_y, w, box_h, WIDGET_ROUNDING, 1.0, s.border);

        let mut summary = String::new();
        for (i, &item) in items.iter().enumerate() {
            if values.get(i).copied().unwrap_or(false) {
                if !summary.is_empty() {
                    summary.push_str(", ");
                }
                summary.push_str(item);
            }
        }
        if summary.is_empty() {
            summary.push_str("none");
        }
        let trunc = dl.truncate(&summary, w - 24.0, 1.0);
        dl.text(
            x + 8.0,
            (box_y + (box_h - th) * 0.5).floor(),
            &trunc,
            if summary == "none" { s.text_dim } else { s.text },
            1.0,
        );

        let ax = x + w - 12.0;
        let ay = box_y + box_h * 0.5;
        dl.line(ax - 3.0, ay - 1.0, ax, ay + 2.0, 1.2, s.text_dim);
        dl.line(ax, ay + 2.0, ax + 3.0, ay - 1.0, 1.2, s.text_dim);

        if hover && self.clicked {
            self.open_list = if is_open { None } else { Some(label.to_string()) };
        }
        self.mark_at(x, y, w, (box_y - y) + box_h, label);

        if self.open_list.as_deref() != Some(label) {
            return false;
        }
        let mut changed = false;
        let item_h = 22.0;
        let pop_y = box_y + box_h + 2.0;
        let pop_h = items.len() as f32 * item_h + 4.0;
        dl.begin_top();
        dl.rect_rounded(x, pop_y, w, pop_h, WIDGET_ROUNDING, s.dark);
        dl.outline_rounded(x, pop_y, w, pop_h, WIDGET_ROUNDING, 1.0, s.border);
        for (index, item) in items.iter().enumerate() {
            let iy = pop_y + 2.0 + index as f32 * item_h;
            let ihover = self.inside(x + 2.0, iy, w - 4.0, item_h);
            let on = values.get(index).copied().unwrap_or(false);
            if ihover {
                dl.rect_rounded(x + 2.0, iy, w - 4.0, item_h, 2.0, s.hovered);
            }
            self.minimal_checkbox(dl, x + 6.0, iy + (item_h - 12.0) * 0.5, 12.0, on, s);
            dl.text(
                x + 26.0,
                (iy + (item_h - th) * 0.5).floor(),
                item,
                if on || ihover { s.text } else { s.text_dim },
                1.0,
            );
            if ihover && self.clicked {
                if let Some(slot) = values.get_mut(index) {
                    *slot = !*slot;
                    changed = true;
                }
            }
        }
        dl.end_top();
        if self.clicked && !self.inside(x, box_y, w, box_h) && !self.inside(x, pop_y, w, pop_h) {
            self.open_list = None;
        }
        changed
    }

    pub fn color_picker(
        &mut self,
        dl: &mut DrawList,
        label: &str,
        color: &mut [u8; 4],
        s: &Style,
    ) -> bool {
        let (x, y) = self.next_pos();
        let w = self.row_w;
        let row_h = 16.0;
        let sw_w = 24.0;
        let sw_h = 12.0;
        let is_open = self.open_color.as_deref() == Some(label);
        let hover = (!self.overlay_blocking() || is_open) && self.inside(x, y, w, row_h);
        let shown = display_label(label);
        let th = dl.text_height(1.0);
        dl.text(
            x,
            (y + (row_h - th) * 0.5).floor(),
            shown,
            if hover { s.text } else { s.text_dim },
            1.0,
        );
        let sx = x + w - sw_w;
        let sy = y + (row_h - sw_h) * 0.5;
        dl.rect_rounded(sx, sy, sw_w, sw_h, WIDGET_ROUNDING, *color);
        dl.outline_rounded(sx, sy, sw_w, sw_h, WIDGET_ROUNDING, 1.0, s.border);
        if hover && self.clicked {
            self.open_color = if is_open { None } else { Some(label.to_string()) };
        }
        self.mark_at(x, y, w, row_h, label);

        if self.open_color.as_deref() != Some(label) {
            return false;
        }
        let mut changed = false;
        let pop_w = 190.0;
        let pop_h = 128.0;
        let (sw, sh) = dl.size();
        let px = (x + w + 6.0).min(sw - pop_w - 6.0).max(6.0);
        let py = y.min(sh - pop_h - 6.0).max(6.0);
        dl.begin_top();
        dl.rect_rounded(px, py, pop_w, pop_h, WIDGET_ROUNDING, s.dark);
        dl.outline_rounded(px, py, pop_w, pop_h, WIDGET_ROUNDING, 1.0, s.border);
        let saved = (self.cursor, self.row_w, self.in_popup);
        self.cursor = Vec2::new(px + 10.0, py + 10.0);
        self.row_w = pop_w - 20.0;
        self.in_popup = true;
        let channels = ["red", "green", "blue", "alpha"];
        for (index, name) in channels.iter().enumerate() {
            let mut value = color[index] as f32;
            let cid = format!("{}##cp_{}_{}", name, label, index);
            if self.slider_f32(dl, &cid, &mut value, 0.0, 255.0, 0, "", s) {
                color[index] = value.round().clamp(0.0, 255.0) as u8;
                changed = true;
            }
        }
        self.cursor = saved.0;
        self.row_w = saved.1;
        self.in_popup = saved.2;
        dl.end_top();
        if self.clicked && !self.inside(x, y, w, row_h) && !self.inside(px, py, pop_w, pop_h) {
            self.open_color = None;
        }
        changed
    }

    pub fn text_input(
        &mut self,
        dl: &mut DrawList,
        label: &str,
        buf: &mut String,
        max_len: usize,
        placeholder: &str,
        s: &Style,
    ) -> bool {
        let (x, y) = self.next_pos();
        let w = self.row_w;
        let shown = display_label(label);
        let th = dl.text_height(1.0);
        let box_y = if shown.is_empty() {
            y
        } else {
            dl.text(x, y, shown, s.text_dim, 1.0);
            y + th + 4.0
        };
        let box_h = 24.0;
        let focused = self.active_text.as_deref() == Some(label);
        let hover = !self.overlay_blocking() && self.inside(x, box_y, w, box_h);
        if self.clicked {
            if hover {
                self.active_text = Some(label.to_string());
            } else if focused {
                self.active_text = None;
            }
        }
        let bg = if focused || hover { s.hovered } else { s.elevated };
        dl.rect_rounded(x, box_y, w, box_h, WIDGET_ROUNDING, bg);
        dl.outline_rounded(
            x,
            box_y,
            w,
            box_h,
            WIDGET_ROUNDING,
            1.0,
            if focused { s.accent } else { s.border },
        );

        let mut changed = false;
        if focused {
            // Poll basic ASCII keys + backspace
            let shift = key_down(0x10);
            for vk in 0x08u16..=0x5Au16 {
                let down = key_down(vk);
                let pressed = down && !self.prev_keys[vk as usize];
                self.prev_keys[vk as usize] = down;
                if !pressed {
                    continue;
                }
                if vk == 0x08 {
                    if buf.pop().is_some() {
                        changed = true;
                    }
                } else if vk == 0x20 && buf.len() < max_len {
                    buf.push(' ');
                    changed = true;
                } else if (0x30..=0x39).contains(&vk) && buf.len() < max_len {
                    buf.push(vk as u8 as char);
                    changed = true;
                } else if (0x41..=0x5A).contains(&vk) && buf.len() < max_len {
                    let ch = if shift {
                        vk as u8 as char
                    } else {
                        (vk as u8 + 32) as char
                    };
                    buf.push(ch);
                    changed = true;
                }
            }
        }

        let display_str = if buf.is_empty() { placeholder } else { buf.as_str() };
        let trunc = dl.truncate(display_str, w - 16.0, 1.0);
        let col = if buf.is_empty() { s.text_dim } else { s.text };
        dl.text(x + 8.0, (box_y + (box_h - th) * 0.5).floor(), &trunc, col, 1.0);

        self.mark_at(x, y, w, (box_y - y) + box_h, label);
        changed
    }

    /// Popup attached to the preceding checkbox (right-click OR click the 3-dots gear icon).
    pub fn popup<R>(
        &mut self,
        dl: &mut DrawList,
        id: &str,
        w: f32,
        h: f32,
        s: &Style,
        f: impl FnOnce(&mut Ui, &mut DrawList) -> R,
    ) -> Option<R> {
        let anchor = self.last_rect;
        if !self.last_id.is_empty() {
            self.popup_owners.insert(self.last_id.clone());
        }
        let gear_hit = self.clicked
            && self.inside(anchor.0 + anchor.2 - 18.0, anchor.1, 18.0, anchor.3);
        let rmb_hit = self.right_clicked && self.inside(anchor.0, anchor.1, anchor.2, anchor.3);

        let is_nested = self.in_popup;
        if gear_hit || rmb_hit {
            if is_nested {
                if self.open_sub_popup.as_deref() == Some(id) {
                    self.open_sub_popup = None;
                } else {
                    self.open_sub_popup = Some(id.to_string());
                }
            } else if self.open_popup.as_deref() == Some(id) {
                self.open_popup = None;
                self.open_sub_popup = None;
            } else {
                self.open_popup = Some(id.to_string());
                self.open_sub_popup = None;
            }
        }
        let is_open = if is_nested {
            self.open_sub_popup.as_deref() == Some(id)
        } else {
            self.open_popup.as_deref() == Some(id)
        };
        if !is_open {
            return None;
        }

        let (screen_w, screen_h) = dl.size();
        // Drop-down style: right-aligned under the row, kept inside the screen.
        let x = (anchor.0 + anchor.2 - w)
            .max(anchor.0)
            .min(screen_w - w - 6.0)
            .max(6.0);
        let y = (anchor.1 + anchor.3 + 4.0)
            .min(screen_h - h - 6.0)
            .max(6.0);
        dl.begin_top();
        dl.rect_rounded(x, y, w, h, WIDGET_ROUNDING, s.dark);
        dl.outline_rounded(x, y, w, h, WIDGET_ROUNDING, 1.0, s.border);
        let saved = (self.cursor, self.row_w, self.in_popup);
        self.cursor = Vec2::new(x + 10.0, y + 10.0);
        self.row_w = w - 20.0;
        self.in_popup = true;
        let out = f(self, dl);
        self.cursor = saved.0;
        self.row_w = saved.1;
        self.in_popup = saved.2;
        dl.end_top();

        if self.clicked
            && !self.inside(x, y, w, h)
            && !self.inside(anchor.0, anchor.1, anchor.2, anchor.3)
            && (is_nested || self.open_sub_popup.is_none())
        {
            if is_nested {
                self.open_sub_popup = None;
            } else {
                self.open_popup = None;
                self.open_sub_popup = None;
            }
        }
        Some(out)
    }
}
