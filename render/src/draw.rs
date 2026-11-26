// Config block: сглаженный TrueType-атлас из crate::font; MAX_VERTICES из dx11.
use crate::dx11::MAX_VERTICES;
use crate::font::{self, CHAR_COUNT, FIRST_CHAR, LAST_CHAR};

pub type Color = [u8; 4];

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub u: f32,
    pub v: f32,
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[derive(Clone, Copy, Debug)]
struct ClipRect {
    x0: f32,
    y0: f32,
    x1: f32,
    y1: f32,
}

pub struct DrawList {
    vertices: Vec<Vertex>,
    top_vertices: Vec<Vertex>,
    top_depth: usize,
    clip_stack: Vec<ClipRect>,
    alpha_stack: Vec<f32>,
    width: f32,
    height: f32,
    white_u: f32,
    white_v: f32,
}

impl DrawList {
    pub fn new() -> Self {
        let atlas = font::get();
        Self {
            vertices: Vec::with_capacity(16_384),
            top_vertices: Vec::with_capacity(4_096),
            top_depth: 0,
            clip_stack: Vec::with_capacity(8),
            alpha_stack: Vec::with_capacity(4),
            width: 1920.0,
            height: 1080.0,
            white_u: atlas.white_u,
            white_v: atlas.white_v,
        }
    }

    #[inline]
    pub fn begin(&mut self, width: f32, height: f32) {
        self.width = width.max(1.0);
        self.height = height.max(1.0);
        self.vertices.clear();
        self.top_vertices.clear();
        self.top_depth = 0;
        self.clip_stack.clear();
        self.alpha_stack.clear();
    }

    #[inline]
    pub fn begin_top(&mut self) {
        self.top_depth = self.top_depth.saturating_add(1);
    }

    #[inline]
    pub fn end_top(&mut self) {
        self.top_depth = self.top_depth.saturating_sub(1);
    }

    #[inline]
    pub fn flush_top(&mut self) {
        if !self.top_vertices.is_empty() {
            let avail = MAX_VERTICES.saturating_sub(self.vertices.len());
            let count = self.top_vertices.len().min(avail);
            self.vertices.extend_from_slice(&self.top_vertices[..count]);
            self.top_vertices.clear();
        }
        self.top_depth = 0;
    }

    #[inline]
    pub fn push_clip(&mut self, x: f32, y: f32, w: f32, h: f32) {
        let mut next = ClipRect {
            x0: x,
            y0: y,
            x1: x + w.max(0.0),
            y1: y + h.max(0.0),
        };
        if let Some(prev) = self.clip_stack.last() {
            next.x0 = next.x0.max(prev.x0);
            next.y0 = next.y0.max(prev.y0);
            next.x1 = next.x1.min(prev.x1).max(next.x0);
            next.y1 = next.y1.min(prev.y1).max(next.y0);
        }
        self.clip_stack.push(next);
    }

    #[inline]
    pub fn pop_clip(&mut self) {
        self.clip_stack.pop();
    }

    #[inline]
    pub fn push_alpha(&mut self, alpha: f32) {
        let cur = self.alpha_stack.last().copied().unwrap_or(1.0);
        self.alpha_stack.push((cur * alpha).clamp(0.0, 1.0));
    }

    #[inline]
    pub fn pop_alpha(&mut self) {
        self.alpha_stack.pop();
    }

    #[inline]
    fn apply_alpha(&self, mut c: Color) -> Color {
        if let Some(&a) = self.alpha_stack.last() {
            c[3] = ((c[3] as f32) * a).round().clamp(0.0, 255.0) as u8;
        }
        c
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.vertices.len()
    }

    #[inline]
    pub fn vertices(&self) -> &[Vertex] {
        &self.vertices
    }

    #[inline]
    pub fn size(&self) -> (f32, f32) {
        (self.width, self.height)
    }

