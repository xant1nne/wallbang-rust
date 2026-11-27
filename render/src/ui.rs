// Config block: 1:1 порт xui поверх DrawList (без доступа к памяти игры).
use crate::draw::{Color, DrawList};
use wallbang_config::collections::{HashMap, HashSet};
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
    pub surface: Color,
    pub popup_shadow: Color,
    pub rarity: [Color; 7],
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
            surface: [16, 17, 20, 255],
            popup_shadow: [0, 0, 0, 88],
            rarity: [
                [176, 195, 217, 255], // Consumer
                [94, 152, 217, 255],  // Industrial
                [75, 105, 255, 255],  // Mil-Spec
                [136, 71, 255, 255],  // Restricted
                [211, 44, 230, 255],  // Classified
                [235, 75, 75, 255],   // Covert
                [228, 174, 57, 255],  // Contraband / Gold
            ],
        }
    }

    #[inline]
    pub fn rarity_color(&self, tier: u8) -> Color {
        self.rarity[(tier as usize).min(self.rarity.len() - 1)]
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

/// Pure RGB [0..255] -> HSV ([0..360), [0..1], [0..1]).
pub fn rgb_to_hsv(r: u8, g: u8, b: u8) -> [f32; 3] {
    let rf = r as f32 / 255.0;
    let gf = g as f32 / 255.0;
    let bf = b as f32 / 255.0;
    let max = rf.max(gf).max(bf);
    let min = rf.min(gf).min(bf);
    let delta = max - min;
    let h = if delta <= 1e-6 {
        0.0
    } else if (max - rf).abs() <= 1e-6 {
        60.0 * (((gf - bf) / delta).rem_euclid(6.0))
    } else if (max - gf).abs() <= 1e-6 {
        60.0 * (((bf - rf) / delta) + 2.0)
    } else {
        60.0 * (((rf - gf) / delta) + 4.0)
    };
    let s = if max <= 1e-6 { 0.0 } else { delta / max };
    [h.rem_euclid(360.0), s.clamp(0.0, 1.0), max.clamp(0.0, 1.0)]
}

/// Pure HSV ([0..360), [0..1], [0..1]) -> RGB [0..255].
pub fn hsv_to_rgb(h: f32, s: f32, v: f32) -> [u8; 3] {
    let h = h.rem_euclid(360.0);
    let s = s.clamp(0.0, 1.0);
    let v = v.clamp(0.0, 1.0);
    let c = v * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = v - c;
    let (r, g, b) = if h < 60.0 {
        (c, x, 0.0)
    } else if h < 120.0 {
        (x, c, 0.0)
    } else if h < 180.0 {
        (0.0, c, x)
    } else if h < 240.0 {
        (0.0, x, c)
    } else if h < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };
    [
        ((r + m) * 255.0).round().clamp(0.0, 255.0) as u8,
        ((g + m) * 255.0).round().clamp(0.0, 255.0) as u8,
        ((b + m) * 255.0).round().clamp(0.0, 255.0) as u8,
    ]
}

fn draw_checkerboard(dl: &mut DrawList, x: f32, y: f32, w: f32, h: f32, cell: f32) {
    let light: Color = [150, 150, 156, 255];
    let dark: Color = [92, 92, 98, 255];
    dl.rect(x, y, w, h, dark);
    let cols = (w / cell).ceil() as i32;
    let rows = (h / cell).ceil() as i32;
    for r in 0..rows {
        let cy = y + r as f32 * cell;
        let ch = (y + h - cy).min(cell);
        if ch <= 0.0 { break; }
        for c in 0..cols {
            if (r + c) & 1 == 0 {
                let cx = x + c as f32 * cell;
                let cw = (x + w - cx).min(cell);
                if cw > 0.0 {
                    dl.rect(cx, cy, cw, ch, light);
                }
            }
        }
    }
}

fn format_u8_buf(v: u8, buf: &mut [u8; 4]) -> &str {
    let mut n = v;
    let mut i = 4usize;
    if n == 0 {
        buf[3] = b'0';
        return std::str::from_utf8(&buf[3..4]).unwrap_or("0");
    }
    while n > 0 && i > 0 {
        i -= 1;
        buf[i] = b'0' + (n % 10);
        n /= 10;
    }
    std::str::from_utf8(&buf[i..4]).unwrap_or("0")
}

fn write_hex_rgba(c: [u8; 4], out: &mut [u8; 8]) {
    const HEX: &[u8; 16] = b"0123456789ABCDEF";
    for i in 0..4 {
        out[i * 2] = HEX[(c[i] >> 4) as usize];
        out[i * 2 + 1] = HEX[(c[i] & 0xF) as usize];
    }
}

