// Smooth anti-aliased TrueType font atlas generated via Win32 GDI.
// Embedded fonts match the C++ build 1:1: Inter Bold for widgets, Rajdhani
// SemiBold for the brand wordmark. Registering them in-process avoids relying
// on fonts installed on the user's machine and removes the "soapy" Segoe look.
use std::ffi::c_void;
use std::sync::OnceLock;
use windows::core::PCWSTR;
use windows::Win32::Graphics::Gdi::*;

pub const ATLAS_W: usize = 2048;
pub const ATLAS_H: usize = 512;

const INTER_BOLD: &[u8] = include_bytes!("fonts/inter_bold.ttf");
const RAJDHANI_SEMIBOLD: &[u8] = include_bytes!("fonts/rajdhani_semibold.ttf");
pub const FIRST_CHAR: u8 = 0x20;
pub const LAST_CHAR: u8 = 0x7E;
const ASCII_COUNT: usize = 95;
const LATIN_COUNT: usize = 96;
const CYRILLIC_COUNT: usize = 256;
pub const CHAR_COUNT: usize = ASCII_COUNT + LATIN_COUNT + CYRILLIC_COUNT + 112;
pub fn glyph_index(ch: char) -> usize {
    match ch as u32 {
        0x20..=0x7e => ch as usize - 0x20,
        0xa0..=0xff => ASCII_COUNT + ch as usize - 0xa0,
        0x400..=0x4ff => ASCII_COUNT + LATIN_COUNT + ch as usize - 0x400,
        0x2000..=0x206f => ASCII_COUNT + LATIN_COUNT + CYRILLIC_COUNT + ch as usize - 0x2000,
        _ => (b'?' - FIRST_CHAR) as usize,
    }
}
fn glyph_code(index: usize) -> u16 {
    if index < ASCII_COUNT { (0x20 + index) as u16 }
    else if index < ASCII_COUNT + LATIN_COUNT { (0xa0 + index - ASCII_COUNT) as u16 }
    else if index < ASCII_COUNT + LATIN_COUNT + CYRILLIC_COUNT { (0x400 + index - ASCII_COUNT - LATIN_COUNT) as u16 }
    else { (0x2000 + index - ASCII_COUNT - LATIN_COUNT - CYRILLIC_COUNT) as u16 }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct GlyphInfo {
    pub u0: f32,
    pub v0: f32,
    pub u1: f32,
    pub v1: f32,
    pub w: f32,
    pub h: f32,
    pub advance: f32,
}

pub struct FontAtlas {
    pub rgba: Vec<u8>,
    pub width: usize,
    pub height: usize,
    pub white_u: f32,
    pub white_v: f32,
    pub ui_glyphs: [GlyphInfo; CHAR_COUNT],
    pub ui_line_h: f32,
    pub brand_glyphs: [GlyphInfo; CHAR_COUNT],
    pub brand_line_h: f32,
}

static ATLAS: OnceLock<FontAtlas> = OnceLock::new();

pub fn get() -> &'static FontAtlas {
    ATLAS.get_or_init(build_atlas)
}

pub fn atlas_rgba() -> (&'static [u8], usize, usize) {
    let a = get();
    (&a.rgba, a.width, a.height)
}