    /// Тестовый полноэкранный квадрат — чтобы ранне был виден путь рендера.
    pub fn debug_quad(&mut self, color: Color) {
        self.textured_quad(0.0, 0.0, self.width, self.height, color, 0.0, 0.0, 1.0, 1.0);
    }

    #[inline]
    fn ndc(&self, x: f32, y: f32) -> (f32, f32) {
        (x / self.width * 2.0 - 1.0, 1.0 - y / self.height * 2.0)
    }

    #[inline]
    fn push(&mut self, x: f32, y: f32, u: f32, v: f32, c: Color) {
        let c = self.apply_alpha(c);
        let (nx, ny) = self.ndc(x, y);
        let target = if self.top_depth > 0 {
            &mut self.top_vertices
        } else {
            &mut self.vertices
        };
        if target.len() >= MAX_VERTICES {
            return;
        }
        target.push(Vertex {
            x: nx,
            y: ny,
            u,
            v,
            r: c[0],
            g: c[1],
            b: c[2],
            a: c[3],
        });
    }

    #[inline]
    fn tri_solid(&mut self, x0: f32, y0: f32, x1: f32, y1: f32, x2: f32, y2: f32, c: Color) {
        // Top-layer commands are popups/tooltips: they must be able to escape
        // the scroll clip of the child that opened them.
        if self.top_depth == 0 {
            if let Some(clip) = self.clip_stack.last() {
                let min_x = x0.min(x1).min(x2);
                let max_x = x0.max(x1).max(x2);
                let min_y = y0.min(y1).min(y2);
                let max_y = y0.max(y1).max(y2);
                if max_x <= clip.x0 || min_x >= clip.x1 || max_y <= clip.y0 || min_y >= clip.y1 {
                    return;
                }
            }
        }
        let (u, v) = (self.white_u, self.white_v);
        self.push(x0, y0, u, v, c);
        self.push(x1, y1, u, v, c);
        self.push(x2, y2, u, v, c);
    }

    pub fn textured_quad(
        &mut self,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        c: Color,
        mut u0: f32,
        mut v0: f32,
        mut u1: f32,
        mut v1: f32,
    ) {
        if w <= 0.0 || h <= 0.0 {
            return;
        }
        let mut x0 = x;
        let mut y0 = y;
        let mut x1 = x + w;
        let mut y1 = y + h;
        if self.top_depth == 0 {
            if let Some(clip) = self.clip_stack.last().copied() {
                if x1 <= clip.x0 || x0 >= clip.x1 || y1 <= clip.y0 || y0 >= clip.y1 {
                    return;
                }
                let du = (u1 - u0) / w;
                let dv = (v1 - v0) / h;
                if x0 < clip.x0 {
                    u0 += (clip.x0 - x0) * du;
                    x0 = clip.x0;
                }
                if x1 > clip.x1 {
                    u1 -= (x1 - clip.x1) * du;
                    x1 = clip.x1;
                }
                if y0 < clip.y0 {
                    v0 += (clip.y0 - y0) * dv;
                    y0 = clip.y0;
                }
                if y1 > clip.y1 {
                    v1 -= (y1 - clip.y1) * dv;
                    y1 = clip.y1;
                }
                if x1 <= x0 || y1 <= y0 {
                    return;
                }
            }
        }
        self.push(x0, y0, u0, v0, c);
        self.push(x1, y0, u1, v0, c);
        self.push(x0, y1, u0, v1, c);
        self.push(x1, y0, u1, v0, c);
        self.push(x1, y1, u1, v1, c);
        self.push(x0, y1, u0, v1, c);
    }

    #[inline]
    pub fn rect(&mut self, x: f32, y: f32, w: f32, h: f32, c: Color) {
        self.textured_quad(x, y, w, h, c, self.white_u, self.white_v, self.white_u, self.white_v);
    }

