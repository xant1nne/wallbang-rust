// Config block: all built-in resources are generated here in Rust; no external binary assets
// (no TTF, no PNG) and no third-party crates are needed — only std.

//! Embedded, dependency-free resources for the wallbang project.
//!
//! The crate provides a small 5x7 bitmap font covering the whole printable ASCII range,
//! helpers that expand glyphs to RGBA pixels for a D3D11 texture atlas, a few shared
//! name tables and a default configuration JSON. Everything is `const` or generated
//! at runtime from `const` data, so nothing has to be loaded from disk.

/// Width of a single glyph in pixels.
pub const GLYPH_W: usize = 5;
/// Height of a single glyph in pixels.
pub const GLYPH_H: usize = 7;
/// Number of printable ASCII glyphs (`0x20` ..= `0x7E`).
pub const GLYPH_COUNT: usize = 95;

/// 5x7 bitmaps for ASCII `0x20..=0x7E`.
///
/// Bit 4 of every row is the leftmost pixel, bit 0 is the rightmost one.
/// The first entry is a space, the last one is `~`.
#[rustfmt::skip]
const GLYPHS: [[u8; GLYPH_H]; GLYPH_COUNT] = [
    // ' '
    [0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000],
    // '!'
    [0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00000, 0b00100],
    // '"'
    [0b01010, 0b01010, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000],
    // '#'
    [0b01010, 0b01010, 0b11111, 0b01010, 0b11111, 0b01010, 0b01010],
    // '$'
    [0b00100, 0b01111, 0b10100, 0b01110, 0b00101, 0b11110, 0b00100],
    // '%'
    [0b11000, 0b11001, 0b00010, 0b00100, 0b01000, 0b10011, 0b00011],
    // '&'
    [0b01100, 0b10010, 0b10100, 0b01000, 0b10101, 0b10010, 0b01101],
    // '\''
    [0b00100, 0b00100, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000],
    // '('
    [0b00010, 0b00100, 0b01000, 0b01000, 0b01000, 0b00100, 0b00010],
    // ')'
    [0b01000, 0b00100, 0b00010, 0b00010, 0b00010, 0b00100, 0b01000],
    // '*'
    [0b00000, 0b00100, 0b10101, 0b01110, 0b10101, 0b00100, 0b00000],
    // '+'
    [0b00000, 0b00100, 0b00100, 0b11111, 0b00100, 0b00100, 0b00000],
    // ','
    [0b00000, 0b00000, 0b00000, 0b00000, 0b00100, 0b00100, 0b01000],
    // '-'
    [0b00000, 0b00000, 0b00000, 0b11111, 0b00000, 0b00000, 0b00000],
    // '.'
    [0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b01100, 0b01100],
    // '/'
    [0b00001, 0b00010, 0b00010, 0b00100, 0b01000, 0b01000, 0b10000],
    // '0'
    [0b01110, 0b10001, 0b10011, 0b10101, 0b11001, 0b10001, 0b01110],
    // '1'
    [0b00100, 0b01100, 0b00100, 0b00100, 0b00100, 0b00100, 0b01110],
    // '2'
    [0b01110, 0b10001, 0b00001, 0b00010, 0b00100, 0b01000, 0b11111],
    // '3'
    [0b11111, 0b00010, 0b00100, 0b00010, 0b00001, 0b10001, 0b01110],
    // '4'
    [0b00010, 0b00110, 0b01010, 0b10010, 0b11111, 0b00010, 0b00010],
    // '5'
    [0b11111, 0b10000, 0b11110, 0b00001, 0b00001, 0b10001, 0b01110],
    // '6'
    [0b00110, 0b01000, 0b10000, 0b11110, 0b10001, 0b10001, 0b01110],
    // '7'
    [0b11111, 0b00001, 0b00010, 0b00100, 0b01000, 0b01000, 0b01000],
    // '8'
    [0b01110, 0b10001, 0b10001, 0b01110, 0b10001, 0b10001, 0b01110],
    // '9'
    [0b01110, 0b10001, 0b10001, 0b01111, 0b00001, 0b00010, 0b01100],
    // ':'
    [0b00000, 0b01100, 0b01100, 0b00000, 0b01100, 0b01100, 0b00000],
    // ';'
    [0b00000, 0b01100, 0b01100, 0b00000, 0b00100, 0b00100, 0b01000],
    // '<'
    [0b00010, 0b00100, 0b01000, 0b10000, 0b01000, 0b00100, 0b00010],
    // '='
    [0b00000, 0b00000, 0b11111, 0b00000, 0b11111, 0b00000, 0b00000],
    // '>'
    [0b01000, 0b00100, 0b00010, 0b00001, 0b00010, 0b00100, 0b01000],
    // '?'
    [0b01110, 0b10001, 0b00001, 0b00010, 0b00100, 0b00000, 0b00100],
    // '@'
    [0b01110, 0b10001, 0b00001, 0b01101, 0b10101, 0b10100, 0b01111],
    // 'A'
    [0b01110, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001],
    // 'B'
    [0b11110, 0b10001, 0b10001, 0b11110, 0b10001, 0b10001, 0b11110],
    // 'C'
    [0b01110, 0b10001, 0b10000, 0b10000, 0b10000, 0b10001, 0b01110],
    // 'D'
    [0b11110, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b11110],
    // 'E'
    [0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b11111],
    // 'F'
    [0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b10000],
    // 'G'
    [0b01110, 0b10001, 0b10000, 0b10111, 0b10001, 0b10001, 0b01110],
    // 'H'
    [0b10001, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001],
    // 'I'
    [0b01110, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b01110],
    // 'J'
    [0b00111, 0b00010, 0b00010, 0b00010, 0b00010, 0b10010, 0b01100],
    // 'K'
    [0b10001, 0b10010, 0b10100, 0b11000, 0b10100, 0b10010, 0b10001],
    // 'L'
    [0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b11111],
    // 'M'
    [0b10001, 0b11011, 0b10101, 0b10101, 0b10001, 0b10001, 0b10001],
    // 'N'
    [0b10001, 0b11001, 0b10101, 0b10011, 0b10001, 0b10001, 0b10001],
    // 'O'
    [0b01110, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110],
    // 'P'
    [0b11110, 0b10001, 0b10001, 0b11110, 0b10000, 0b10000, 0b10000],
    // 'Q'
    [0b01110, 0b10001, 0b10001, 0b10001, 0b10101, 0b10010, 0b01101],
    // 'R'
    [0b11110, 0b10001, 0b10001, 0b11110, 0b10100, 0b10010, 0b10001],
    // 'S'
    [0b01111, 0b10000, 0b10000, 0b01110, 0b00001, 0b00001, 0b11110],
    // 'T'
    [0b11111, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100],
    // 'U'
    [0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110],
    // 'V'
    [0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01010, 0b00100],
    // 'W'
    [0b10001, 0b10001, 0b10001, 0b10101, 0b10101, 0b11011, 0b10001],
    // 'X'
    [0b10001, 0b10001, 0b01010, 0b00100, 0b01010, 0b10001, 0b10001],
    // 'Y'
    [0b10001, 0b10001, 0b01010, 0b00100, 0b00100, 0b00100, 0b00100],
    // 'Z'
    [0b11111, 0b00001, 0b00010, 0b00100, 0b01000, 0b10000, 0b11111],
    // '['
    [0b01110, 0b01000, 0b01000, 0b01000, 0b01000, 0b01000, 0b01110],
    // '\\'
    [0b00000, 0b10000, 0b01000, 0b00100, 0b00010, 0b00001, 0b00000],
    // ']'
    [0b01110, 0b00010, 0b00010, 0b00010, 0b00010, 0b00010, 0b01110],
    // '^'
    [0b00100, 0b01010, 0b10001, 0b00000, 0b00000, 0b00000, 0b00000],
    // '_'
    [0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b11111],
    // '`'
    [0b00100, 0b00010, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000],
    // 'a'
    [0b00000, 0b00000, 0b01110, 0b00001, 0b01111, 0b10001, 0b01111],
    // 'b'
    [0b10000, 0b10000, 0b10110, 0b11001, 0b10001, 0b10001, 0b11110],
    // 'c'
    [0b00000, 0b00000, 0b01110, 0b10000, 0b10000, 0b10000, 0b01110],
    // 'd'
    [0b00001, 0b00001, 0b01101, 0b10011, 0b10001, 0b10001, 0b01111],
    // 'e'
    [0b00000, 0b00000, 0b01110, 0b10001, 0b11111, 0b10000, 0b01110],
    // 'f'
    [0b00110, 0b01001, 0b01000, 0b11100, 0b01000, 0b01000, 0b01000],
    // 'g'
    [0b00000, 0b01111, 0b10001, 0b10001, 0b01111, 0b00001, 0b01110],
    // 'h'
    [0b10000, 0b10000, 0b10110, 0b11001, 0b10001, 0b10001, 0b10001],
    // 'i'
    [0b00100, 0b00000, 0b01100, 0b00100, 0b00100, 0b00100, 0b01110],
    // 'j'
    [0b00010, 0b00000, 0b00110, 0b00010, 0b00010, 0b10010, 0b01100],
    // 'k'
    [0b10000, 0b10000, 0b10010, 0b10100, 0b11000, 0b10100, 0b10010],
    // 'l'
    [0b01100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b01110],
    // 'm'
    [0b00000, 0b00000, 0b11010, 0b10101, 0b10101, 0b10101, 0b10101],
    // 'n'
    [0b00000, 0b00000, 0b10110, 0b11001, 0b10001, 0b10001, 0b10001],
    // 'o'
    [0b00000, 0b00000, 0b01110, 0b10001, 0b10001, 0b10001, 0b01110],
    // 'p'
    [0b00000, 0b11110, 0b10001, 0b10001, 0b11110, 0b10000, 0b10000],
    // 'q'
    [0b00000, 0b01111, 0b10001, 0b10001, 0b01111, 0b00001, 0b00001],
    // 'r'
    [0b00000, 0b00000, 0b10110, 0b11001, 0b10000, 0b10000, 0b10000],
    // 's'
    [0b00000, 0b00000, 0b01111, 0b10000, 0b01110, 0b00001, 0b11110],
    // 't'
    [0b01000, 0b01000, 0b11100, 0b01000, 0b01000, 0b01001, 0b00110],
    // 'u'
    [0b00000, 0b00000, 0b10001, 0b10001, 0b10001, 0b10011, 0b01101],
    // 'v'
    [0b00000, 0b00000, 0b10001, 0b10001, 0b10001, 0b01010, 0b00100],
    // 'w'
    [0b00000, 0b00000, 0b10001, 0b10001, 0b10101, 0b10101, 0b01010],
    // 'x'
    [0b00000, 0b00000, 0b10001, 0b01010, 0b00100, 0b01010, 0b10001],
    // 'y'
    [0b00000, 0b10001, 0b10001, 0b10001, 0b01111, 0b00001, 0b01110],
    // 'z'
    [0b00000, 0b00000, 0b11111, 0b00010, 0b00100, 0b01000, 0b11111],
    // '{'
    [0b00110, 0b01000, 0b01000, 0b10000, 0b01000, 0b01000, 0b00110],
    // '|'
    [0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100],
    // '}'
    [0b01100, 0b00010, 0b00010, 0b00001, 0b00010, 0b00010, 0b01100],
    // '~'
    [0b00000, 0b01001, 0b10110, 0b00000, 0b00000, 0b00000, 0b00000],
];

