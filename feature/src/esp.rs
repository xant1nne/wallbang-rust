// Config block: только чтение Snapshot + DrawList; запись в игру отсутствует.
use wallbang_config::ui_state::CATALOG_ITEMS;
use wallbang_core::state::GameState;
use wallbang_core::{bone::BoneArray, math::w2s};
use wallbang_render::DrawList;

const HEAD_BONE: usize = 6;
const CHEST_BONE: usize = 4;
const STOMACH_BONE: usize = 3;
const FOOT_BONE: usize = 14;

#[inline]
fn format_hp(hp: i32, buf: &mut [u8]) -> &str {
    let mut n = hp as u32;
    if n > 999 {
        n = 999;
    }
    if n == 0 {
        return "0";
    }
    let mut i = buf.len();
    while n > 0 {
        i -= 1;
        buf[i] = b'0' + (n % 10) as u8;
        n /= 10;
    }
    std::str::from_utf8(&buf[i..]).unwrap_or("0")
}

#[inline]
fn format_i32(value: i32, buf: &mut [u8; 12]) -> &str {
    let negative = value < 0;
    let mut n = value.unsigned_abs().min(999_999_999);
    if n == 0 && !negative {
        return "0";
    }
    let mut i = buf.len();
    while n > 0 {
        i -= 1;
        buf[i] = b'0' + (n % 10) as u8;
        n /= 10;
    }
    if negative {
        i -= 1;
        buf[i] = b'-';
    }
    std::str::from_utf8(&buf[i..]).unwrap_or("0")
}

#[inline]
fn weapon_name(def_index: u16) -> &'static str {
    CATALOG_ITEMS
        .iter()
        .find(|item| item.def_index == def_index as i16)
        .map(|item| item.name)
        .unwrap_or("weapon")
}

