// Panorama-драйвер реальной модели игрока: создаёт MapPlayerPreviewPanel
// и рисует ESP прямо на ней через нативные bone-проекции движка.
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;
use std::time::{Duration, Instant};
use wallbang_core::panorama::{Panorama, PanelMode};

const UPDATE_INTERVAL: Duration = Duration::from_millis(120);

const SCRIPT: &str = r#"
(function (a) {
    var root = $.GetContextPanel();
    while (root.GetParent()) root = root.GetParent();
    var data = root.Data();
    var key = 'WallbangAgentPreview';
    var state = data[key];
    if (!a.active) {
        if (state) state.dispose();
        return;
    }
    if (!state) {
        var host = $.CreatePanel('Panel', root, key + 'Host', { hittest: false, hittestchildren: false });
        host.style.backgroundColor = '#000000';
        host.style.zIndex = 10000;
        var panel = $.CreatePanel('MapPlayerPreviewPanel', host, key + 'Model', {
            'require-composition-layer': 'true', 'transparent-background': 'false',
            'disable-depth-of-field': 'true', 'pin-fov': 'vertical',
            camera: 'cam_char_inspect_wide', player: 'true',
            playername: 'vanity_character', animgraphcharactermode: 'inventory-inspect',
            animgraphturns: 'false', map: 'ui/acknowledge_item',
            initial_entity: 'item', mouse_rotate: 'false', panzoom_enabled: 'false',
            rotation_limit_x: '360', rotation_limit_y: '90',
            auto_rotate_x: '0', auto_rotate_y: '0', auto_recenter: false
        });
        panel.style.width = '100%'; panel.style.height = '100%';
        var dbg = $.CreatePanel('Label', host, key + 'Dbg', { hittest: false });
        dbg.style.x = '4px'; dbg.style.y = '2px'; dbg.style.color = '#ff5555';
        dbg.style.fontSize = '10px'; dbg.style.zIndex = 10002;
        var espHost = $.CreatePanel('Panel', host, key + 'Esp', { hittest: false, hittestchildren: false });
        espHost.style.width = '100%'; espHost.style.height = '100%'; espHost.style.zIndex = 10001;
        var marker = $.CreatePanel('Panel', host, key + 'Marker', { hittest: false });
        marker.style.position = '0px 0px 0px';
        marker.style.horizontalAlign = 'left'; marker.style.verticalAlign = 'top';
        marker.style.backgroundColor = '#ff00ff'; marker.style.opacity = '1.0';
        marker.style.zIndex = 10003;
        state = { host: host, panel: panel, dbg: dbg, marker: marker, espNodes: {}, espConfig: { enabled: false },
                  width: a.w, height: a.h, marginX: 8, marginY: 14, heartbeat: Date.now(), dead: false, model: '', bones: 0 };
        data[key] = state;
        state.node = function (id, label) {
            var n = state.espNodes[id];
            if (!n) { n = $.CreatePanel(label ? 'Label' : 'Panel', espHost, id, { hittest: false }); state.espNodes[id] = n; }
            n.style.visibility = 'visible';
            return n;
        };
        state.rect = function (id, x, y, w, h, color) {
            if (!isFinite(x + y + w + h) || w <= 0 || h <= 0) return null;
            var n = state.node(id, false);
            n.style.x = x + 'px'; n.style.y = y + 'px';
            n.style.width = w + 'px'; n.style.height = h + 'px';
            n.style.backgroundColor = color; n.style.transform = 'none';
            return n;
        };
        state.line = function (id, p, q, color, thick) {
            var dx = q.x - p.x, dy = q.y - p.y, len = Math.sqrt(dx * dx + dy * dy);
            var n = state.rect(id, p.x, p.y - thick * 0.5, len, thick, color);
            if (n) { n.style.transformOrigin = '0% 50%'; n.style.transform = 'rotateZ(' + Math.atan2(dy, dx) * 180 / Math.PI + 'deg)'; }
        };
        state.text = function (id, x, y, value, color) {
            var n = state.node(id, true);
            n.text = value;
            n.style.x = x + 'px'; n.style.y = y + 'px'; n.style.color = color;
            n.style.fontSize = '11px'; n.style.fontWeight = 'bold';
        };
        state.drawEsp = function () {
            if (state.dead || !panel.IsValid()) return;
            for (var id in state.espNodes) state.espNodes[id].style.visibility = 'collapse';
            var cfg = state.espConfig;
            if (!cfg.enabled || typeof panel.GetBonePositionInPanelSpace !== 'function') return;
            panel.SetActiveCharacter(5);
            var names = ['pelvis', 'spine_0', 'spine_1', 'spine_2', 'spine_3', 'neck_0', 'head_0',
                         'arm_upper_L', 'arm_lower_L', 'hand_L', 'arm_upper_R', 'arm_lower_R', 'hand_R',
                         'leg_upper_L', 'leg_lower_L', 'ankle_L', 'leg_upper_R', 'leg_lower_R', 'ankle_R'];
            var bones = {}, points = [];
            for (var i = 0; i < names.length; ++i) {
                var pt;
                try { pt = panel.GetBonePositionInPanelSpace(names[i]); } catch (e) { continue; }
                if (!pt || !isFinite(pt.x) || !isFinite(pt.y) || (pt.x === 0 && pt.y === 0)) continue;
                if (pt.x < 0 || pt.y < 0 || pt.x > state.width || pt.y > state.height) continue;
                bones[names[i]] = { x: pt.x, y: pt.y }; points.push(bones[names[i]]);
            }
            state.bones = points.length;
            if (!bones.head_0 || !bones.pelvis || (!bones.ankle_L && !bones.ankle_R)) return;
            var left = state.width, right = 0, top = state.height, bottom = 0;
            for (var i = 0; i < points.length; ++i) {
                left = Math.min(left, points[i].x); right = Math.max(right, points[i].x);
                top = Math.min(top, points[i].y); bottom = Math.max(bottom, points[i].y);
            }
            left = Math.max(state.marginX, left); right = Math.min(state.width - 2, right);
            top = Math.max(Math.max(14, state.marginY), top); bottom = Math.min(state.height - 32, bottom);
            if (right <= left || bottom <= top) return;
            // Celerity box: height from feet to head, width = height * 0.5,
            // horizontally centred on the model.
            var boxH = bottom - top;
            var boxW = boxH * 0.5;
            var cx = (left + right) * 0.5;
            var x1 = Math.max(2, cx - boxW * 0.5), x2 = Math.min(state.width - 2, cx + boxW * 0.5);
            var y1 = top, y2 = bottom;
            var w = x2 - x1, h = y2 - y1;
            var box = cfg.box || {};
            if (box.on) {
                var frac = Math.max(0.005, Math.min(0.5, box.corner || 0.25));
                if (frac >= 0.5) {
                    state.line('boxT', { x: x1 - 1, y: y1 - 1 }, { x: x2 + 1, y: y1 - 1 }, 'rgba(0,0,0,0.7)', 1);
                    state.line('boxB', { x: x1 - 1, y: y2 + 1 }, { x: x2 + 1, y: y2 + 1 }, 'rgba(0,0,0,0.7)', 1);
                    state.line('boxL', { x: x1 - 1, y: y1 }, { x: x1 - 1, y: y2 }, 'rgba(0,0,0,0.7)', 1);
                    state.line('boxR', { x: x2 + 1, y: y1 }, { x: x2 + 1, y: y2 }, 'rgba(0,0,0,0.7)', 1);
                    state.line('boxT2', { x: x1, y: y1 }, { x: x2, y: y1 }, box.color, 1);
                    state.line('boxB2', { x: x1, y: y2 }, { x: x2, y: y2 }, box.color, 1);
                    state.line('boxL2', { x: x1, y: y1 }, { x: x1, y: y2 }, box.color, 1);
                    state.line('boxR2', { x: x2, y: y1 }, { x: x2, y: y2 }, box.color, 1);
                } else {
                    var len = Math.min(w, h) * frac;
                    var corners = [[x1, y1, 1, 1], [x2, y1, -1, 1], [x1, y2, 1, -1], [x2, y2, -1, -1]];
                    for (var i = 0; i < corners.length; ++i) {
                        var c = corners[i];
                        state.line('ch' + i, { x: c[0], y: c[1] }, { x: c[0] + c[2] * len, y: c[1] }, box.color, 1);
                        state.line('cv' + i, { x: c[0], y: c[1] }, { x: c[0], y: c[1] + c[3] * len }, box.color, 1);
                    }
                }
            }
            if ((cfg.skeleton || {}).on) {
                var sk = cfg.skeleton;
                var pairs = [['head_0', 'neck_0'], ['neck_0', 'spine_3'], ['spine_3', 'spine_2'],
                             ['spine_2', 'spine_1'], ['spine_1', 'spine_0'], ['spine_0', 'pelvis'],
                             ['spine_3', 'arm_upper_L'], ['arm_upper_L', 'arm_lower_L'], ['arm_lower_L', 'hand_L'],
                             ['spine_3', 'arm_upper_R'], ['arm_upper_R', 'arm_lower_R'], ['arm_lower_R', 'hand_R'],
                             ['pelvis', 'leg_upper_L'], ['leg_upper_L', 'leg_lower_L'], ['leg_lower_L', 'ankle_L'],
                             ['pelvis', 'leg_upper_R'], ['leg_upper_R', 'leg_lower_R'], ['leg_lower_R', 'ankle_R']];
                for (var i = 0; i < pairs.length; ++i) {
                    var p = bones[pairs[i][0]], q = bones[pairs[i][1]];
                    if (p && q) state.line('bone' + i, p, q, sk.color, Math.max(0.6, sk.thickness));
                }
            }
            var hp = cfg.health || {};
            if (hp.on) {
                var fracH = Math.max(0, Math.min(1, hp.frac || 0));
                var barX = x1 - 5, fillH = h * fracH;
                state.rect('hpOut', barX - 1, y1 - 1, 4, h + 2, 'rgba(0,0,0,0.7)');
                state.rect('hpBg', barX, y1, 2, h, 'rgba(0,0,0,0.8)');
                if (fillH > 0) state.rect('hpFill', barX, y2 - fillH, 2, fillH, hp.color);
                if (hp.text && hp.value < 100)
                    state.text('hpValue', barX - 3, y2 - fillH - 7, '' + hp.value, hp.textColor);
            }
            if ((cfg.name || {}).on && cfg.name.text)
                state.text('name', x1 + (w - 46) * 0.5, y1 - 14, cfg.name.text, cfg.name.color);
            var weapon = cfg.weapon || {};
            var ammo = cfg.ammo || {};
            var below = y2 + 2;
            if (ammo.on) {
                var fracA = Math.max(0, Math.min(1, ammo.frac || 0));
                state.rect('ammoOut', x1 - 1, below - 1, w + 2, 4, 'rgba(0,0,0,0.7)');
                state.rect('ammoBg', x1, below, w, 2, 'rgba(0,0,0,0.8)');
                if (fracA > 0) state.rect('ammoFill', x1, below, w * fracA, 2, ammo.color);
                below += 4;
                if (ammo.text) state.text('ammoText', x1 + 4, below, ammo.text, ammo.textColor);
                below += ammo.text ? 11 : 0;
            }
            if (weapon.on && weapon.text)
                state.text('weapon', x1 + (w - 40) * 0.5, below, weapon.text, weapon.color);
            var flags = cfg.flags || [], fy = y1;
            for (var i = 0; i < flags.length; ++i) {
                if (!flags[i].on) continue;
                state.text('flag' + i, x2 + 3, fy, flags[i].text, flags[i].color);
                fy += 7;
            }
        };
        state.frameHandler = $.RegisterForUnhandledEvent('UISceneFrameBoundary', state.drawEsp);
        var expire = function () {
            if (state.dead) return;
            if (Date.now() - state.heartbeat > 1000) { state.dispose(); return; }
            $.Schedule(0.25, expire);
        };
        $.Schedule(0.25, expire);
        state.dispose = function () {
            if (state.dead) return;
            state.dead = true;
            $.UnregisterForUnhandledEvent('UISceneFrameBoundary', state.frameHandler);
            if (host.IsValid()) host.DeleteAsync(0);
            if (data[key] === state) delete data[key];
        };
    }
    state.heartbeat = Date.now();
    var panel = state.panel;
    if (!panel || !panel.IsValid()) {
        state.dispose();
        return;
    }
    var sx = root.actuallayoutwidth / a.sw, sy = root.actuallayoutheight / a.sh;
    if (!isFinite(sx) || sx <= 0) sx = 1;
    if (!isFinite(sy) || sy <= 0) sy = 1;
    state.host.style.x = (a.x * sx) + 'px'; state.host.style.y = (a.y * sy) + 'px';
    state.host.style.width = (a.w * sx) + 'px'; state.host.style.height = (a.h * sy) + 'px';
    state.host.style.visibility = 'visible';
    state.width = a.w * sx; state.height = a.h * sy;
    state.marginX = a.m * sx + 2; state.marginY = a.m * sy + 2;
    state.marker.style.width = (a.m * sx) + 'px'; state.marker.style.height = (a.m * sy) + 'px';
    state.espConfig = a.esp;
    if (state.model !== a.model) {
        panel.SetActiveCharacter(5);
        panel.SetPlayerModel(a.model);
        state.model = a.model;
    }
    panel.TransitionToCamera('cam_char_inspect_wide', 0);
    state.drawEsp();
    state.dbg.text = 'P:' + (panel.IsValid() ? 1 : 0) + ' M:' + (state.model ? 1 : 0) +
                     ' B:' + state.bones + ' ' + Math.round(state.width) + 'x' + Math.round(state.height);
})(__ARGS__);
"#;

