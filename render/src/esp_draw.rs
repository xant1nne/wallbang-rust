// Config block: shared ESP element rendering, 1:1 port of Celerity
// `src/features/visuals/` (`player_esp.cpp`, `visuals_shared.cpp`).
//
// Lives in the render crate (not `feature`) so the world overlay and the menu
// preview draw through the exact same code: the preview cannot drift into a
// "fake" look again. Pure DrawList work — no game reads happen here.
use crate::draw::DrawList;
use crate::pixel_font;
use wallbang_config::settings::{esp_bits, has_flag, EspPlayerSettings, Rgba};
use wallbang_core::bone::SKELETON_PAIRS;
use wallbang_core::math::Vec2;

/// Celerity draws flags with `get_pixel_font_height() + 2` line stride.
const FLAG_LINE: f32 = pixel_font::HEIGHT + 2.0;
/// Health bar geometry from `draw_health_bar`: 2px wide, 5px left of the box.
const HEALTH_BAR_WIDTH: f32 = 2.0;
const HEALTH_BAR_GAP: f32 = 5.0;
const OUTLINE_COLOR: Rgba = [0, 0, 0, 180];
/// Ammo bar offset below the box (`draw_ammo(..., offset_y = 2.0)`).
const AMMO_OFFSET: f32 = 2.0;
/// Gradient bars are split into at most this many slices.
const GRADIENT_STEPS: usize = 24;
/// Highest bone index the skeleton pairs touch (spine_3 lives at 23).
pub const BONE_SLOTS: usize = 24;

/// One ESP target reduced to screen space. Bones are indexed by the CS2 bone id
/// (`wallbang_core::bone::ids`); `None` means the point was not available.
pub struct EspTargetView<'a> {
    pub box_min: Vec2,
    pub box_max: Vec2,
    pub bones: [Option<Vec2>; BONE_SLOTS],
    pub name: Option<&'a str>,
    pub health: i32,
    pub animated_health: f32,
    pub armor: i32,
    pub money: i32,
    pub helmet: bool,
    pub scoped: bool,
    pub has_c4: bool,
    pub weapon: Option<&'a str>,
    pub clip: i32,
    pub max_clip: i32,
    pub distance: f32,
}

#[inline]
fn lerp_color(a: Rgba, b: Rgba, t: f32) -> Rgba {
    let t = t.clamp(0.0, 1.0);
    [
        (a[0] as f32 + (b[0] as f32 - a[0] as f32) * t) as u8,
        (a[1] as f32 + (b[1] as f32 - a[1] as f32) * t) as u8,
        (a[2] as f32 + (b[2] as f32 - a[2] as f32) * t) as u8,
        (a[3] as f32 + (b[3] as f32 - a[3] as f32) * t) as u8,
    ]
}

/// Vertical gradient, top `top` -> bottom `bottom` (Celerity health bar).
fn gradient_vertical(dl: &mut DrawList, x: f32, y: f32, w: f32, h: f32, top: Rgba, bottom: Rgba) {
    if h <= 0.0 {
        return;
    }
    let steps = (h.ceil() as usize).clamp(1, GRADIENT_STEPS);
    let slice = h / steps as f32;
    for index in 0..steps {
        let t = index as f32 / (steps.saturating_sub(1)).max(1) as f32;
        dl.rect(x, y + slice * index as f32, w, slice, lerp_color(top, bottom, t));
    }
}

/// Horizontal gradient, left `left` -> right `right` (Celerity ammo bar).
fn gradient_horizontal(dl: &mut DrawList, x: f32, y: f32, w: f32, h: f32, left: Rgba, right: Rgba) {
    if w <= 0.0 {
        return;
    }
    let steps = (w.ceil() as usize).clamp(1, GRADIENT_STEPS);
    let slice = w / steps as f32;
    for index in 0..steps {
        let t = index as f32 / (steps.saturating_sub(1)).max(1) as f32;
        dl.rect(x + slice * index as f32, y, slice, h, lerp_color(left, right, t));
    }
}