/// Returns the 5x7 bitmap for a printable ASCII character.
///
/// Codes outside `0x20..=0x7E` (including `0`) resolve to the `'?'` glyph.
pub fn glyph(code: u8) -> &'static [u8; GLYPH_H] {
    if (0x20..=0x7E).contains(&code) {
        &GLYPHS[(code - 0x20) as usize]
    } else {
        // The missing-character substitute is always the '?' bitmap.
        &GLYPHS[(b'?' - 0x20) as usize]
    }
}

/// Expands a glyph into `GLYPH_W * GLYPH_H` RGBA pixels (row-major, top-left first).
///
/// Set pixels are opaque white `(255, 255, 255, 255)`, unset pixels are fully
/// transparent `(255, 255, 255, 0)`. The white RGB makes the mask tintable when
/// uploaded into a D3D11 texture atlas.
pub fn rgba_mask(code: u8) -> [[u8; 4]; GLYPH_W * GLYPH_H] {
    let bitmap = glyph(code);
    let mut pixels = [[0u8; 4]; GLYPH_W * GLYPH_H];
    for (pixel, (row, col)) in pixels
        .iter_mut()
        .zip((0..GLYPH_H).flat_map(|y| (0..GLYPH_W).map(move |x| (y, x))))
    {
        let lit = (bitmap[row] >> (GLYPH_W - 1 - col)) & 1 != 0;
        *pixel = [255, 255, 255, if lit { 255 } else { 0 }];
    }
    pixels
}

