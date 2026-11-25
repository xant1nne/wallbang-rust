// Config block: 5x5 pixel font, 1:1 port of Celerity
// `src/features/visuals/pixel_font.hpp` (`pixel_font_5x5::glyphs`). Used for
// ESP flags, ammo text and the health value — the same elements Celerity
// renders with it.
use crate::draw::DrawList;

#[derive(Clone, Copy)]
pub struct Glyph {
    pub width: u8,
    pub height: u8,
    pub rows: [u8; 6],
}

const EMPTY: Glyph = Glyph { width: 0, height: 0, rows: [0; 6] };

const fn glyph(width: u8, height: u8, rows: [u8; 6]) -> Glyph {
    Glyph { width, height, rows }
}

/// Char spacing from Celerity: `char_spacing = 1.0f`, `pixel_size = 1.0f`.
const PIXEL_SIZE: f32 = 1.0;
const CHAR_SPACING: f32 = 1.0;
/// Fallback advance for characters the table does not contain.
const MISSING_ADVANCE: f32 = 4.0 + CHAR_SPACING;

pub const HEIGHT: f32 = 5.0;

/// Celerity's table is indexed by lowercase ASCII (uppercase is folded before
/// lookup), which is why only lowercase letters exist here.
const GLYPHS: [Glyph; 128] = build_table();

const fn build_table() -> [Glyph; 128] {
    let mut t = [EMPTY; 128];
    t[b'0' as usize] = glyph(3, 5, [2, 5, 5, 5, 2, 0]);
    t[b'1' as usize] = glyph(2, 5, [1, 3, 1, 1, 1, 0]);
    t[b'2' as usize] = glyph(3, 5, [6, 1, 2, 4, 7, 0]);
    t[b'3' as usize] = glyph(3, 5, [6, 1, 2, 1, 6, 0]);
    t[b'4' as usize] = glyph(3, 5, [1, 3, 5, 7, 1, 0]);
    t[b'5' as usize] = glyph(3, 5, [7, 4, 7, 1, 6, 0]);
    t[b'6' as usize] = glyph(3, 5, [2, 4, 6, 5, 2, 0]);
    t[b'7' as usize] = glyph(3, 5, [7, 1, 2, 2, 2, 0]);
    t[b'8' as usize] = glyph(3, 5, [2, 5, 2, 5, 2, 0]);
    t[b'9' as usize] = glyph(3, 5, [2, 5, 7, 1, 2, 0]);
    t[b'a' as usize] = glyph(4, 5, [6, 6, 9, 15, 9, 0]);
    t[b'b' as usize] = glyph(4, 5, [14, 9, 14, 9, 14, 0]);
    t[b'c' as usize] = glyph(4, 5, [7, 8, 8, 8, 7, 0]);
    t[b'd' as usize] = glyph(4, 5, [14, 9, 9, 9, 14, 0]);
    t[b'e' as usize] = glyph(3, 5, [7, 4, 6, 4, 7, 0]);
    t[b'f' as usize] = glyph(3, 5, [7, 4, 6, 4, 4, 0]);
    t[b'g' as usize] = glyph(4, 5, [7, 8, 11, 9, 7, 0]);
    t[b'h' as usize] = glyph(4, 5, [9, 9, 15, 9, 9, 0]);
    t[b'i' as usize] = glyph(1, 5, [1, 1, 1, 1, 1, 0]);
    t[b'j' as usize] = glyph(4, 5, [1, 1, 1, 9, 6, 0]);
    t[b'k' as usize] = glyph(4, 5, [9, 10, 12, 10, 9, 0]);
    t[b'l' as usize] = glyph(3, 5, [4, 4, 4, 4, 7, 0]);
    t[b'm' as usize] = glyph(5, 5, [17, 27, 21, 17, 17, 0]);
    t[b'n' as usize] = glyph(4, 5, [9, 13, 11, 9, 9, 0]);
    t[b'o' as usize] = glyph(4, 5, [6, 9, 9, 9, 6, 0]);
    t[b'p' as usize] = glyph(4, 5, [14, 9, 14, 8, 8, 0]);
    t[b'r' as usize] = glyph(4, 5, [14, 9, 14, 10, 9, 0]);
    t[b's' as usize] = glyph(4, 5, [7, 8, 6, 1, 14, 0]);
    t[b't' as usize] = glyph(3, 5, [7, 2, 2, 2, 2, 0]);
    t[b'u' as usize] = glyph(4, 5, [9, 9, 9, 9, 6, 0]);
    t[b'v' as usize] = glyph(4, 5, [9, 9, 9, 6, 0, 0]);
    t[b'w' as usize] = glyph(5, 5, [17, 17, 21, 27, 17, 0]);
    t[b'x' as usize] = glyph(3, 5, [5, 5, 2, 5, 5, 0]);
    t[b'y' as usize] = glyph(3, 5, [5, 5, 2, 2, 2, 0]);
    t[b'z' as usize] = glyph(3, 5, [7, 1, 2, 4, 7, 0]);
    t[b' ' as usize] = glyph(3, 5, [0, 0, 0, 0, 0, 0]);
    t[b'$' as usize] = glyph(3, 6, [2, 7, 6, 3, 7, 2]);
    t[b'/' as usize] = glyph(3, 5, [1, 1, 2, 4, 4, 0]);
    t[b'.' as usize] = glyph(3, 5, [0, 0, 0, 0, 2, 0]);
    t[b'q' as usize] = glyph(4, 5, [6, 9, 9, 11, 7, 0]);
    t[b'(' as usize] = glyph(3, 5, [2, 4, 4, 4, 2, 0]);
    t[b')' as usize] = glyph(3, 5, [2, 1, 1, 1, 2, 0]);
    t[b'-' as usize] = glyph(2, 5, [0, 0, 3, 0, 0, 0]);
    t[b':' as usize] = glyph(3, 5, [0, 2, 0, 2, 0, 0]);
    t[b'!' as usize] = glyph(3, 5, [2, 2, 2, 0, 2, 0]);
    t
}