    pub fn rect_rounded(&mut self, x: f32, y: f32, w: f32, h: f32, r: f32, c: Color) {
        let r = r.min(w * 0.5).min(h * 0.5).max(0.0);
        if r <= 0.5 {
            self.rect(x, y, w, h, c);
            return;
        }
        // Center cross rects
        self.rect(x + r, y, w - r * 2.0, h, c);
        self.rect(x, y + r, r, h - r * 2.0, c);
        self.rect(x + w - r, y + r, r, h - r * 2.0, c);
        // 4 corner quarter-fans
        let corners = [
            (x + r, y + r, std::f32::consts::PI, std::f32::consts::FRAC_PI_2 * 3.0),
            (x + w - r, y + r, std::f32::consts::FRAC_PI_2 * 3.0, std::f32::consts::TAU),
            (x + w - r, y + h - r, 0.0, std::f32::consts::FRAC_PI_2),
            (x + r, y + h - r, std::f32::consts::FRAC_PI_2, std::f32::consts::PI),
        ];
        const SEGS: usize = 5;
        for (cx, cy, a0, a1) in corners {
            let step = (a1 - a0) / SEGS as f32;
            for i in 0..SEGS {
                let t0 = a0 + i as f32 * step;
                let t1 = t0 + step;
                self.tri_solid(
                    cx,
                    cy,
                    cx + t0.cos() * r,
                    cy + t0.sin() * r,
                    cx + t1.cos() * r,
                    cy + t1.sin() * r,
                    c,
                );
            }
        }
    }

    pub fn outline(&mut self, x: f32, y: f32, w: f32, h: f32, t: f32, c: Color) {
        self.rect(x, y, w, t, c);
        self.rect(x, y + h - t, w, t, c);
        self.rect(x, y, t, h, c);
        self.rect(x + w - t, y, t, h, c);
    }

    pub fn outline_rounded(&mut self, x: f32, y: f32, w: f32, h: f32, r: f32, t: f32, c: Color) {
        let r = r.min(w * 0.5).min(h * 0.5).max(0.0);
        if r <= 0.5 {
            self.outline(x, y, w, h, t, c);
            return;
        }
        // Straight edges
        self.rect(x + r, y, w - r * 2.0, t, c);
        self.rect(x + r, y + h - t, w - r * 2.0, t, c);
        self.rect(x, y + r, t, h - r * 2.0, c);
        self.rect(x + w - t, y + r, t, h - r * 2.0, c);
        // Rounded corner arcs
        let corners = [
            (x + r, y + r, std::f32::consts::PI, std::f32::consts::FRAC_PI_2 * 3.0),
            (x + w - r, y + r, std::f32::consts::FRAC_PI_2 * 3.0, std::f32::consts::TAU),
            (x + w - r, y + h - r, 0.0, std::f32::consts::FRAC_PI_2),
            (x + r, y + h - r, std::f32::consts::FRAC_PI_2, std::f32::consts::PI),
        ];
        const SEGS: usize = 5;
        let ri = (r - t).max(0.0);
        for (cx, cy, a0, a1) in corners {
            let step = (a1 - a0) / SEGS as f32;
            for i in 0..SEGS {
                let t0 = a0 + i as f32 * step;
                let t1 = t0 + step;
                let (c0, s0) = (t0.cos(), t0.sin());
                let (c1, s1) = (t1.cos(), t1.sin());
                let ox0 = cx + c0 * r;
                let oy0 = cy + s0 * r;
                let ox1 = cx + c1 * r;
                let oy1 = cy + s1 * r;
                let ix0 = cx + c0 * ri;
                let iy0 = cy + s0 * ri;
                let ix1 = cx + c1 * ri;
                let iy1 = cy + s1 * ri;
                self.tri_solid(ox0, oy0, ox1, oy1, ix0, iy0, c);
                self.tri_solid(ox1, oy1, ix1, iy1, ix0, iy0, c);
            }
        }
    }

    pub fn circle_filled(&mut self, cx: f32, cy: f32, r: f32, c: Color) {
        const SEGS: usize = 14;
        let step = std::f32::consts::TAU / SEGS as f32;
        for i in 0..SEGS {
            let a0 = i as f32 * step;
            let a1 = a0 + step;
            self.tri_solid(
                cx,
                cy,
                cx + a0.cos() * r,
                cy + a0.sin() * r,
                cx + a1.cos() * r,
                cy + a1.sin() * r,
                c,
            );
        }
    }