/// One flag line for the preview ESP (already formatted, e.g. "37m").
#[derive(Clone, Debug, Default)]
pub struct PreviewFlag {
    pub on: bool,
    pub text: String,
    pub color: [u8; 4],
}

/// Everything the Panorama preview script needs to draw the exact same ESP the
/// world overlay draws (Celerity geometry, real target values).
#[derive(Clone, Debug, Default)]
pub struct PreviewConfig {
    pub enabled: bool,
    pub enemy: bool,
    pub box_on: bool,
    pub corner_fraction: f32,
    pub box_color: [u8; 4],
    pub skeleton_on: bool,
    pub skeleton_thickness: f32,
    pub skeleton_color: [u8; 4],
    pub name_on: bool,
    pub name_text: String,
    pub name_color: [u8; 4],
    pub health_on: bool,
    pub health_value: i32,
    pub health_fraction: f32,
    pub health_text: bool,
    pub health_low_color: [u8; 4],
    pub health_high_color: [u8; 4],
    pub ammo_on: bool,
    pub ammo_fraction: f32,
    pub ammo_text: String,
    pub ammo_text_on: bool,
    pub ammo_low_color: [u8; 4],
    pub ammo_high_color: [u8; 4],
    pub weapon_on: bool,
    pub weapon_text: String,
    pub weapon_color: [u8; 4],
    pub flags: Vec<PreviewFlag>,
}