#[inline]
fn lookup(c: char) -> Option<&'static Glyph> {
    if !c.is_ascii() {
        return None;
    }
    let folded = c.to_ascii_lowercase() as usize;
    let glyph = &GLYPHS[folded];
    (glyph.width > 0).then_some(glyph)
}

#[inline]
fn pixel(glyph: &Glyph, x: usize, y: usize) -> bool {
    // MSB-first rows, exactly like the C++ `pixel_at` helper.
    (glyph.rows[y] >> (glyph.width - 1 - x as u8)) & 1 != 0
}

/// Advance width of `text` in pixels (Celerity `calc_pixel_text_size` x).
pub fn text_width(text: &str) -> f32 {
    let mut width = 0.0;
    for c in text.chars() {
        width += match lookup(c) {
            Some(glyph) => glyph.width as f32 * PIXEL_SIZE + CHAR_SPACING,
            None => MISSING_ADVANCE,
        };
    }
    width
}

/// Draws every lit pixel of one glyph at `(x, y)`.
fn glyph_pass(dl: &mut DrawList, glyph: &Glyph, x: f32, y: f32, color: [u8; 4]) {
    for py in 0..glyph.height as usize {
        for px in 0..glyph.width as usize {
            if pixel(glyph, px, py) {
                dl.rect(
                    x + px as f32 * PIXEL_SIZE,
                    y + py as f32 * PIXEL_SIZE,
                    PIXEL_SIZE,
                    PIXEL_SIZE,
                    color,
                );
            }
        }
    }
}

/// Draws `text` at `(x, y)` with a 1px black outline, 1:1 in shape with
/// Celerity `draw_pixel_text`. The outline is produced by four 1px-offset
/// passes instead of a per-pixel 3x3 scan: identical silhouette, 5x fewer
/// quads (the 3x3 scan re-emitted the same outline quad for every neighbour).
pub fn draw(dl: &mut DrawList, x: f32, y: f32, color: [u8; 4], text: &str) {
    const OUTLINE: [u8; 4] = [0, 0, 0, 255];
    const OFFSETS: [(f32, f32); 4] = [(-1.0, 0.0), (1.0, 0.0), (0.0, -1.0), (0.0, 1.0)];
    let mut pen_x = x;
    for c in text.chars() {
        let Some(glyph) = lookup(c) else {
            pen_x += MISSING_ADVANCE;
            continue;
        };
        for (offset_x, offset_y) in OFFSETS {
            glyph_pass(dl, glyph, pen_x + offset_x, y + offset_y, OUTLINE);
        }
        glyph_pass(dl, glyph, pen_x, y, color);
        pen_x += glyph.width as f32 * PIXEL_SIZE + CHAR_SPACING;
    }
}

#[cfg(test)]
mod tests {
    use super::{text_width, HEIGHT};
    use super::GLYPHS;

    #[test]
    fn glyph_table_matches_celerity_entries() {
        assert_eq!(GLYPHS[b'0' as usize].width, 3);
        assert_eq!(GLYPHS[b'a' as usize].rows, [6, 6, 9, 15, 9, 0]);
        assert_eq!(GLYPHS[b'$' as usize].height, 6);
        assert_eq!(GLYPHS[b' ' as usize].width, 3);
        // Uppercase is folded at lookup time, so the table stays lowercase only.
        assert_eq!(GLYPHS[b'A' as usize].width, 0);
    }

    #[test]
    fn widths_use_glyph_advance_plus_spacing() {
        assert_eq!(HEIGHT, 5.0);
        // '0' is 3px wide plus 1px spacing, twice.
        assert_eq!(text_width("00"), 8.0);
        // Missing glyphs advance like Celerity's fallback.
        assert_eq!(text_width("\u{4e2d}"), 5.0);
    }
}