fn wide_name(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

/// Adds the embedded TTF files to the process font table. The byte slices are
/// static, so GDI can keep referencing them for the whole process lifetime.
fn register_embedded_fonts() {
    unsafe {
        let mut count = 0u32;
        let _ = AddFontMemResourceEx(
            INTER_BOLD.as_ptr() as *const c_void,
            INTER_BOLD.len() as u32,
            None,
            &mut count,
        );
        let mut count = 0u32;
        let _ = AddFontMemResourceEx(
            RAJDHANI_SEMIBOLD.as_ptr() as *const c_void,
            RAJDHANI_SEMIBOLD.len() as u32,
            None,
            &mut count,
        );
    }
}

fn rasterize_face(
    hdc: HDC,
    rgba: &mut [u8],
    face_name: &str,
    height_px: i32,
    weight: i32,
    start_x: usize,
    start_y: usize,
) -> ([GlyphInfo; CHAR_COUNT], f32) {
    let mut glyphs = [GlyphInfo::default(); CHAR_COUNT];
    let mut line_h = height_px.unsigned_abs() as f32;

    unsafe {
        let name_w = wide_name(face_name);
        let font = CreateFontW(
            -height_px,
            0,
            0,
            0,
            weight,
            0,
            0,
            0,
            DEFAULT_CHARSET,
            OUT_TT_PRECIS,
            CLIP_DEFAULT_PRECIS,
            ANTIALIASED_QUALITY,
            FF_SWISS.0 as u32 | DEFAULT_PITCH.0 as u32,
            PCWSTR(name_w.as_ptr()),
        );
        let old_font = SelectObject(hdc, HGDIOBJ(font.0));

        let mut tm = TEXTMETRICW::default();
        if GetTextMetricsW(hdc, &mut tm).as_bool() {
            line_h = tm.tmHeight as f32;
        }

        // Scratch 48x48 DIB per glyph for crisp alpha extraction
        let cell_w = 48i32;
        let cell_h = 48i32;
        let bmi = BITMAPINFO {
            bmiHeader: BITMAPINFOHEADER {
                biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                biWidth: cell_w,
                biHeight: -cell_h, // top-down
                biPlanes: 1,
                biBitCount: 32,
                biCompression: BI_RGB.0,
                ..Default::default()
            },
            ..Default::default()
        };
        let mut bits_ptr: *mut c_void = std::ptr::null_mut();
        let bmp = CreateDIBSection(Some(hdc), &bmi, DIB_RGB_COLORS, &mut bits_ptr, None, 0)
            .expect("CreateDIBSection");
        let old_bmp = SelectObject(hdc, HGDIOBJ(bmp.0));
        SetBkMode(hdc, OPAQUE);
        SetBkColor(hdc, windows::Win32::Foundation::COLORREF(0x0000_0000));
        SetTextColor(hdc, windows::Win32::Foundation::COLORREF(0x00FF_FFFF));

        let mut pen_x = start_x;
        let mut pen_y = start_y;
        let row_h = (line_h.ceil() as usize + 4).max(18);
        let bits = std::slice::from_raw_parts_mut(bits_ptr as *mut u8, (cell_w * cell_h * 4) as usize);

        for idx in 0..CHAR_COUNT {
            let ch_u16 = [glyph_code(idx)];
            let mut sz = windows::Win32::Foundation::SIZE::default();
            let _ = GetTextExtentPoint32W(hdc, &ch_u16, &mut sz);
            let gw = (sz.cx.max(1) as usize).min(cell_w as usize - 2);
            let gh = (sz.cy.max(1) as usize).min(cell_h as usize - 2);

            if pen_x + gw + 2 >= ATLAS_W {
                pen_x = 4;
                pen_y += row_h;
            }
            if pen_y + gh + 1 >= ATLAS_H {
                break;
            }

            // Clear scratch buffer to black
            bits.fill(0);
            let _ = TextOutW(hdc, 1, 0, &ch_u16);
            let _ = GdiFlush();

            for y in 0..gh {
                for x in 0..(gw + 1) {
                    let si = (y * cell_w as usize + x) * 4;
                    let lum = bits[si].max(bits[si + 1]).max(bits[si + 2]);
                    // Keep the native coverage curve: the previous gamma boost
                    // made 14px text look soft/soapy next to the C++ build.
                    let alpha = lum;
                    let dx = pen_x + x;
                    let dy = pen_y + y;
                    let di = (dy * ATLAS_W + dx) * 4;
                    rgba[di] = 255;
                    rgba[di + 1] = 255;
                    rgba[di + 2] = 255;
                    rgba[di + 3] = alpha;
                }
            }

            let rw = (gw + 1) as f32;
            let rh = gh as f32;
            glyphs[idx] = GlyphInfo {
                u0: pen_x as f32 / ATLAS_W as f32,
                v0: pen_y as f32 / ATLAS_H as f32,
                u1: (pen_x as f32 + rw) / ATLAS_W as f32,
                v1: (pen_y as f32 + rh) / ATLAS_H as f32,
                w: rw,
                h: rh,
                advance: sz.cx as f32,
            };
            pen_x += gw + 3;
        }

        SelectObject(hdc, old_bmp);
        let _ = DeleteObject(HGDIOBJ(bmp.0));
        SelectObject(hdc, old_font);
        let _ = DeleteObject(HGDIOBJ(font.0));
    }

    (glyphs, line_h)
}

fn build_atlas() -> FontAtlas {
    let mut rgba = vec![0u8; ATLAS_W * ATLAS_H * 4];
    // Solid 4x4 opaque white block at (0..4, 0..4) for solid geometry sampling
    for y in 0..4 {
        for x in 0..4 {
            let i = (y * ATLAS_W + x) * 4;
            rgba[i] = 255;
            rgba[i + 1] = 255;
            rgba[i + 2] = 255;
            rgba[i + 3] = 255;
        }
    }

    let (ui_glyphs, ui_line_h, brand_glyphs, brand_line_h) = unsafe {
        register_embedded_fonts();
        let hdc = CreateCompatibleDC(None);
        let (ui_g, ui_h) = rasterize_face(hdc, &mut rgba, "Inter", 14, FW_BOLD.0 as i32, 8, 2);
        let (br_g, br_h) =
            rasterize_face(hdc, &mut rgba, "Rajdhani", 20, FW_SEMIBOLD.0 as i32, 8, 256);
        let _ = DeleteDC(hdc);
        (ui_g, ui_h, br_g, br_h)
    };

    FontAtlas {
        rgba,
        width: ATLAS_W,
        height: ATLAS_H,
        white_u: 2.0 / ATLAS_W as f32,
        white_v: 2.0 / ATLAS_H as f32,
        ui_glyphs,
        ui_line_h,
        brand_glyphs,
        brand_line_h,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cyrillic_and_ukrainian_glyphs_are_rasterized() {
        let atlas = get();
        for ch in "Название трека ЖёЇїЄєҐґ".chars().filter(|c| *c != ' ') {
            let index = glyph_index(ch);
            assert_ne!(index, glyph_index('?'));
            let glyph = atlas.ui_glyphs[index];
            assert!(glyph.advance > 0.0);
            let x = (glyph.u0 * atlas.width as f32) as usize;
            let y = (glyph.v0 * atlas.height as f32) as usize;
            let ink = (0..glyph.h as usize).any(|dy| (0..glyph.w as usize).any(|dx|
                atlas.rgba[((y+dy)*atlas.width+x+dx)*4+3] > 0));
            assert!(ink, "empty glyph {ch}");
        }
    }
}
