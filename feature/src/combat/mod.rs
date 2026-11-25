// Config block: весь combat — чистые функции над Snapshot; запись только через MutationGuard в silent.rs.
pub mod backtrack;
pub mod cmd;
pub mod extrapolation;
pub mod hide_shots;
pub mod hitchance;
pub mod hitbox;
pub mod mindamage;
pub mod multipoint;
pub mod silent;
pub mod target;

use parking_lot::Mutex;
use wallbang_core::math::{QAngle, Vec3};
use wallbang_core::state::{GameState, PlayerSnapshot};

use backtrack::History;

static HISTORY: Mutex<History> = Mutex::new(History::new());

#[derive(Clone, Copy, Debug)]
pub struct Target {
    pub index: usize,
    pub address: usize,
    pub origin: Vec3,
    pub velocity: Vec3,
    pub health: i32,
    pub score: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct Decision {
    pub fire: bool,
    pub target: Option<Target>,
    pub point: Vec3,
    pub angles: QAngle,
    pub hitchance: f32,
    pub damage: f32,
}

impl Default for Decision {
    fn default() -> Self {
        Self {
            fire: false,
            target: None,
            point: Vec3::default(),
            angles: QAngle::default(),
            hitchance: 0.0,
            damage: 0.0,
        }
    }
}

pub fn local_view_angles() -> Option<QAngle> {
    let client = wallbang_core::modules::module_base("client.dll")?;
    let raw = unsafe {
        wallbang_core::memory::read::<[f32; 3]>(client + wallbang_config::offsets::DW_VIEW_ANGLES)?
    };
    Some(QAngle::new(raw[0], raw[1], raw[2]))
}

pub fn run(state: &GameState, tick: u32) -> Decision {
    let mut out = Decision::default();
    let snapshot = state.snapshot();
    if !snapshot.valid {
        return out;
    }
    let settings = state.settings();
    let combat = &settings.combat;
    if !combat.ragebot.enabled && !combat.legit.enabled {
        return out;
    }
    let (
        max_fov,
        hitchance_target,
        min_damage,
        ticks,
        use_multipoints,
        backtrack,
        backtrack_ticks,
        hitboxes,
    ) = if combat.ragebot.enabled {
        let group = combat.ragebot.group();
        (
            group.max_fov,
            if group.hitchance_override {
                group.hitchance_override_value
            } else {
                group.hitchance
            },
            if group.min_damage_override {
                group.min_damage_override_value
            } else {
                group.min_damage
            },
            group.extrapolation_ticks.max(0) as usize,
            group.pointscale > 0.0 || group.hitboxes.iter().filter(|b| **b).count() > 1,
            group.backtrack,
            group.backtrack_ticks.max(1) as usize,
            group.hitboxes,
        )
    } else {
        let legit = combat.legit.group();
        (
            legit.fov,
            60,
            1,
            0,
            false,
            false,
            1,
            [true, true, false, false, false, false],
        )
    };
    {
        let mut history = HISTORY.lock();
        for player in snapshot.players.iter().filter(|p| p.alive && p.enemy) {
            history.record(player.index, tick, player.origin, player.velocity);
        }
    }
    let local_angles = local_view_angles().unwrap_or_default();
    let eye = wallbang_core::usercmd::UserCmd::local_eye_position()
        .unwrap_or(snapshot.local_origin);
    let enemies: Vec<&PlayerSnapshot> = snapshot
        .players
        .iter()
        .filter(|p| p.alive && p.enemy)
        .collect();
    let Some(best) = target::select(&enemies, eye, local_angles, max_fov) else {
        return out;
    };
    out.target = Some(best);
    let (base, velocity) = if backtrack {
        HISTORY
            .lock()
            .select_tick(best.index, tick, backtrack_ticks)
            .map(|record| (record.origin, record.velocity))
            .unwrap_or((best.origin, best.velocity))
    } else {
        (best.origin, best.velocity)
    };
    let predicted = extrapolation::extrapolate(base, velocity, ticks, 1.0 / 64.0);
    let points = hitbox::extract(best.address, &hitboxes);
    let radius = if use_multipoints { 6.0 } else { 2.0 };
    let candidates = if points.is_empty() {
        multipoint::build(predicted, radius, 1.0 - hitchance_target as f32 / 200.0)
    } else {
        multipoint::build(points[0], radius, 1.0 - hitchance_target as f32 / 200.0)
    };
    out.point = candidates.first().copied().unwrap_or(predicted);
    out.angles = angle_to_point(eye, out.point);
    out.hitchance = hitchance::roll(hitchance_target as f32, 1.0, 0x9E3779B9, candidates.len());
    out.damage = mindamage::damage(100.0, 4.0, 0.5, best.origin.distance_to(eye));
    out.fire = out.hitchance >= hitchance_target as f32 && out.damage >= min_damage as f32;
    out
}

fn angle_to_point(from: Vec3, to: Vec3) -> QAngle {
    wallbang_core::math::angle_to(from, to)
}