struct Request {
    active: bool,
    rect: (f32, f32, f32, f32),
    screen: (f32, f32),
    cfg: PreviewConfig,
}

struct Driver {
    panorama: Option<Panorama>,
    /// Panorama is touched only while the optional engine model is requested.
    engine_active: bool,
    last_try: Instant,
    last_send: Instant,
    last_active: bool,
    last_mode: Option<PanelMode>,
    last_fail_log: Instant,
    pending: Option<Request>,
    ticks: u32,
    sent: u32,
}

static DRIVER: Mutex<Option<Driver>> = Mutex::new(None);
static LAST_RECT: Mutex<Option<(f32, f32, f32, f32)>> = Mutex::new(None);

fn lock() -> std::sync::MutexGuard<'static, Option<Driver>> {
    DRIVER.lock().unwrap_or_else(|e| e.into_inner())
}

fn preferred_mode() -> PanelMode {
    if crate::menu::is_menu_open() {
        PanelMode::MainMenu
    } else {
        PanelMode::Hud
    }
}

pub fn init() {
    let mut guard = lock();
    if guard.is_some() {
        return;
    }
    // Own rendering is the default. Panorama is attached lazily only when the
    // engine model is selected, so a normal match never calls into UI code.
    *guard = Some(Driver {
        panorama: None,
        engine_active: false,
        last_try: Instant::now() - Duration::from_secs(1),
        last_send: Instant::now() - UPDATE_INTERVAL,
        last_active: false,
        last_mode: None,
        last_fail_log: Instant::now() - Duration::from_secs(1),
        pending: None,
        ticks: 0,
        sent: 0,
    });
}