/// Celerity `draw_corners`: corner length is a fraction of the smaller box side;
/// `0.5` degenerates into a full rectangle with a 1px black outline.
fn draw_corners(
    dl: &mut DrawList,
    box_min: Vec2,
    box_max: Vec2,
    color: Rgba,
    outline: Rgba,
    corner_fraction: f32,
) {
    let x1 = box_min.x;
    let y1 = box_min.y;
    let x2 = box_max.x;
    let y2 = box_max.y;
    let width = (x2 - x1).max(1.0);
    let height = (y2 - y1).max(1.0);
    let fraction = corner_fraction.clamp(0.005, 0.5);
    if fraction >= 0.5 {
        dl.outline(x1 - 1.0, y1 - 1.0, width + 2.0, height + 2.0, 1.0, outline);
        dl.outline(x1 + 1.0, y1 + 1.0, width - 2.0, height - 2.0, 1.0, outline);
        dl.outline(x1, y1, width, height, 1.0, color);
        return;
    }
    let len = width.min(height) * fraction;
    let corners = [
        (x1, y1, 1.0f32, 1.0f32),
        (x2, y1, -1.0f32, 1.0f32),
        (x1, y2, 1.0f32, -1.0f32),
        (x2, y2, -1.0f32, -1.0f32),
    ];
    for (cx, cy, sx, _sy) in corners {
        let start = if sx > 0.0 { cx - 1.0 } else { cx - len };
        let end = if sx > 0.0 { cx + 1.0 + len } else { cx + 2.0 };
        dl.rect(start, cy - 1.0, end - start, 1.0, outline);
        let start = if sx > 0.0 { cx } else { cx - len };
        let end = if sx > 0.0 { cx + 1.0 + len } else { cx + 1.0 };
        dl.rect(start, cy, end - start, 1.0, color);
    }
    for (cx, cy, _sx, sy) in corners {
        let start = if sy > 0.0 { cy - 1.0 } else { cy - len };
        let end = if sy > 0.0 { cy + 1.0 + len } else { cy + 2.0 };
        dl.rect(cx - 1.0, start, 1.0, end - start, outline);
        let start = if sy > 0.0 { cy } else { cy - len };
        let end = if sy > 0.0 { cy + 1.0 + len } else { cy + 1.0 };
        dl.rect(cx, start, 1.0, end - start, color);
    }
}

fn draw_box(dl: &mut DrawList, cfg: &EspPlayerSettings, box_min: Vec2, box_max: Vec2) {
    if !cfg.r#box {
        return;
    }
    draw_corners(
        dl,
        box_min,
        box_max,
        cfg.box_color,
        OUTLINE_COLOR,
        cfg.corner_length,
    );
}

fn draw_health_bar(
    dl: &mut DrawList,
    cfg: &EspPlayerSettings,
    health: i32,
    animated_health: f32,
    box_min: Vec2,
    box_max: Vec2,
) {
    if !cfg.health_bar || health <= 0 || animated_health <= 0.0 {
        return;
    }
    let fraction = animated_health.min(100.0) / 100.0;
    let box_h = box_max.y - box_min.y;
    let fill_h = box_h * fraction;
    let bar_x = box_min.x - HEALTH_BAR_GAP;
    let bar_y = box_max.y - fill_h;
    let show_bar = has_flag(cfg.health_type, esp_bits::BAR);
    let show_gradient = has_flag(cfg.health_type, esp_bits::GRADIENT);
    let show_text = has_flag(cfg.health_type, esp_bits::TEXT);

    let draw_value = |dl: &mut DrawList, text_x_base: f32, text_y_base: f32, centered: bool| {
        let text = format!("{health}");
        let text_w = pixel_font::text_width(&text);
        let text_x = if centered {
            (text_x_base - text_w * 0.5 + 1.0).floor()
        } else {
            (text_x_base - text_w - 2.0).floor()
        };
        let text_y = if centered {
            (text_y_base - pixel_font::HEIGHT - 1.0).floor()
        } else {
            text_y_base
        };
        pixel_font::draw(dl, text_x, text_y, [255, 255, 255, 255], &text);
    };

    if show_bar {
        dl.rect(
            bar_x - 1.0,
            box_min.y - 1.0,
            HEALTH_BAR_WIDTH + 2.0,
            box_h + 2.0,
            OUTLINE_COLOR,
        );
        if show_gradient {
            let top = lerp_color(cfg.health_low_color, cfg.health_high_color, fraction);
            gradient_vertical(
                dl,
                bar_x,
                bar_y,
                HEALTH_BAR_WIDTH,
                fill_h,
                top,
                cfg.health_low_color,
            );
        } else {
            dl.rect(bar_x, bar_y, HEALTH_BAR_WIDTH, fill_h, cfg.health_high_color);
        }
        if show_text && health < 100 {
            draw_value(dl, bar_x, bar_y, true);
        }
    } else if show_text && health < 100 {
        draw_value(dl, box_min.x - HEALTH_BAR_GAP, box_min.y - 2.0, false);
    }
}