    pub fn convex_poly(&mut self, pts: &[(f32, f32)], c: Color) {
        if pts.len() < 3 {
            return;
        }
        let p0 = pts[0];
        for i in 1..(pts.len() - 1) {
            self.tri_solid(p0.0, p0.1, pts[i].0, pts[i].1, pts[i + 1].0, pts[i + 1].1, c);
        }
    }

    pub fn line(&mut self, x0: f32, y0: f32, x1: f32, y1: f32, thickness: f32, c: Color) {
        let dx = x1 - x0;
        let dy = y1 - y0;
        let len = (dx * dx + dy * dy).sqrt();
        if len < 0.0001 {
            return;
        }
        let nx = -dy / len * thickness * 0.5;
        let ny = dx / len * thickness * 0.5;
        self.tri_solid(x0 + nx, y0 + ny, x1 + nx, y1 + ny, x0 - nx, y0 - ny, c);
        self.tri_solid(x1 + nx, y1 + ny, x1 - nx, y1 - ny, x0 - nx, y0 - ny, c);
    }

    pub fn polyline(&mut self, pts: &[(f32, f32)], thickness: f32, c: Color) {
        for pair in pts.windows(2) {
            self.line(pair[0].0, pair[0].1, pair[1].0, pair[1].1, thickness, c);
        }
    }

    /// Vector WALLBANG "W" logo with flared serif tips and diagonal white slash.
    pub fn draw_wallbang_logo(&mut self, x: f32, y: f32, w: f32, h: f32) {
        let p = |nx: f32, ny: f32| (x + nx * w, y + ny * h);
        let blue: Color = [164, 204, 232, 255];
        let dark: Color = [12, 12, 12, 255];
        let white: Color = [245, 250, 255, 255];

        // Left serif wing + outer left stroke of W
        self.convex_poly(
            &[p(0.06, 0.17), p(0.22, 0.21), p(0.28, 0.30), p(0.18, 0.28)],
            blue,
        );
        self.convex_poly(
            &[p(0.18, 0.26), p(0.28, 0.26), p(0.42, 0.85), p(0.32, 0.85)],
            blue,
        );
        // Center-left stroke up to middle peak
        self.convex_poly(
            &[p(0.32, 0.85), p(0.42, 0.85), p(0.55, 0.36), p(0.47, 0.36)],
            blue,
        );
        // Center-right stroke down to right valley
        self.convex_poly(
            &[p(0.47, 0.36), p(0.55, 0.36), p(0.70, 0.85), p(0.60, 0.85)],
            blue,
        );
        // Right outer stroke up to flared wing
        self.convex_poly(
            &[p(0.60, 0.85), p(0.70, 0.85), p(0.84, 0.22), p(0.74, 0.22)],
            blue,
        );
        self.convex_poly(
            &[p(0.74, 0.22), p(0.94, 0.14), p(0.84, 0.28), p(0.75, 0.31)],
            blue,
        );

        // Dark cut behind diagonal slash for sharp negative space
        self.convex_poly(
            &[p(0.02, 0.65), p(0.97, 0.34), p(0.97, 0.39), p(0.02, 0.69)],
            dark,
        );
        // Sharp tapered pure-white diagonal slash
        self.convex_poly(
            &[p(0.01, 0.67), p(0.99, 0.35), p(0.96, 0.375), p(0.05, 0.665)],
            white,
        );
    }

    /// Slanted parallelogram (long slim watermark shape with parallel left/right bevels).
    pub fn parallelogram(&mut self, x: f32, y: f32, w: f32, h: f32, slant: f32, fill: Color, border: Color) {
        let pts = [
            (x + slant, y),
            (x + w, y),
            (x + w - slant, y + h),
            (x, y + h),
        ];
        self.convex_poly(&pts, fill);
        let loop_pts = [pts[0], pts[1], pts[2], pts[3], pts[0]];
        self.polyline(&loop_pts, 1.0, border);
    }