/// Сколько кадров подряд без успешного захвата терпим, прежде чем показать fallback.
const CAPTURE_GRACE_FRAMES: u64 = 30;
static CAPTURE_FRAME: AtomicU64 = AtomicU64::new(0);
static CAPTURE_LAST_OK: AtomicU64 = AtomicU64::new(0);

/// Вызывается из Present-хука после capture_preview.
pub fn report_capture(ok: bool) {
    let frame = CAPTURE_FRAME.fetch_add(1, Ordering::Relaxed) + 1;
    if ok {
        CAPTURE_LAST_OK.store(frame, Ordering::Relaxed);
    }
}

/// true только если Panorama жива И кадр модели реально захватывается.
/// Иначе меню рисует процедурный fallback (раньше тут было просто «attach удался»).
pub fn available() -> bool {
    let panorama_ok = lock()
        .as_ref()
        .filter(|d| d.engine_active)
        .and_then(|d| d.panorama.as_ref())
        .map(|p| p.is_ready())
        .unwrap_or(false);
    if !panorama_ok {
        return false;
    }
    let last_ok = CAPTURE_LAST_OK.load(Ordering::Relaxed);
    last_ok > 0 && CAPTURE_FRAME.load(Ordering::Relaxed).saturating_sub(last_ok) <= CAPTURE_GRACE_FRAMES
}