fn draw_skeleton(
    dl: &mut DrawList,
    cfg: &EspPlayerSettings,
    bones: &[Option<Vec2>; BONE_SLOTS],
    box_min: Vec2,
    box_max: Vec2,
) {
    if !cfg.skeleton {
        return;
    }
    let box_h = (box_max.y - box_min.y).max(1.0);
    let max_segment = box_h * 0.48;
    let min_x = box_min.x - box_h * 0.35;
    let max_x = box_max.x + box_h * 0.35;
    let min_y = box_min.y - box_h * 0.08;
    let max_y = box_max.y + box_h * 0.08;
    for (index_a, index_b) in SKELETON_PAIRS {
        let (Some(a), Some(b)) = (bones[index_a], bones[index_b]) else {
            continue;
        };
        if a.x < min_x || a.x > max_x || a.y < min_y || a.y > max_y {
            continue;
        }
        if b.x < min_x || b.x > max_x || b.y < min_y || b.y > max_y {
            continue;
        }
        let dx = a.x - b.x;
        let dy = a.y - b.y;
        if dx * dx + dy * dy > max_segment * max_segment {
            continue;
        }
        dl.line(a.x, a.y, b.x, b.y, 1.0, cfg.skeleton_color);
    }
}

fn draw_name(
    dl: &mut DrawList,
    cfg: &EspPlayerSettings,
    name: Option<&str>,
    box_min: Vec2,
    box_max: Vec2,
) {
    if !cfg.name {
        return;
    }
    let Some(name) = name else {
        return;
    };
    let name = name.to_ascii_lowercase();
    if name.is_empty() {
        return;
    }
    let text_w = dl.text_width(&name, 1.0);
    let text_h = dl.text_height(1.0);
    let x = (box_min.x + (box_max.x - box_min.x - text_w) * 0.5).floor();
    let y = (box_min.y - text_h - 2.0).floor();
    dl.text(x + 1.0, y + 1.0, &name, [0, 0, 0, 255], 1.0);
    dl.text(x, y, &name, cfg.name_color, 1.0);
}

fn draw_flags(dl: &mut DrawList, cfg: &EspPlayerSettings, target: &EspTargetView, box_min: Vec2, box_max: Vec2) {
    if !cfg.flags {
        return;
    }
    let color = cfg.flags_color;
    let x = box_max.x + 3.0;
    let mut y = box_min.y;
    let line = |dl: &mut DrawList, y: &mut f32, text: &str| {
        pixel_font::draw(dl, x, *y, color, text);
        *y += FLAG_LINE;
    };
    if has_flag(cfg.flags_type, esp_bits::FLAG_DISTANCE) {
        line(dl, &mut y, &format!("{:.0}m", target.distance));
    }
    if has_flag(cfg.flags_type, esp_bits::FLAG_CASH) && target.money > 0 {
        line(dl, &mut y, &format!("${}", target.money));
    }
    if has_flag(cfg.flags_type, esp_bits::FLAG_ARMOR) && target.armor > 0 {
        line(dl, &mut y, if target.helmet { "HK" } else { "K" });
    }
    if has_flag(cfg.flags_type, esp_bits::FLAG_SCOPED) && target.scoped {
        line(dl, &mut y, "scoped");
    }
    if has_flag(cfg.flags_type, esp_bits::FLAG_BOMB) && target.has_c4 {
        line(dl, &mut y, "bomb");
    }
}

