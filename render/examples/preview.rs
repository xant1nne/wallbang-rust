use std::fs::File;
use std::io::Write;
use wallbang_core::state::GameState;
use wallbang_render::draw::{DrawList, Vertex};
use wallbang_render::font::atlas_rgba;
use wallbang_render::menu::Menu;

fn rasterize(
    verts: &[Vertex],
    atlas: &[u8],
    aw: usize,
    ah: usize,
    width: usize,
    height: usize,
    bg: [u8; 4],
) -> Vec<u8> {
    let mut fb = vec![0u8; width * height * 4];
    for px in fb.chunks_exact_mut(4) {
        px.copy_from_slice(&bg);
    }

    let to_px = |v: &Vertex| -> (f32, f32) {
        (
            (v.x + 1.0) * 0.5 * width as f32,
            (1.0 - v.y) * 0.5 * height as f32,
        )
    };

    for tri in verts.chunks_exact(3) {
        let (x0, y0) = to_px(&tri[0]);
        let (x1, y1) = to_px(&tri[1]);
        let (x2, y2) = to_px(&tri[2]);

        let min_x = x0.min(x1).min(x2).floor().max(0.0) as usize;
        let max_x = x0.max(x1).max(x2).ceil().min((width - 1) as f32) as usize;
        let min_y = y0.min(y1).min(y2).floor().max(0.0) as usize;
        let max_y = y0.max(y1).max(y2).ceil().min((height - 1) as f32) as usize;

        let area = (x1 - x0) * (y2 - y0) - (y1 - y0) * (x2 - x0);
        if area.abs() < 1e-5 {
            continue;
        }
        let inv_area = 1.0 / area;

        for py in min_y..=max_y {
            let fy = py as f32 + 0.5;
            for px in min_x..=max_x {
                let fx = px as f32 + 0.5;
                let w0 = ((x1 - fx) * (y2 - fy) - (y1 - fy) * (x2 - fx)) * inv_area;
                let w1 = ((x2 - fx) * (y0 - fy) - (y2 - fy) * (x0 - fx)) * inv_area;
                let w2 = 1.0 - w0 - w1;
                if w0 < -1e-4 || w1 < -1e-4 || w2 < -1e-4 {
                    continue;
                }
                let u = (w0 * tri[0].u + w1 * tri[1].u + w2 * tri[2].u).clamp(0.0, 0.9999);
                let v = (w0 * tri[0].v + w1 * tri[1].v + w2 * tri[2].v).clamp(0.0, 0.9999);
                let tx = (u * aw as f32) as usize;
                let ty = (v * ah as f32) as usize;
                let ti = (ty * aw + tx) * 4;
                let ta = atlas[ti + 3] as f32 / 255.0;
                if ta <= 0.001 {
                    continue;
                }
                let r = (w0 * tri[0].r as f32 + w1 * tri[1].r as f32 + w2 * tri[2].r as f32)
                    * (atlas[ti] as f32 / 255.0);
                let g = (w0 * tri[0].g as f32 + w1 * tri[1].g as f32 + w2 * tri[2].g as f32)
                    * (atlas[ti + 1] as f32 / 255.0);
                let b = (w0 * tri[0].b as f32 + w1 * tri[1].b as f32 + w2 * tri[2].b as f32)
                    * (atlas[ti + 2] as f32 / 255.0);
                let a = ((w0 * tri[0].a as f32 + w1 * tri[1].a as f32 + w2 * tri[2].a as f32)
                    / 255.0)
                    * ta;

                let idx = (py * width + px) * 4;
                let inv_a = 1.0 - a;
                fb[idx] = (r * a + fb[idx] as f32 * inv_a).clamp(0.0, 255.0) as u8;
                fb[idx + 1] = (g * a + fb[idx + 1] as f32 * inv_a).clamp(0.0, 255.0) as u8;
                fb[idx + 2] = (b * a + fb[idx + 2] as f32 * inv_a).clamp(0.0, 255.0) as u8;
                fb[idx + 3] = 255;
            }
        }
    }
    fb
}

fn crc32(data: &[u8]) -> u32 {
    let mut crc = 0xFFFF_FFFFu32;
    for &b in data {
        crc ^= b as u32;
        for _ in 0..8 {
            crc = if crc & 1 != 0 {
                (crc >> 1) ^ 0xEDB8_8320
            } else {
                crc >> 1
            };
        }
    }
    !crc
}

fn adler32(data: &[u8]) -> u32 {
    let mut a = 1u32;
    let mut b = 0u32;
    for &byte in data {
        a = (a + byte as u32) % 65521;
        b = (b + a) % 65521;
    }
    (b << 16) | a
}

fn write_chunk(out: &mut Vec<u8>, tag: &[u8; 4], payload: &[u8]) {
    out.extend_from_slice(&(payload.len() as u32).to_be_bytes());
    let start = out.len();
    out.extend_from_slice(tag);
    out.extend_from_slice(payload);
    let crc = crc32(&out[start..]);
    out.extend_from_slice(&crc.to_be_bytes());
}