#[inline]
fn css(c: [u8; 4]) -> String {
    format!("rgba({},{},{},{})", c[0], c[1], c[2], c[3] as f32 / 255.0)
}

fn build_script(active: bool, rect: (f32, f32, f32, f32), screen: (f32, f32), cfg: PreviewConfig) -> String {
    let (x, y, w, h) = rect;
    let model = if cfg.enemy {
        "agents/models/tm_phoenix/tm_phoenix.vmdl"
    } else {
        "agents/models/ctm_sas/ctm_sas.vmdl"
    };
    let box_color = cfg.box_color;
    let skel_color = cfg.skeleton_color;
    let name_color = cfg.name_color;
    let health_color = lerp_preview_color(cfg.health_low_color, cfg.health_high_color, cfg.health_fraction);
    let ammo_color = lerp_preview_color(cfg.ammo_low_color, cfg.ammo_high_color, cfg.ammo_fraction);
    let flags: Vec<serde_json::Value> = cfg
        .flags
        .iter()
        .map(|flag| {
            serde_json::json!({ "on": flag.on, "text": flag.text, "color": css(flag.color) })
        })
        .collect();
    let args = serde_json::json!({
        "active": active,
        "agent": true,
        "x": x,
        "y": y,
        "w": w,
        "h": h,
        "m": crate::dx11::MARKER_PX,
        "sw": screen.0,
        "sh": screen.1,
        "model": model,
        "def": 5,
        "esp": {
            "enabled": cfg.enabled,
            "box": { "on": cfg.box_on, "corner": cfg.corner_fraction, "color": css(box_color) },
            "skeleton": { "on": cfg.skeleton_on, "thickness": cfg.skeleton_thickness, "color": css(skel_color) },
            "name": { "on": cfg.name_on, "text": cfg.name_text, "color": css(name_color) },
            "health": {
                "on": cfg.health_on,
                "value": cfg.health_value,
                "frac": cfg.health_fraction,
                "text": cfg.health_text,
                "color": css(health_color),
                "textColor": "rgba(255,255,255,1)"
            },
            "ammo": {
                "on": cfg.ammo_on,
                "frac": cfg.ammo_fraction,
                "text": cfg.ammo_text,
                "textColor": "rgba(255,255,255,1)",
                "color": css(ammo_color)
            },
            "weapon": { "on": cfg.weapon_on, "text": cfg.weapon_text, "color": css(cfg.weapon_color) },
            "flags": flags
        }
    });
    SCRIPT.replace("__ARGS__", &args.to_string())
}

/// Same linear interpolation the overlay uses for gradient health/ammo bars.
fn lerp_preview_color(a: [u8; 4], b: [u8; 4], t: f32) -> [u8; 4] {
    let t = t.clamp(0.0, 1.0);
    [
        (a[0] as f32 + (b[0] as f32 - a[0] as f32) * t) as u8,
        (a[1] as f32 + (b[1] as f32 - a[1] as f32) * t) as u8,
        (a[2] as f32 + (b[2] as f32 - a[2] as f32) * t) as u8,
        (a[3] as f32 + (b[3] as f32 - a[3] as f32) * t) as u8,
    ]
}