/// Builds a one-row RGBA atlas with all 95 glyphs laid out left to right.
///
/// Returns `(bytes, width, height)` where `width == GLYPH_COUNT * GLYPH_W`
/// (475 px), `height == GLYPH_H` (7 px) and `bytes.len() == width * height * 4`.
pub fn atlas_rgba() -> (Vec<u8>, usize, usize) {
    let width = GLYPH_COUNT * GLYPH_W;
    let height = GLYPH_H;
    let mut bytes = vec![0u8; width * height * 4];
    for (index, bitmap) in GLYPHS.iter().enumerate() {
        for y in 0..GLYPH_H {
            for x in 0..GLYPH_W {
                let lit = (bitmap[y] >> (GLYPH_W - 1 - x)) & 1 != 0;
                let offset = ((y * width) + (index * GLYPH_W) + x) * 4;
                bytes[offset] = 255;
                bytes[offset + 1] = 255;
                bytes[offset + 2] = 255;
                bytes[offset + 3] = if lit { 255 } else { 0 };
            }
        }
    }
    (bytes, width, height)
}

/// Weapon groups used by the aimbot tabs, in menu order.
pub const WEAPON_GROUPS: [&str; 6] = ["rifle", "scout", "awp", "pistol", "knife", "other"];

/// Hitbox groups shown in the ragebot hit selection.
pub const HITBOX_NAMES: [&str; 6] = ["head", "chest", "stomach", "arms", "legs", "feet"];