    /// Panel with parallel chamfered top-left and bottom-right corners (hotkeys / indicators / spectators).
    pub fn chamfered_panel(&mut self, x: f32, y: f32, w: f32, h: f32, cut: f32, fill: Color, border: Color) {
        let c = cut.min(w * 0.4).min(h * 0.4).max(0.0);
        let pts = [
            (x + c, y),
            (x + w, y),
            (x + w, y + h - c),
            (x + w - c, y + h),
            (x, y + h),
            (x, y + c),
        ];
        self.convex_poly(&pts, fill);
        let loop_pts = [pts[0], pts[1], pts[2], pts[3], pts[4], pts[5], pts[0]];
        self.polyline(&loop_pts, 1.0, border);
    }

    fn text_width_with(&self, text: &str, scale: f32, brand: bool) -> f32 {
        let atlas = font::get();
        let table = if brand { &atlas.brand_glyphs } else { &atlas.ui_glyphs };
        let mut total = 0.0f32;
        for byte in text.bytes() {
            let idx = if (FIRST_CHAR..=LAST_CHAR).contains(&byte) {
                (byte - FIRST_CHAR) as usize
            } else {
                (b'?' - FIRST_CHAR) as usize
            };
            if idx < CHAR_COUNT {
                total += table[idx].advance * scale;
            }
        }
        total
    }

    pub fn text_width(&self, text: &str, scale: f32) -> f32 {
        self.text_width_with(text, scale, false)
    }

    pub fn brand_text_width(&self, text: &str, scale: f32) -> f32 {
        self.text_width_with(text, scale, true)
    }

    #[inline]
    pub fn text_height(&self, scale: f32) -> f32 {
        font::get().ui_line_h * scale
    }

    #[inline]
    pub fn brand_text_height(&self, scale: f32) -> f32 {
        font::get().brand_line_h * scale
    }

    pub fn truncate(&self, text: &str, max_w: f32, scale: f32) -> String {
        if self.text_width(text, scale) <= max_w {
            return text.to_string();
        }
        let ell_w = self.text_width("...", scale);
        let mut out = String::new();
        for ch in text.chars() {
            let mut probe = out.clone();
            probe.push(ch);
            if self.text_width(&probe, scale) + ell_w > max_w {
                break;
            }
            out.push(ch);
        }
        out.push_str("...");
        out
    }

    fn draw_text_impl(&mut self, x: f32, y: f32, text: &str, c: Color, scale: f32, brand: bool) {
        let atlas = font::get();
        let table = if brand { &atlas.brand_glyphs } else { &atlas.ui_glyphs };
        let mut pen_x = x.floor();
        let py = y.floor();
        for byte in text.bytes() {
            let idx = if (FIRST_CHAR..=LAST_CHAR).contains(&byte) {
                (byte - FIRST_CHAR) as usize
            } else {
                (b'?' - FIRST_CHAR) as usize
            };
            if idx >= CHAR_COUNT {
                continue;
            }
            let g = table[idx];
            if byte != b' ' && g.w > 0.0 && g.h > 0.0 {
                self.textured_quad(
                    pen_x,
                    py,
                    g.w * scale,
                    g.h * scale,
                    c,
                    g.u0,
                    g.v0,
                    g.u1,
                    g.v1,
                );
            }
            pen_x += g.advance * scale;
        }
    }

    pub fn text(&mut self, x: f32, y: f32, text: &str, c: Color, scale: f32) {
        self.draw_text_impl(x, y, text, c, scale, false);
    }

    pub fn text_brand(&mut self, x: f32, y: f32, text: &str, c: Color, scale: f32) {
        self.draw_text_impl(x, y, text, c, scale, true);
    }
}

impl Default for DrawList {
    fn default() -> Self {
        Self::new()
    }
}