/// Меню только сохраняет запрос; отправка идёт из FrameStageNotify.
pub fn last_rect() -> Option<(f32, f32, f32, f32)> {
    *LAST_RECT.lock().unwrap_or_else(|e| e.into_inner())
}

pub fn set_request(active: bool, rect: (f32, f32, f32, f32), screen: (f32, f32), cfg: PreviewConfig) {
    *LAST_RECT.lock().unwrap_or_else(|e| e.into_inner()) = if active { Some(rect) } else { None };
    let mut guard = lock();
    if let Some(driver) = guard.as_mut() {
        driver.engine_active = active;
        // The JS panel expires its own state after a missed heartbeat.
        driver.pending = if active { Some(Request { active, rect, screen, cfg }) } else { None };
    }
}

/// Tick only while the optional engine model is active. Own rendering needs no
/// Panorama work in either the menu or a live match.
pub fn wants_menu_tick() -> bool {
    lock().as_ref().map(|d| d.engine_active).unwrap_or(false)
}

fn log_fail(driver: &mut Driver, reason: &str) {
    if driver.last_fail_log.elapsed() >= Duration::from_secs(1) {
        driver.last_fail_log = Instant::now();
        crate::trace(&format!("Panorama bridge: {reason}"));
    }
}

/// Вызывается только из UI-потока в FrameStageNotify (матч: stage 6, меню: любой stage).
pub fn tick() {
    let mut guard = lock();
    let Some(driver) = guard.as_mut() else { return };
    if !driver.engine_active {
        return;
    }
    driver.ticks = driver.ticks.wrapping_add(1);
    let now = Instant::now();
    let preferred = preferred_mode();
    if driver.panorama.is_none() && now.duration_since(driver.last_try) > Duration::from_secs(1) {
        driver.last_try = now;
        match Panorama::attach(preferred) {
            Ok(panorama) => {
                crate::trace(&format!(
                    "Panorama bridge: ok mode={} panel=0x{:X}",
                    panorama.mode().as_str(),
                    panorama.panel()
                ));
                driver.panorama = Some(panorama);
            }
            Err(reason) => log_fail(driver, &format!("attach failed: {reason}")),
        }
    }
    if let Some(panorama) = driver.panorama.as_mut() {
        panorama.set_preferred(preferred);
    }
    let Some(req) = driver.pending.take() else { return };
    let force = driver.last_active != req.active;
    if !force && now.duration_since(driver.last_send) < UPDATE_INTERVAL {
        return;
    }
    if !driver.panorama.as_ref().map(|p| p.is_ready()).unwrap_or(false) {
        log_fail(driver, "script dropped: panel not live");
        return;
    }
    let live = driver.panorama.as_ref().and_then(|p| p.live_root());
    let (dispose_error, result) = {
        let Some(panorama) = driver.panorama.as_mut() else {
            log_fail(driver, "script dropped: no bridge");
            return;
        };
        // Смена HUD <-> MainMenu: старый JS-state должен исчезнуть до создания нового.
        let mut dispose_error = None;
        if let Some((mode, panel)) = live {
            if driver.last_mode != Some(mode) {
                crate::trace(&format!(
                    "Panorama bridge: ok mode={} panel=0x{:X}",
                    mode.as_str(),
                    panel
                ));
                let dispose = build_script(false, req.rect, req.screen, req.cfg.clone());
                if let Err(reason) = panorama.run_script(&dispose) {
                    dispose_error = Some(reason.to_string());
                }
            }
        }
        let script = build_script(req.active, req.rect, req.screen, req.cfg);
        (dispose_error, panorama.run_script(&script))
    };
    if let Some(reason) = dispose_error {
        log_fail(driver, &reason);
    }
    match result {
        Ok(result) => {
            driver.last_mode = Some(result.mode);
            driver.last_send = now;
            driver.last_active = req.active;
            driver.sent += 1;
        }
        Err(reason) => log_fail(driver, reason),
    }
}
