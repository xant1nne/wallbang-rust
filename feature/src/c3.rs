// Scene controls backed by confirmed engine cvars.
//
// Nothing here invents a backend: every toggle maps to a cvar that was found
// offline in this build's engine2.dll/client.dll. All writes go through the
// guarded cvar layer, which restores the original bits when the toggle is off.
use parking_lot::Mutex;
use wallbang_config::settings::AmbienceSettings;
use wallbang_core::mutation;
use wallbang_core::state::GameState;

use crate::cvar;

/// Applying cvars is O(number of cvars) on a cache miss; the menu may sit on
/// the same values for minutes. Sync only when a value or the write gate
/// actually changes.
static LAST_SYNC: Mutex<Option<(bool, AmbienceSettings)>> = Mutex::new(None);
static LAST_ATTEMPT: Mutex<Option<std::time::Instant>> = Mutex::new(None);
const SYNC_INTERVAL: std::time::Duration = std::time::Duration::from_millis(250);

fn needs_sync(
    last: &Option<(bool, AmbienceSettings)>,
    allowed: bool,
    ambience: &AmbienceSettings,
) -> bool {
    last.as_ref()
        .map_or(true, |(was_allowed, previous)| {
            *was_allowed != allowed || previous != ambience
        })
}

fn safe(value: f32, min: f32, max: f32) -> f64 {
    if !value.is_finite() {
        return f64::from(min);
    }
    f64::from(value.clamp(min, max))
}

fn group(enabled: bool, entries: &[(&'static str, &str, f64)], applied: &mut usize, failed: &mut Vec<&'static str>) {
    for &(key, name, value) in entries {
        if enabled {
            if cvar::apply(key, name, value) {
                *applied += 1;
            } else {
                cvar::restore(key);
                failed.push(key);
            }
        } else {
            cvar::restore(key);
        }
    }
}

/// Applies the current scene settings. Called once per rendered frame after
/// the write gate has been updated from `Misc -> allow writes`.
/// Returns a diagnostic line only when a sync actually runs.
pub fn sync(state: &GameState) -> Option<String> {
    let now = std::time::Instant::now();
    {
        let mut previous = LAST_ATTEMPT.lock();
        if previous.is_some_and(|last| now.saturating_duration_since(last) < SYNC_INTERVAL) { return None; }
        *previous = Some(now);
    }
    if !cvar::available() {
        return None;
    }
    let settings = state.settings();
    let ambience = &settings.visual.ambience;
    let allowed = mutation::is_allowed();
    let mut last = LAST_SYNC.lock();
    if !needs_sync(&last, allowed, ambience) {
        return None;
    }
    if !allowed {
        *last = Some((false, ambience.clone()));
        return None;
    }
    let mut applied = 0usize;
    let mut failed = Vec::new();
    let intensity = safe(ambience.lighting_intensity, 0.0, 4.0);
    group(
        false, // Scene-light hook owns lighting; release legacy boolean cvars.
        &[
            ("c3_lighting_direct", "r_directlighting", intensity),
            ("c3_lighting_indirect", "r_indirectlighting", intensity),
            ("c3_lighting_sun", "r_rendersun", intensity),
        ],
        &mut applied,
        &mut failed,
    );

    let fog_density = safe(ambience.fog_density, 0.0, 1.0);
    let fog_end = safe(ambience.fog_end, 0.0, 20_000.0);
    group(
        false, // SetupFog writes actual shader parameters for this build.
        &[
            ("c3_fog_enable", "fog_override_enable", 1.0),
            ("c3_fog_density", "fog_override_max_density", fog_density),
            ("c3_fog_end", "fog_override_end", fog_end),
        ],
        &mut applied,
        &mut failed,
    );

    // Our world-space renderer owns weather; release the former engine backend
    // to avoid drawing rain twice and to keep decorative particles read-only.
    cvar::restore("c3_weather_rain");

    group(
        ambience.bloom,
        &[(
            "c3_post_bloom",
            "r_csgo_render_post_bloom_strength",
            safe(ambience.bloom_strength, 0.0, 4.0),
        )],
        &mut applied,
        &mut failed,
    );
    group(
        ambience.gamma,
        &[(
            "c3_post_gamma",
            "r_fullscreen_gamma",
            safe(ambience.gamma_value, 0.5, 4.0),
        )],
        &mut applied,
        &mut failed,
    );
    group(
        ambience.dof,
        &[
            ("c3_post_dof", "dof_enabled", 1.0),
            (
                "c3_post_dof_near",
                "dof_near_blurry",
                safe(ambience.dof_near_blurry, 0.0, 2_000.0),
            ),
            (
                "c3_post_dof_far",
                "dof_far_blurry",
                safe(ambience.dof_far_blurry, 0.0, 2_000.0),
            ),
        ],
        &mut applied,
        &mut failed,
    );
    group(
        ambience.smoke_removal,
        &[
            ("c3_smoke_fullres", "r_csgo_smoke_fullres_pass", 0.0),
            ("c3_smoke_shadow", "r_csgo_smoke_shadow", 0.0),
            ("c3_smoke_edge", "cl_smoke_edge_feather", 0.0),
        ],
        &mut applied,
        &mut failed,
    );
    *last = Some((allowed, ambience.clone()));
    Some(format!(
        "c3 sync: allowed={} applied={} failed={:?} overrides={}",
        allowed,
        applied,
        failed,
        cvar::active_overrides()
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sync_is_skipped_until_a_value_or_the_write_gate_changes() {
        let mut ambience = AmbienceSettings::default();
        let mut last = None;
        assert!(needs_sync(&last, false, &ambience));
        last = Some((false, ambience.clone()));
        assert!(!needs_sync(&last, false, &ambience));
        assert!(needs_sync(&last, true, &ambience));
        ambience.bloom = true;
        assert!(needs_sync(&last, false, &ambience));
    }
}