/// Cham material names from the C++ menu, in combo order (20 entries).
pub const CHAM_MATERIALS: [&str; 20] = [
    "liquid",
    "metallic",
    "matte",
    "flat",
    "bloom",
    "outlines",
    "glow",
    "electric",
    "distortion",
    "hologram",
    "pearl",
    "liquid (iz)",
    "matte (iz)",
    "flat (iz)",
    "bloom (iz)",
    "outlines (iz)",
    "glow (iz)",
    "distortion (iz)",
    "hologram (iz)",
    "wireframe",
];

/// Deterministic FNV-1a 32-bit hash (kept here so `resources` stays independent
/// of the `config` crate).
pub fn fnv1a(s: &str) -> u32 {
    let mut hash = 0x811c_9dc5u32;
    for &byte in s.as_bytes() {
        hash ^= byte as u32;
        hash = hash.wrapping_mul(0x0100_0193);
    }
    hash
}

/// Default configuration: valid JSON, every boolean disabled, accent blue.
pub fn default_config_json() -> &'static str {
    // Generated from Settings::default(); no legacy C++ JSON.
    static JSON: std::sync::OnceLock<String> = std::sync::OnceLock::new();
    JSON.get_or_init(|| {
        serde_json::to_string_pretty(&wallbang_config::Settings::default()).unwrap_or_default()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn glyph_table_is_complete() {
        assert_eq!(glyph(0x20), &GLYPHS[0]);
        assert_eq!(glyph(0x7E), &GLYPHS[GLYPH_COUNT - 1]);
        assert_eq!(glyph(0x00), glyph(b'?'));
        assert_eq!(glyph(0xFF), glyph(b'?'));
        for code in 0x20u8..=0x7E {
            assert!(glyph(code).iter().all(|&row| row & !0b11111 == 0));
        }
    }

    #[test]
    fn masks_and_atlas_have_expected_layout() {
        let mask = rgba_mask(b'A');
        assert_eq!(mask.len(), GLYPH_W * GLYPH_H);
        assert!(mask.iter().all(|p| p[0] == 255 && p[1] == 255 && p[2] == 255));
        assert!(mask.iter().any(|p| p[3] == 255));
        assert!(mask.iter().any(|p| p[3] == 0));

        let (bytes, width, height) = atlas_rgba();
        assert_eq!(width, GLYPH_COUNT * GLYPH_W);
        assert_eq!(height, GLYPH_H);
        assert_eq!(bytes.len(), width * height * 4);
        let (a, _, _) = atlas_rgba();
        assert_eq!(bytes, a);
    }

    #[test]
    fn fnv1a_matches_known_vectors() {
        assert_eq!(fnv1a(""), 0x811c_9dc5);
        assert_eq!(fnv1a("a"), 0xe40c_292c);
        assert_eq!(fnv1a("foobar"), 0xbf9c_f968);
    }

    #[test]
    fn tables_have_expected_lengths() {
        assert_eq!(WEAPON_GROUPS.len(), 6);
        assert_eq!(HITBOX_NAMES.len(), 6);
        assert_eq!(CHAM_MATERIALS.len(), 20);
    }
}