fn parse_hex_rgba(buf: &[u8], len: usize, fallback_a: u8) -> Option<[u8; 4]> {
    let hex_val = |b: u8| -> Option<u8> {
        match b {
            b'0'..=b'9' => Some(b - b'0'),
            b'A'..=b'F' => Some(b - b'A' + 10),
            b'a'..=b'f' => Some(b - b'a' + 10),
            _ => None,
        }
    };
    if len != 6 && len != 8 {
        return None;
    }
    let byte = |idx: usize| -> Option<u8> {
        Some((hex_val(buf[idx])? << 4) | hex_val(buf[idx + 1])?)
    };
    let r = byte(0)?;
    let g = byte(2)?;
    let b = byte(4)?;
    let a = if len == 8 { byte(6)? } else { fallback_a };
    Some([r, g, b, a])
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
    list_first: HashMap<String, usize>,
    open_popup: Option<String>,
    open_sub_popup: Option<String>,
    open_color: Option<String>,
    color_hsv: [f32; 3],
    color_last_rgb: [u8; 3],
    color_drag: u8,
    color_hex_buf: [u8; 8],
    color_hex_len: usize,
    color_hex_focus: bool,
    wear_input_id: Option<String>,
    wear_input_buf: [u8; 10],
    wear_input_len: usize,
    esc_pressed: bool,
    popup_owners: HashSet<String>,
    in_popup: bool,
    dragging_window: bool,
    drag_offset: Vec2,
    menu_origin: Vec2,
    clip_rect: Option<(f32, f32, f32, f32)>,
    prev_keys: [bool; 256],
    scroll: HashMap<u64, f32>,
    scroll_max: HashMap<u64, f32>,
    /// Popups grow to fit their content. The first frame uses the caller's
    /// hint; every later frame reuses the measured height.
    popup_heights: HashMap<String, f32>,
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
            list_first: HashMap::default(),
            open_popup: None,
            open_sub_popup: None,
            open_color: None,
            color_hsv: [0.0, 1.0, 1.0],
            color_last_rgb: [255, 0, 0],
            color_drag: 0,
            color_hex_buf: *b"FFFFFFFF",
            color_hex_len: 8,
            color_hex_focus: false,
            wear_input_id: None,
            wear_input_buf: [0; 10],
            wear_input_len: 0,
            esc_pressed: false,
            popup_owners: HashSet::default(),
            in_popup: false,
            dragging_window: false,
            drag_offset: Vec2::default(),
            menu_origin: Vec2::default(),
            clip_rect: None,
            prev_keys: [false; 256],
            scroll: HashMap::default(),
            scroll_max: HashMap::default(),
            popup_heights: HashMap::default(),
            wheel: 0.0,
        }
    }

    pub fn begin_frame(&mut self) {
        let cursor = cursor_position().map(|p| Vec2::new(p.x as f32, p.y as f32));
        self.begin_frame_with_cursor(cursor);
    }

    pub fn begin_frame_with_cursor(&mut self, cursor: Option<Vec2>) {
        self.prev_mouse = self.mouse;
        self.mouse = cursor.unwrap_or(Vec2::new(-10_000.0, -10_000.0));
        self.down = key_down(0x01);
        self.clicked = if crate::hooks::has_menu_input() {
            crate::hooks::take_menu_click()
        } else {
            self.down && !self.prev_down
        };
        if !self.down {
            self.active_slider = None;
            self.dragging_window = false;
            self.color_drag = 0;
        }
        let esc = key_down(0x1B);
        self.esc_pressed = esc && !self.prev_keys[0x1B];
        self.prev_keys[0x1B] = esc;
        if self.esc_pressed {
            if self.open_color.is_some() {
                self.open_color = None;
                self.color_hex_focus = false;
                self.color_drag = 0;
            } else if self.open_list.is_some() {
                self.open_list = None;
            } else if self.wear_input_id.is_some() {
                self.wear_input_id = None;
            }
        }
        self.prev_down = self.down;
        let right = key_down(0x02);
        self.right_clicked = right && !self.prev_right;
        self.prev_right = right;
        self.same_line_next = false;
        self.in_popup = false;
        self.clip_rect = None;
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

    #[cfg(test)]
    pub(crate) fn test_pointer(&mut self, x: f32, y: f32, clicked: bool, wheel: f32) {
        self.set_synthetic_mouse(x,y);
        self.clicked = clicked; self.down = clicked; self.wheel = wheel;
    }

    #[inline]
    pub fn mouse(&self) -> Vec2 {
        self.mouse
    }

    pub fn take_wheel_at(&mut self, rect: (f32, f32, f32, f32)) -> f32 {
        if !self.overlay_blocking() && self.inside(rect.0, rect.1, rect.2, rect.3) {
            std::mem::take(&mut self.wheel)
        } else { 0.0 }
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
        if self.in_popup {
            self.open_sub_popup.is_some()
                || self.open_list.is_some()
                || self.open_color.is_some()
        } else {
            self.open_popup.is_some()
                || self.open_sub_popup.is_some()
                || self.open_list.is_some()
                || self.open_color.is_some()
        }
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
        if !self.in_popup {
            if let Some((cx, cy, cw, ch)) = self.clip_rect {
                if self.mouse.x < cx || self.mouse.x > cx + cw || self.mouse.y < cy || self.mouse.y > cy + ch {
                    return false;
                }
            }
        }
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
        self.menu_origin = Vec2::new(*mx, *my);
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
        let key = Self::child_key(x - self.menu_origin.x, y - self.menu_origin.y, w, h);
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
        let saved_clip = self.clip_rect;
        self.clip_rect = Some((x + 1.0, y + 1.0, (w - 2.0).max(0.0), (h - 2.0).max(0.0)));
        dl.push_clip(x + 1.0, y + 1.0, (w - 2.0).max(0.0), (h - 2.0).max(0.0));
        let out = f(self, dl);
        self.clip_rect = saved_clip;
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

        let gear = self.popup_owners.contains(label) && self.inside(x + w - 52.0, y, 20.0, row_h);
        let clicked = hover && self.clicked && !gear;
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

    pub fn wear_slider(
        &mut self,
        dl: &mut DrawList,
        label: &str,
        value: &mut f32,
        s: &Style,
    ) -> bool {
        let (x, y) = self.next_pos();
        let w = self.row_w;
        let th = dl.text_height(1.0);
        let mut changed = false;

        let shown = display_label(label);
        if !shown.is_empty() {
            dl.text(x, y, shown, s.text, 1.0);
        }

        let wear = value.clamp(0.0, 1.0);
        let tier_idx: usize = if wear < 0.07 {
            0
        } else if wear < 0.15 {
            1
        } else if wear < 0.38 {
            2
        } else if wear < 0.45 {
            3
        } else {
            4
        };
        const TIER_FULL: [&str; 5] = [
            "Factory New",
            "Minimal Wear",
            "Field-Tested",
            "Well-Worn",
            "Battle-Scarred",
        ];
        const TIER_SHORT: [&str; 5] = ["FN", "MW", "FT", "WW", "BS"];
        const TIER_POS: [f32; 5] = [0.035, 0.11, 0.265, 0.415, 0.725];
        const TIER_PRESET: [f32; 5] = [0.01, 0.09, 0.20, 0.40, 0.60];

        // Numeric input box on the right
        let num_w = 54.0;
        let num_h = 18.0;
        let num_x = x + w - num_w;
        let num_y = y - 1.0;
        let is_editing = self.wear_input_id.as_deref() == Some(label);
        let num_hover = !self.overlay_blocking() && self.inside(num_x, num_y, num_w, num_h);

        // Tier name to the left of the numeric box
        let tier_name = TIER_FULL[tier_idx];
        let tier_w = dl.text_width(tier_name, 0.92);
        let tier_x = (num_x - tier_w - 8.0).max(x + dl.text_width(shown, 1.0) + 8.0);
        if tier_x + tier_w <= num_x - 4.0 {
            dl.text(tier_x, y + 0.5, tier_name, s.text_dim, 0.92);
        }

        if num_hover && self.clicked && !is_editing {
            self.wear_input_id = Some(label.to_string());
            let scaled = (wear * 10000.0).round() as u32;
            let whole = scaled / 10000;
            let frac = scaled % 10000;
            self.wear_input_buf[0] = b'0' + (whole.min(9) as u8);
            self.wear_input_buf[1] = b'.';
            self.wear_input_buf[2] = b'0' + ((frac / 1000) % 10) as u8;
            self.wear_input_buf[3] = b'0' + ((frac / 100) % 10) as u8;
            self.wear_input_buf[4] = b'0' + ((frac / 10) % 10) as u8;
            self.wear_input_buf[5] = b'0' + (frac % 10) as u8;
            self.wear_input_len = 6;
        } else if is_editing && self.clicked && !num_hover {
            if let Ok(text_str) = core::str::from_utf8(&self.wear_input_buf[..self.wear_input_len]) {
                if let Ok(parsed) = text_str.parse::<f32>() {
                    *value = parsed.clamp(0.0, 1.0);
                    changed = true;
                }
            }
            self.wear_input_id = None;
        }

        if self.wear_input_id.as_deref() == Some(label) {
            for vk in 0x30u8..=0x39u8 {
                let down = key_down(vk as u16);
                if down && !self.prev_keys[vk as usize] && self.wear_input_len < 8 {
                    self.wear_input_buf[self.wear_input_len] = vk;
                    self.wear_input_len += 1;
                }
                self.prev_keys[vk as usize] = down;
            }
            // '.' key (VK_OEM_PERIOD = 0xBE, VK_DECIMAL = 0x6E)
            for vk in [0xBEusize, 0x6Eusize] {
                let down = key_down(vk as u16);
                let has_dot = self.wear_input_buf[..self.wear_input_len].contains(&b'.');
                if down && !self.prev_keys[vk] && !has_dot && self.wear_input_len < 8 {
                    self.wear_input_buf[self.wear_input_len] = b'.';
                    self.wear_input_len += 1;
                }
                self.prev_keys[vk] = down;
            }
            // Backspace
            let bs = key_down(0x08);
            if bs && !self.prev_keys[0x08] && self.wear_input_len > 0 {
                self.wear_input_len -= 1;
            }
            self.prev_keys[0x08] = bs;
            // Enter
            let enter = key_down(0x0D);
            if enter && !self.prev_keys[0x0D] {
                if let Ok(text_str) = core::str::from_utf8(&self.wear_input_buf[..self.wear_input_len]) {
                    if let Ok(parsed) = text_str.parse::<f32>() {
                        *value = parsed.clamp(0.0, 1.0);
                        changed = true;
                    }
                }
                self.wear_input_id = None;
            }
            self.prev_keys[0x0D] = enter;
        }

        let editing_now = self.wear_input_id.as_deref() == Some(label);
        let num_bg = if editing_now { s.active } else if num_hover { s.hovered } else { s.elevated };
        let num_border = if editing_now { s.accent } else { s.border };
        dl.rect_rounded(num_x, num_y, num_w, num_h, 3.0, num_bg);
        dl.outline_rounded(num_x, num_y, num_w, num_h, 3.0, 1.0, num_border);

        let mut stack_buf = [0u8; 8];
        let num_str = if editing_now {
            core::str::from_utf8(&self.wear_input_buf[..self.wear_input_len]).unwrap_or("")
        } else {
            let scaled = (value.clamp(0.0, 1.0) * 10000.0).round() as u32;
            let whole = scaled / 10000;
            let frac = scaled % 10000;
            stack_buf[0] = b'0' + (whole.min(9) as u8);
            stack_buf[1] = b'.';
            stack_buf[2] = b'0' + ((frac / 1000) % 10) as u8;
            stack_buf[3] = b'0' + ((frac / 100) % 10) as u8;
            stack_buf[4] = b'0' + ((frac / 10) % 10) as u8;
            stack_buf[5] = b'0' + (frac % 10) as u8;
            core::str::from_utf8(&stack_buf[..6]).unwrap_or("0.0000")
        };
        let ntw = dl.text_width(num_str, 0.9);
        dl.text(
            num_x + (num_w - ntw) * 0.5,
            num_y + (num_h - th * 0.9) * 0.5,
            num_str,
            s.text,
            0.9,
        );

        // Gradient wear bar
        let bar_y = y + th + 10.0;
        let bar_h = 5.0;
        let stops: [(f32, Color); 6] = [
            (0.00, [68, 194, 114, 255]),
            (0.07, [116, 206, 94, 255]),
            (0.15, [208, 208, 72, 255]),
            (0.38, [236, 174, 62, 255]),
            (0.45, [230, 108, 60, 255]),
            (1.00, [212, 62, 66, 255]),
        ];
        for i in 0..5 {
            let sx0 = x + w * stops[i].0;
            let sx1 = x + w * stops[i + 1].0;
            let c0 = stops[i].1;
            let c1 = stops[i + 1].1;
            dl.rect_multicolor(sx0, bar_y, sx1 - sx0, bar_h, c0, c1, c1, c0);
        }
        dl.outline(x, bar_y, w, bar_h, 1.0, [0, 0, 0, 120]);
        for &t in &[0.07f32, 0.15, 0.38, 0.45] {
            let tx = x + w * t;
            dl.line(tx, bar_y, tx, bar_y + bar_h, 1.0, [14, 16, 20, 160]);
        }

        // Drag interaction on bar + triangle zone
        let hit_y = bar_y - 7.0;
        let hit_h = bar_h + 12.0;
        let bar_hover = !self.overlay_blocking() && self.inside(x, hit_y, w, hit_h);
        if bar_hover && self.clicked {
            self.active_slider = Some(label.to_string());
            self.wear_input_id = None;
        }
        if self.active_slider.as_deref() == Some(label) && self.down {
            let t = ((self.mouse.x - x) / w.max(1.0)).clamp(0.0, 1.0);
            if (*value - t).abs() > 1e-5 {
                *value = t;
                changed = true;
            }
        }

        // Triangle thumb pointing down onto the bar
        let thumb_x = (x + value.clamp(0.0, 1.0) * w).clamp(x + 3.0, x + w - 3.0);
        dl.convex_poly(
            &[
                (thumb_x - 4.5, bar_y - 6.5),
                (thumb_x + 4.5, bar_y - 6.5),
                (thumb_x, bar_y - 1.0),
            ],
            s.text,
        );

        // Threshold labels FN / MW / FT / WW / BS
        let lbl_y = bar_y + bar_h + 4.0;
        let lbl_scale = 0.85;
        let lbl_h = dl.text_height(lbl_scale);
        for i in 0..5 {
            let tw = dl.text_width(TIER_SHORT[i], lbl_scale);
            let lx = (x + w * TIER_POS[i] - tw * 0.5).clamp(x, x + w - tw);
            let active_tier = i == tier_idx;
            let lhover = !self.overlay_blocking() && self.inside(lx - 2.0, lbl_y, tw + 4.0, lbl_h + 2.0);
            let col = if active_tier || lhover { s.text } else { s.text_dim };
            dl.text(lx, lbl_y, TIER_SHORT[i], col, lbl_scale);
            if lhover && self.clicked {
                *value = TIER_PRESET[i];
                self.wear_input_id = None;
                changed = true;
            }
        }

        let total_h = (lbl_y + lbl_h) - y + 2.0;
        self.mark_at(x, y, w, total_h, label);
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
        let border = if is_open {
            [s.accent[0], s.accent[1], s.accent[2], 160]
        } else {
            s.border
        };
        dl.rect_rounded(x, box_y, w, box_h, WIDGET_ROUNDING, bg);
        dl.outline_rounded(x, box_y, w, box_h, WIDGET_ROUNDING, 1.0, border);

        let current = items.get(*value).copied().unwrap_or("-");
        let trunc = dl.truncate(current, w - 24.0, 1.0);
        dl.text(x + 8.0, (box_y + (box_h - th) * 0.5).floor(), &trunc, s.text, 1.0);

        // Combo chevron arrow
        let ax = x + w - 12.0;
        let ay = box_y + box_h * 0.5;
        let arrow_col = if is_open { s.accent } else { s.text_dim };
        if is_open {
            dl.line(ax - 3.0, ay + 1.0, ax, ay - 2.0, 1.2, arrow_col);
            dl.line(ax, ay - 2.0, ax + 3.0, ay + 1.0, 1.2, arrow_col);
        } else {
            dl.line(ax - 3.0, ay - 1.0, ax, ay + 2.0, 1.2, arrow_col);
            dl.line(ax, ay + 2.0, ax + 3.0, ay - 1.0, 1.2, arrow_col);
        }

        if hover && self.clicked {
            if is_open {
                self.open_list = None;
            } else {
                self.open_color = None;
                self.color_hex_focus = false;
                self.color_drag = 0;
                self.open_list = Some(label.to_string());
                self.list_first.insert(label.to_owned(), value.saturating_sub(3));
            }
        }

        let total_h = (box_y - y) + box_h;
        self.mark_at(x, y, w, total_h, label);

        if self.open_list.as_deref() != Some(label) {
            return false;
        }

        let mut changed = false;
        let item_h = 24.0;
        let rows = items.len().min(8);
        let pad = 4.0;
        let pop_h = rows as f32 * item_h + pad * 2.0;
        let below = box_y + box_h + 3.0;
        let pop_y = if below + pop_h <= dl.size().1 - 6.0 {
            below
        } else {
            (box_y - pop_h - 3.0).max(6.0)
        };
        let max_first = items.len().saturating_sub(rows);
        let mut first = self.list_first.get(label).copied().unwrap_or(0).min(max_first);
        if self.inside(x, pop_y, w, pop_h) && self.wheel != 0.0 {
            first = (first as f32 - self.wheel * 3.0).clamp(0.0, max_first as f32) as usize;
            self.wheel = 0.0;
        }
        self.list_first.insert(label.to_owned(), first);
        let pop_r = 5.0;
        dl.begin_top();
        // Soft popup drop shadow
        dl.rect_rounded(x - 2.0, pop_y + 3.0, w + 4.0, pop_h + 4.0, pop_r + 2.0, [0, 0, 0, 72]);
        dl.rect_rounded(x - 1.0, pop_y + 1.0, w + 2.0, pop_h + 2.0, pop_r + 1.0, [0, 0, 0, 96]);
        dl.rect_rounded(x, pop_y, w, pop_h, pop_r, s.dark);
        dl.outline_rounded(x, pop_y, w, pop_h, pop_r, 1.0, s.border);
        let text_max_w = if max_first > 0 { w - 24.0 } else { w - 18.0 };
        for (index, item) in items.iter().enumerate().skip(first).take(rows) {
            let iy = pop_y + pad + (index - first) as f32 * item_h;
            let ihover = self.inside(x + pad, iy, w - pad * 2.0, item_h);
            let isel = *value == index;
            if isel || ihover {
                let row_bg = if isel {
                    [s.accent[0], s.accent[1], s.accent[2], 34]
                } else {
                    s.hovered
                };
                dl.rect_rounded(x + pad, iy + 1.0, w - pad * 2.0, item_h - 2.0, 3.0, row_bg);
            }
            if isel {
                dl.rect_rounded(x + pad + 2.0, iy + 5.0, 2.0, item_h - 10.0, 1.0, s.accent);
            }
            let item_trunc = dl.truncate(item, text_max_w, 1.0);
            dl.text(
                x + 11.0,
                (iy + (item_h - th) * 0.5).floor(),
                &item_trunc,
                if isel || ihover { s.text } else { s.text_dim },
                1.0,
            );
            if ihover && self.clicked {
                *value = index;
                changed = true;
                self.open_list = None;
            }
        }
        if max_first > 0 {
            let track = pop_h - 10.0;
            let thumb = (track * rows as f32 / items.len() as f32).max(14.0);
            let offset = (track - thumb) * first as f32 / max_first as f32;
            dl.rect_rounded(x + w - 6.0, pop_y + 5.0, 2.0, track, 1.0, [255, 255, 255, 18]);
            dl.rect_rounded(x + w - 6.0, pop_y + 5.0 + offset, 2.0, thumb, 1.0, s.text_dim);
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
        let row_h = 24.0;
        let sw_w = 28.0;
        let sw_h = 14.0;
        let is_open = self.open_color.as_deref() == Some(label);
        let hover = (!self.overlay_blocking() || is_open) && self.inside(x, y, w, row_h);
        let shown = display_label(label);
        let th = dl.text_height(1.0);
        dl.text(
            x,
            (y + (row_h - th) * 0.5).floor(),
            shown,
            if hover || is_open { s.text } else { s.text_dim },
            1.0,
        );
        let sx = x + w - sw_w;
        let sy = y + (row_h - sw_h) * 0.5;
        draw_checkerboard(dl, sx + 1.0, sy + 1.0, sw_w - 2.0, sw_h - 2.0, 4.0);
        dl.rect_rounded(sx, sy, sw_w, sw_h, WIDGET_ROUNDING, *color);
        dl.outline_rounded(
            sx,
            sy,
            sw_w,
            sw_h,
            WIDGET_ROUNDING,
            1.0,
            if is_open { s.accent } else { s.border },
        );
        if hover && self.clicked {
            if is_open {
                self.open_color = None;
                self.color_hex_focus = false;
                self.color_drag = 0;
            } else {
                self.open_list = None;
                self.open_sub_popup = None;
                self.open_color = Some(label.to_string());
                self.color_hsv = rgb_to_hsv(color[0], color[1], color[2]);
                self.color_last_rgb = [color[0], color[1], color[2]];
                self.color_drag = 0;
                self.color_hex_focus = false;
                write_hex_rgba(*color, &mut self.color_hex_buf);
                self.color_hex_len = 8;
            }
        }
        self.mark_at(x, y, w, row_h, label);

        if self.open_color.as_deref() != Some(label) {
            return false;
        }
        if self.color_drag == 0 && self.color_last_rgb != [color[0], color[1], color[2]] {
            self.color_hsv = rgb_to_hsv(color[0], color[1], color[2]);
            self.color_last_rgb = [color[0], color[1], color[2]];
            if !self.color_hex_focus {
                write_hex_rgba(*color, &mut self.color_hex_buf);
                self.color_hex_len = 8;
            }
        }

        let mut changed = false;
        let pop_w = 224.0;
        let pop_h = 242.0;
        let (sw, sh) = dl.size();
        let px = x.min((sw - pop_w - 6.0).max(6.0)).max(6.0);
        let mut py = y + row_h + 4.0;
        if py + pop_h > sh - 6.0 {
            py = y - pop_h - 4.0;
        }
        let py = py.min((sh - pop_h - 6.0).max(6.0)).max(6.0);

        dl.begin_top();
        // Soft drop shadow + panel
        dl.rect_rounded(px + 3.0, py + 4.0, pop_w, pop_h, ROUNDING, [0, 0, 0, 110]);
        dl.rect_rounded(px, py, pop_w, pop_h, ROUNDING, s.dark);
        dl.outline_rounded(px, py, pop_w, pop_h, ROUNDING, 1.0, s.border);

        let pad = 10.0;
        let inner_w = pop_w - pad * 2.0;
        let sv_x = px + pad;
        let sv_y = py + pad;
        let sv_w = inner_w;
        let sv_h = 124.0;

        // SV square
        let hue_rgb = hsv_to_rgb(self.color_hsv[0], 1.0, 1.0);
        let pure_hue: Color = [hue_rgb[0], hue_rgb[1], hue_rgb[2], 255];
        dl.rect_multicolor(sv_x, sv_y, sv_w, sv_h, [255, 255, 255, 255], pure_hue, pure_hue, [255, 255, 255, 255]);
        dl.rect_multicolor(sv_x, sv_y, sv_w, sv_h, [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 255], [0, 0, 0, 255]);
        dl.outline(sv_x, sv_y, sv_w, sv_h, 1.0, s.border);

        if self.clicked && self.inside(sv_x, sv_y, sv_w, sv_h) {
            self.color_drag = 1;
            self.color_hex_focus = false;
        }
        if self.color_drag == 1 && self.down {
            let ns = ((self.mouse.x - sv_x) / sv_w).clamp(0.0, 1.0);
            let nv = (1.0 - (self.mouse.y - sv_y) / sv_h).clamp(0.0, 1.0);
            self.color_hsv[1] = ns;
            self.color_hsv[2] = nv;
            let rgb = hsv_to_rgb(self.color_hsv[0], self.color_hsv[1], self.color_hsv[2]);
            color[0] = rgb[0];
            color[1] = rgb[1];
            color[2] = rgb[2];
            self.color_last_rgb = rgb;
            write_hex_rgba(*color, &mut self.color_hex_buf);
            self.color_hex_len = 8;
            changed = true;
        }
        let cx = sv_x + self.color_hsv[1] * sv_w;
        let cy = sv_y + (1.0 - self.color_hsv[2]) * sv_h;
        dl.circle_filled(cx, cy, 4.5, [0, 0, 0, 200]);
        dl.circle_filled(cx, cy, 3.0, [255, 255, 255, 255]);

        // Hue bar (6 segments)
        let hue_y = sv_y + sv_h + 8.0;
        let bar_h = 12.0;
        const HUE_STOPS: [Color; 7] = [
            [255, 0, 0, 255],
            [255, 255, 0, 255],
            [0, 255, 0, 255],
            [0, 255, 255, 255],
            [0, 0, 255, 255],
            [255, 0, 255, 255],
            [255, 0, 0, 255],
        ];
        let seg_w = sv_w / 6.0;
        for i in 0..6 {
            let x0 = sv_x + i as f32 * seg_w;
            dl.rect_multicolor(x0, hue_y, seg_w + 0.5, bar_h, HUE_STOPS[i], HUE_STOPS[i + 1], HUE_STOPS[i + 1], HUE_STOPS[i]);
        }
        dl.outline(sv_x, hue_y, sv_w, bar_h, 1.0, s.border);
        if self.clicked && self.inside(sv_x, hue_y, sv_w, bar_h) {
            self.color_drag = 2;
            self.color_hex_focus = false;
        }
        if self.color_drag == 2 && self.down {
            let nh = (((self.mouse.x - sv_x) / sv_w).clamp(0.0, 1.0)) * 359.99;
            self.color_hsv[0] = nh;
            let rgb = hsv_to_rgb(self.color_hsv[0], self.color_hsv[1], self.color_hsv[2]);
            color[0] = rgb[0];
            color[1] = rgb[1];
            color[2] = rgb[2];
            self.color_last_rgb = rgb;
            write_hex_rgba(*color, &mut self.color_hex_buf);
            self.color_hex_len = 8;
            changed = true;
        }
        let hx = sv_x + (self.color_hsv[0] / 360.0).clamp(0.0, 1.0) * sv_w;
        dl.rect(hx - 2.0, hue_y - 1.0, 4.0, bar_h + 2.0, [255, 255, 255, 255]);
        dl.outline(hx - 2.0, hue_y - 1.0, 4.0, bar_h + 2.0, 1.0, [0, 0, 0, 200]);

        // Alpha bar
        let alpha_y = hue_y + bar_h + 8.0;
        draw_checkerboard(dl, sv_x, alpha_y, sv_w, bar_h, 4.0);
        let a0: Color = [color[0], color[1], color[2], 0];
        let a1: Color = [color[0], color[1], color[2], 255];
        dl.rect_multicolor(sv_x, alpha_y, sv_w, bar_h, a0, a1, a1, a0);
        dl.outline(sv_x, alpha_y, sv_w, bar_h, 1.0, s.border);
        if self.clicked && self.inside(sv_x, alpha_y, sv_w, bar_h) {
            self.color_drag = 3;
            self.color_hex_focus = false;
        }
        if self.color_drag == 3 && self.down {
            let na = (((self.mouse.x - sv_x) / sv_w).clamp(0.0, 1.0) * 255.0).round() as u8;
            color[3] = na;
            write_hex_rgba(*color, &mut self.color_hex_buf);
            self.color_hex_len = 8;
            changed = true;
        }
        let ax = sv_x + (color[3] as f32 / 255.0) * sv_w;
        dl.rect(ax - 2.0, alpha_y - 1.0, 4.0, bar_h + 2.0, [255, 255, 255, 255]);
        dl.outline(ax - 2.0, alpha_y - 1.0, 4.0, bar_h + 2.0, 1.0, [0, 0, 0, 200]);

        // R G B A scrubbable fields (zero heap allocations)
        let rgba_y = alpha_y + bar_h + 10.0;
        let cell_gap = 4.0;
        let cell_w = (sv_w - cell_gap * 3.0) * 0.25;
        let cell_h = 22.0;
        const CH_LABELS: [&str; 4] = ["R", "G", "B", "A"];
        let mut num_buf = [0u8; 4];
        for i in 0..4 {
            let bx = sv_x + i as f32 * (cell_w + cell_gap);
            let hov = self.inside(bx, rgba_y, cell_w, cell_h);
            let drag_id = 4 + i as u8;
            if self.clicked && hov {
                self.color_drag = drag_id;
                self.color_hex_focus = false;
            }
            if self.color_drag == drag_id && self.down {
                let dx = self.mouse_delta().x;
                if dx.abs() > 0.0 {
                    let next = (color[i] as f32 + dx * 1.5).round().clamp(0.0, 255.0) as u8;
                    if next != color[i] {
                        color[i] = next;
                        if i < 3 {
                            self.color_hsv = rgb_to_hsv(color[0], color[1], color[2]);
                            self.color_last_rgb = [color[0], color[1], color[2]];
                        }
                        write_hex_rgba(*color, &mut self.color_hex_buf);
                        self.color_hex_len = 8;
                        changed = true;
                    }
                }
            }
            dl.rect_rounded(bx, rgba_y, cell_w, cell_h, WIDGET_ROUNDING, if hov || self.color_drag == drag_id { s.hovered } else { s.elevated });
            dl.outline_rounded(bx, rgba_y, cell_w, cell_h, WIDGET_ROUNDING, 1.0, if self.color_drag == drag_id { s.accent } else { s.border });
            dl.text(bx + 4.0, (rgba_y + (cell_h - th) * 0.5).floor(), CH_LABELS[i], s.text_dim, 0.9);
            let val_str = format_u8_buf(color[i], &mut num_buf);
            let vw = dl.text_width(val_str, 0.9);
            dl.text(bx + cell_w - vw - 4.0, (rgba_y + (cell_h - th) * 0.5).floor(), val_str, s.text, 0.9);
        }

        // HEX field (stack buffer, zero heap allocations)
        let hex_y = rgba_y + cell_h + 8.0;
        let hex_h = 22.0;
        let hex_hov = self.inside(sv_x, hex_y, sv_w, hex_h);
        if self.clicked {
            self.color_hex_focus = hex_hov;
        }
        if self.color_hex_focus {
            // Poll 0-9, A-F, Backspace, Enter
            for vk in 0x30u16..=0x39 {
                let d = key_down(vk);
                if d && !self.prev_keys[vk as usize] && self.color_hex_len < 8 {
                    self.color_hex_buf[self.color_hex_len] = b'0' + (vk - 0x30) as u8;
                    self.color_hex_len += 1;
                }
                self.prev_keys[vk as usize] = d;
            }
            for vk in 0x41u16..=0x46 {
                let d = key_down(vk);
                if d && !self.prev_keys[vk as usize] && self.color_hex_len < 8 {
                    self.color_hex_buf[self.color_hex_len] = b'A' + (vk - 0x41) as u8;
                    self.color_hex_len += 1;
                }
                self.prev_keys[vk as usize] = d;
            }
            let bs = key_down(0x08);
            if bs && !self.prev_keys[0x08] && self.color_hex_len > 0 {
                self.color_hex_len -= 1;
            }
            self.prev_keys[0x08] = bs;
            let enter = key_down(0x0D);
            if enter && !self.prev_keys[0x0D] {
                if let Some(parsed) = parse_hex_rgba(&self.color_hex_buf, self.color_hex_len, color[3]) {
                    *color = parsed;
                    self.color_hsv = rgb_to_hsv(color[0], color[1], color[2]);
                    self.color_last_rgb = [color[0], color[1], color[2]];
                    write_hex_rgba(*color, &mut self.color_hex_buf);
                    self.color_hex_len = 8;
                    changed = true;
                }
                self.color_hex_focus = false;
            }
            self.prev_keys[0x0D] = enter;
        }
        dl.rect_rounded(sv_x, hex_y, sv_w, hex_h, WIDGET_ROUNDING, if self.color_hex_focus { s.hovered } else { s.elevated });
        dl.outline_rounded(sv_x, hex_y, sv_w, hex_h, WIDGET_ROUNDING, 1.0, if self.color_hex_focus { s.accent } else { s.border });
        dl.text(sv_x + 6.0, (hex_y + (hex_h - th) * 0.5).floor(), "HEX  #", s.text_dim, 0.9);
        let hex_str = std::str::from_utf8(&self.color_hex_buf[..self.color_hex_len]).unwrap_or("");
        dl.text(sv_x + 48.0, (hex_y + (hex_h - th) * 0.5).floor(), hex_str, s.text, 0.9);

        dl.end_top();

        if self.clicked
            && self.color_drag == 0
            && !self.inside(x, y, w, row_h)
            && !self.inside(px, py, pop_w, pop_h)
        {
            self.open_color = None;
            self.color_hex_focus = false;
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

    pub fn popup_button<R>(&mut self, dl: &mut DrawList, label: &str, id: &str,
        rect: (f32,f32,f32,f32), width: f32, height: f32, s: &Style,
        content: impl FnOnce(&mut Ui, &mut DrawList) -> R) -> Option<R> {
        let was_in_popup = self.in_popup;
        if self.open_popup.as_deref() == Some(id) { self.in_popup = true; }
        let pressed = self.button_at(dl,label,rect.0,rect.1,rect.2,rect.3,s);
        self.in_popup = was_in_popup;
        self.last_rect = rect;
        self.last_id = id.to_owned();
        if pressed {
            self.open_popup = if self.open_popup.as_deref() == Some(id) { None } else { Some(id.to_owned()) };
            self.open_sub_popup = None;
        }
        let clicked = self.clicked;
        if pressed { self.clicked = false; }
        let result = self.popup(dl,id,width,height,s,content);
        self.clicked = clicked;
        result
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
        let is_nested = self.in_popup;
        for offset in [0.0, 5.0, 10.0] {
            dl.rect(anchor.0 + anchor.2 - 49.0 + offset, anchor.1 + 11.0, 2.0, 2.0, s.text_dim);
        }
        let own_open = self.open_popup.as_deref() == Some(id) || self.open_sub_popup.as_deref() == Some(id);
        let can_open = !self.overlay_blocking() || is_nested || own_open;
        let gear_hit = can_open
            && self.clicked
            && self.inside(anchor.0 + anchor.2 - 52.0, anchor.1, 20.0, anchor.3);
        let rmb_hit = can_open
            && self.right_clicked
            && self.inside(anchor.0, anchor.1, anchor.2, anchor.3);

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
        // Drop-down style: right-aligned under the row. The measured height
        // from the previous frame keeps long popups from spilling over rows.
        let popup_h = self.popup_heights.get(id).copied().unwrap_or(h).max(h);
        let x = (anchor.0 + anchor.2 - w)
            .max(anchor.0)
            .min(screen_w - w - 6.0)
            .max(6.0);
        let y = (anchor.1 + anchor.3 + 4.0)
            .min(screen_h - popup_h - 6.0)
            .max(6.0);
        dl.begin_top();
        dl.rect_rounded(x, y, w, popup_h, WIDGET_ROUNDING, s.dark);
        dl.outline_rounded(x, y, w, popup_h, WIDGET_ROUNDING, 1.0, s.border);
        let saved = (self.cursor, self.row_w, self.in_popup);
        self.cursor = Vec2::new(x + 10.0, y + 10.0);
        self.row_w = w - 20.0;
        self.in_popup = true;
        let out = f(self, dl);
        let measured = (self.cursor.y - (y + 10.0)).max(popup_h);
        self.popup_heights.insert(id.to_string(), measured);
        self.cursor = saved.0;
        self.row_w = saved.1;
        self.in_popup = saved.2;
        dl.end_top();

        if self.clicked
            && !self.inside(x, y, w, popup_h)
            && !self.inside(anchor.0, anchor.1, anchor.2, anchor.3)
            && self.open_color.is_none()
            && self.open_list.is_none()
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

#[cfg(test)]
mod interaction_tests {
    use super::*;
    #[test]
    fn options_button_opens_and_closes_its_own_popup() {
        let mut ui=Ui::new(); let mut dl=DrawList::new();
        let style=Style::new([120,180,255,255]);
        for expected in [true,false] {
            dl.begin(640.0,480.0);
            ui.test_pointer(20.0,20.0,true,0.0);
            ui.popup_button(&mut dl,"Options","options",(10.0,10.0,84.0,26.0),200.0,100.0,&style,|_,_|{});
            assert_eq!(ui.open_popup.is_some(),expected);
        }
    }
    #[test]
    fn switch_and_popup_have_independent_hit_regions() {
        let mut ui = Ui::new();
        let mut dl = DrawList::new();
        let style = Style::new([120,180,255,255]);
        let mut enabled = false;
        // Register the popup owner on the first frame.
        for (mouse_x, expected_switch, expected_popup) in [(0.0,false,false),(90.0,true,false),(56.0,true,true)] {
            dl.begin(640.0,480.0);
            ui.cursor = Vec2::new(0.0,0.0); ui.row_w = 100.0;
            ui.mouse = Vec2::new(mouse_x,12.0); ui.clicked = mouse_x != 0.0;
            ui.checkbox(&mut dl,"test",&mut enabled,&style);
            ui.popup(&mut dl,"popup",100.0,50.0,&style,|_,_| {});
            assert_eq!(enabled, expected_switch);
            assert_eq!(ui.open_popup.is_some(), expected_popup);
        }
    }
}