fn draw_ammo(
    dl: &mut DrawList,
    cfg: &EspPlayerSettings,
    target: &EspTargetView,
    box_min: Vec2,
    box_max: Vec2,
    offset_y: f32,
) -> f32 {
    if !cfg.ammo || target.clip < 0 || target.max_clip <= 0 {
        return 0.0;
    }
    let fraction = target.clip as f32 / target.max_clip as f32;
    let box_w = box_max.x - box_min.x;
    let fill_w = box_w * fraction;
    let bar_y = box_max.y + offset_y;
    let mut total_height = 0.0;
    if has_flag(cfg.ammo_type, esp_bits::BAR) {
        dl.rect(box_min.x - 1.0, bar_y - 1.0, box_w + 2.0, 4.0, OUTLINE_COLOR);
        if has_flag(cfg.ammo_type, esp_bits::GRADIENT) {
            let right = lerp_color(cfg.ammo_low_color, cfg.ammo_high_color, fraction);
            gradient_horizontal(dl, box_min.x, bar_y, fill_w, 2.0, cfg.ammo_low_color, right);
        } else {
            dl.rect(box_min.x, bar_y, fill_w, 2.0, cfg.ammo_high_color);
        }
        total_height += 4.0;
    }
    if has_flag(cfg.ammo_type, esp_bits::TEXT) {
        let text = format!("{} / {}", target.clip, target.max_clip);
        let text_w = pixel_font::text_width(&text);
        let x = (box_min.x + (box_w - text_w) * 0.5).floor();
        let y = if total_height > 0.0 { bar_y + 4.0 } else { bar_y };
        pixel_font::draw(dl, x, y, [255, 255, 255, 255], &text);
        total_height += pixel_font::HEIGHT;
    }
    total_height
}

fn draw_weapon(
    dl: &mut DrawList,
    cfg: &EspPlayerSettings,
    target: &EspTargetView,
    box_min: Vec2,
    box_max: Vec2,
    offset_y: f32,
) -> f32 {
    if !cfg.weapon {
        return 0.0;
    }
    let Some(name) = target.weapon else {
        return 0.0;
    };
    let text = name.to_ascii_lowercase();
    let text_w = pixel_font::text_width(&text);
    let x = (box_min.x + (box_max.x - box_min.x - text_w) * 0.5).floor();
    let y = box_max.y + offset_y;
    pixel_font::draw(dl, x, y, cfg.weapon_color, &text);
    pixel_font::HEIGHT
}

/// Draws every enabled ESP element for one target. Shared by the world overlay
/// and the menu preview.
pub fn draw_target(dl: &mut DrawList, cfg: &EspPlayerSettings, target: &EspTargetView) {
    if !cfg.enabled {
        return;
    }
    let (box_min, box_max) = (target.box_min, target.box_max);
    if box_max.x <= box_min.x || box_max.y <= box_min.y {
        return;
    }
    draw_box(dl, cfg, box_min, box_max);
    draw_health_bar(dl, cfg, target.health, target.animated_health, box_min, box_max);
    draw_skeleton(dl, cfg, &target.bones, box_min, box_max);
    draw_name(dl, cfg, target.name, box_min, box_max);
    draw_flags(dl, cfg, target, box_min, box_max);
    let mut offset_y = AMMO_OFFSET;
    let ammo_height = draw_ammo(dl, cfg, target, box_min, box_max, offset_y);
    if ammo_height > 0.0 {
        offset_y += ammo_height + 2.0;
    }
    draw_weapon(dl, cfg, target, box_min, box_max, offset_y);
}

#[cfg(test)]
mod tests {
    use super::{draw_target, lerp_color, EspTargetView, BONE_SLOTS};
    use wallbang_config::settings::EspPlayerSettings;
    use wallbang_core::math::Vec2;

    #[test]
    fn colours_interpolate_both_ends() {
        let a = [255, 102, 170, 255];
        let b = [255, 255, 255, 255];
        assert_eq!(lerp_color(a, b, 0.0), a);
        assert_eq!(lerp_color(a, b, 1.0), b);
        assert_eq!(lerp_color(a, b, 0.5), [255, 178, 212, 255]);
    }

    #[test]
    fn target_draws_with_celerity_geometry() {
        let mut dl = crate::draw::DrawList::new();
        dl.begin(400.0, 300.0);
        let mut cfg = EspPlayerSettings::default();
        cfg.enabled = true;
        cfg.r#box = true;
        cfg.health_bar = true;
        cfg.ammo = true;
        cfg.weapon = true;
        let target = EspTargetView {
            box_min: Vec2::new(100.0, 50.0),
            box_max: Vec2::new(160.0, 170.0),
            bones: [None; BONE_SLOTS],
            name: Some("player"),
            health: 84,
            animated_health: 84.0,
            armor: 100,
            money: 800,
            helmet: true,
            scoped: false,
            has_c4: false,
            weapon: Some("AK-47"),
            clip: 30,
            max_clip: 90,
            distance: 12.0,
        };
        draw_target(&mut dl, &cfg, &target);
        assert!(!dl.vertices().is_empty());
    }
}