pub fn draw_esp(dl: &mut DrawList, state: &GameState) {
    let snapshot = state.snapshot();
    if !snapshot.valid {
        return;
    }
    let settings = state.settings();
    let esp = &settings.esp;
    if !esp.enabled {
        return;
    }
    let (sw, sh) = (snapshot.screen_w, snapshot.screen_h);
    let local = snapshot.local_origin;
    let flags = (
        esp.r#box,
        esp.skeleton,
        esp.name,
        esp.health,
        esp.armor,
        esp.distance,
        esp.weapon,
        esp.ammo,
    );
    let style = (
        esp.box_style,
        esp.corner_size,
        esp.box_color,
        esp.skeleton_thickness,
        esp.skeleton_visible_color,
        esp.skeleton_occluded_color,
        esp.health_bar,
        esp.health_pos,
        esp.name_color,
        esp.armor_color,
        esp.distance_color,
        esp.visible_only,
    );
    drop(settings);

    let mut number_buf = [0u8; 12];
    for player in snapshot.players.iter() {
        if !player.alive || (style.11 && !player.spotted) {
            continue;
        }
        let distance = player.origin.distance_to(local) / 32.0;
        if distance > 200.0 {
            continue;
        }
        let head = w2s(&snapshot.view_matrix, player.origin, sw, sh);
        let feet = w2s(
            &snapshot.view_matrix,
            wallbang_core::math::Vec3::new(player.origin.x, player.origin.y, player.origin.z - 72.0),
            sw,
            sh,
        );
        let (Some(head), Some(feet)) = (head, feet) else { continue };

        let box_color = if player.spotted {
            style.2
        } else {
            [style.2[0] / 2, style.2[1] / 2, style.2[2] / 2, style.2[3]]
        };
        let half = (head.y - feet.y).abs() * 0.22;
        let width = half * 0.85;
        let top = head.y - 10.0;
        let bottom = feet.y + 4.0;
        let bx = head.x - width;
        let bw = width * 2.0;
        let bh = bottom - top;

        if flags.0 {
            if style.0 == 0 {
                let len = style.1.clamp(4.0, 24.0);
                let slant = 3.0;
                for (x, y, sx, sy) in [
                    (bx, top, 1.0, 1.0),
                    (bx + bw, top, -1.0, 1.0),
                    (bx, bottom, 1.0, -1.0),
                    (bx + bw, bottom, -1.0, -1.0),
                ] {
                    dl.line(x, y, x + sx * slant, y + sy * len, 1.5, box_color);
                    dl.line(x + sx * slant, y + sy * len, x + sx * len, y + sy * len, 1.5, box_color);
                }
            } else {
                dl.outline(bx, top, bw, bh, 1.0, box_color);
            }
        }
        if flags.1 {
            if let Some(bones) = unsafe { BoneArray::from_pawn(player.address) } {
                let chain = [
                    (HEAD_BONE, CHEST_BONE),
                    (CHEST_BONE, STOMACH_BONE),
                    (STOMACH_BONE, FOOT_BONE),
                ];
                let skel_color = if player.spotted { style.4 } else { style.5 };
                for (a, b) in chain {
                    let (Some(pa), Some(pb)) = (unsafe { bones.position(a) }, unsafe { bones.position(b) }) else {
                        continue;
                    };
                    let (Some(sa), Some(sb)) = (
                        w2s(&snapshot.view_matrix, pa, sw, sh),
                        w2s(&snapshot.view_matrix, pb, sw, sh),
                    ) else {
                        continue;
                    };
                    dl.line(sa.x, sa.y, sb.x, sb.y, style.3.max(0.5), skel_color);
                }
            }
        }

        if style.6 && bh > 4.0 {
            let frac = (player.health.clamp(0, 100) as f32) / 100.0;
            let (bar_x, bar_y) = match style.7 {
                1 => (bx - 7.0, top),
                2 => (bx + bw + 3.0, top),
                _ => (bx, top - 7.0),
            };
            dl.rect(bar_x, bar_y, 4.0, bh.min(120.0), [18, 20, 24, 220]);
            dl.rect(
                bar_x,
                bar_y + bh.min(120.0) * (1.0 - frac),
                4.0,
                bh.min(120.0) * frac,
                [110, 228, 145, 255],
            );
        }

        let mut line_y = head.y - 12.0;
        if flags.2 {
            // Real name from m_sSanitizedPlayerName; the neutral label is only
            // a fallback while the controller has not spawned yet.
            let text = player
                .name
                .as_deref()
                .unwrap_or(if player.enemy { "Enemy" } else { "Team" });
            dl.text(head.x - dl.text_width(text, 1.0) * 0.5, line_y, text, style.8, 1.0);
            line_y += 10.0;
        }
        if flags.3 {
            let hp = format_hp(player.health, &mut number_buf);
            dl.text(head.x - dl.text_width(hp, 1.0) * 0.5, line_y, hp, [120, 230, 140, 255], 1.0);
            line_y += 10.0;
        }
        if flags.4 {
            let armor = format_i32(player.armor.max(0), &mut number_buf);
            let text = format!("{}ar", armor);
            dl.text(head.x - dl.text_width(&text, 1.0) * 0.5, line_y, &text, style.9, 1.0);
            line_y += 10.0;
        }
        if flags.5 {
            let dist = format!("{}m", distance as u32);
            dl.text(head.x - dl.text_width(&dist, 1.0) * 0.5, line_y, &dist, style.10, 1.0);
            line_y += 10.0;
        }
        if flags.6 {
            let name = weapon_name(player.weapon.def_index);
            dl.text(head.x - dl.text_width(name, 1.0) * 0.5, line_y, name, [200, 205, 215, 255], 1.0);
            line_y += 10.0;
        }
        if flags.7 && player.weapon.def_index != 0 {
            let clip = format_i32(player.weapon.clip, &mut number_buf);
            let ammo = if player.weapon.max_clip > 0 {
                format!("{}/{}", clip, player.weapon.max_clip)
            } else {
                clip.to_owned()
            };
            dl.text(head.x - dl.text_width(&ammo, 1.0) * 0.5, line_y, &ammo, [220, 225, 235, 255], 1.0);
        }
    }
}