fn save_png(path: &str, width: u32, height: u32, rgba: &[u8]) {
    let mut png = Vec::new();
    png.extend_from_slice(&[137, 80, 78, 71, 13, 10, 26, 10]);

    let mut ihdr = Vec::with_capacity(13);
    ihdr.extend_from_slice(&width.to_be_bytes());
    ihdr.extend_from_slice(&height.to_be_bytes());
    ihdr.extend_from_slice(&[8, 6, 0, 0, 0]); // 8-bit RGBA
    write_chunk(&mut png, b"IHDR", &ihdr);

    let stride = (width as usize) * 4;
    let mut raw = Vec::with_capacity((stride + 1) * height as usize);
    for y in 0..height as usize {
        raw.push(0); // filter: None
        raw.extend_from_slice(&rgba[y * stride..(y + 1) * stride]);
    }

    // Valid zlib stream using uncompressed DEFLATE blocks (RFC 1950/1951)
    let mut zlib = Vec::with_capacity(raw.len() + raw.len() / 65535 * 5 + 16);
    zlib.push(0x78);
    zlib.push(0x01);
    let mut offset = 0usize;
    while offset < raw.len() {
        let len = (raw.len() - offset).min(65535);
        let is_last = offset + len >= raw.len();
        zlib.push(if is_last { 0x01 } else { 0x00 });
        let l16 = len as u16;
        zlib.extend_from_slice(&l16.to_le_bytes());
        zlib.extend_from_slice(&(!l16).to_le_bytes());
        zlib.extend_from_slice(&raw[offset..offset + len]);
        offset += len;
    }
    zlib.extend_from_slice(&adler32(&raw).to_be_bytes());
    write_chunk(&mut png, b"IDAT", &zlib);
    write_chunk(&mut png, b"IEND", &[]);

    let mut file = File::create(path).expect("create preview png");
    file.write_all(&png).expect("write preview png");
}

fn render_one(
    menu: &mut Menu,
    state: &GameState,
    atlas: &[u8],
    aw: usize,
    ah: usize,
    w: usize,
    h: usize,
    tab: usize,
    subtab: usize,
    path: &str,
) {
    let mut dl = DrawList::new();
    dl.begin(w as f32, h as f32);
    // Prime popup owners on frame 1, then render clean frame 2
    menu.render_preview_frame(&mut dl, state, w as f32, h as f32, tab, subtab);
    dl.begin(w as f32, h as f32);
    menu.render_preview_frame(&mut dl, state, w as f32, h as f32, tab, subtab);
    let fb = rasterize(dl.vertices(), atlas, aw, ah, w, h, [7, 8, 10, 255]);
    save_png(path, w as u32, h as u32, &fb);
    println!("Saved {} ({} verts)", path, dl.len());
}

fn main() {
    let (atlas, aw, ah) = atlas_rgba();
    let state = GameState::new();
    {
        let mut s = state.settings_mut();
        s.menu.x = 240.0;
        s.menu.y = 56.0;
        s.menu.w = 848.0;
        s.menu.h = 580.0;
        s.menu.accent = [164, 204, 232, 255];
        s.misc.watermark = true;
        s.combat.ragebot.enabled = true;
        s.combat.ragebot.silent = true;
        s.combat.ragebot.weapon_group = 2; // Rifles
        s.combat.ragebot.groups[2].max_fov = 180.0;
        s.combat.ragebot.groups[2].min_damage = 30;
        s.combat.ragebot.groups[2].pointscale = 75.0;
        s.combat.ragebot.groups[2].hitchance = 60;
        s.combat.ragebot.groups[2].autostop = true;
        s.combat.antiaim.enabled = true;
        s.combat.antiaim.pitch = 1;
        s.combat.antiaim.auto_yaw_adjust = true;
        s.combat.antiaim.direction_indicator = true;
        s.combat.zeusbot.enabled = true;
        s.combat.knifebot.enabled = true;
    }

    let mut menu = Menu::new();
    // New shell: sidebar + animated 3D ESP preview on the first tab.
    render_one(
        &mut menu,
        &state,
        atlas,
        aw,
        ah,
        1320,
        750,
        0,
        0,
        r"C:\wallbang-rust\target\tmp\menu_preview.png",
    );
    // Legitbot tab (real aim-assist panel)
    {
        let mut s = state.settings_mut();
        s.combat.legit.enabled = true;
        s.combat.legit.weapons = 2;
        s.combat.legit.groups[2].aimbot = true;
        s.combat.legit.groups[2].fov = 4.0;
        s.combat.legit.groups[2].smooth = 12;
    }
    render_one(
        &mut menu,
        &state,
        atlas,
        aw,
        ah,
        1320,
        750,
        1,
        0,
        r"C:\wallbang-rust\target\tmp\menu_preview_legit.png",
    );
    // Players tab + floating 3D ESP preview panel
    state.settings_mut().menu.x = 236.0;
    render_one(
        &mut menu,
        &state,
        atlas,
        aw,
        ah,
        1340,
        750,
        2,
        0,
        r"C:\wallbang-rust\target\tmp\menu_preview_players.png",
    );
    // Skins grid
    state.settings_mut().menu.x = 240.0;
    render_one(
        &mut menu,
        &state,
        atlas,
        aw,
        ah,
        1320,
        750,
        4,
        0,
        r"C:\wallbang-rust\target\tmp\menu_preview_skins.png",
    );
    // Visuals / World tabs (layout regression checks)
    for (subtab, path) in [
        (0, r"C:\wallbang-rust\target\tmp\menu_preview_world_items.png"),
        (1, r"C:\wallbang-rust\target\tmp\menu_preview_world_scene.png"),
        (2, r"C:\wallbang-rust\target\tmp\menu_preview_world_weather.png"),
    ] {
        render_one(&mut menu, &state, atlas, aw, ah, 1320, 750, 3, subtab, path);
    }
}
