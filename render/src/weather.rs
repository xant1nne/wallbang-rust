// Config block: decorative, read-only world-space particles. No game offsets.
// This pass has no depth buffer sampling: particles do not collide with roofs.
use crate::DrawList;
use parking_lot::Mutex;
use std::sync::OnceLock;
use std::time::Instant;
use wallbang_config::settings::{AmbienceSettings, Rgba};
use wallbang_core::math::{w2s, Vec3};
use wallbang_core::state::GameState;

const MAX_WEATHER: usize = 768;
const MAX_AMBIENT: usize = 192;
const RADIUS: f32 = 650.0;
const HALF_HEIGHT: f32 = 380.0;
const MAX_DT: f32 = 0.05;
const FADE_SECONDS: f32 = 0.6;
const DRAW_MARGIN: f32 = 32.0;
const MAX_SEGMENT_PIXELS: f32 = 90.0;
const TELEPORT_DISTANCE: f32 = 1500.0;
const DENSITY_LIMIT: f32 = 10_000.0;
const INITIAL_SEED: u32 = 0x71AC_902D;

#[derive(Clone, Copy)]
struct Particle {
    position: Vec3,
    phase: f32,
    size: f32,
}

struct Field {
    particles: Vec<Particle>,
    seed: u32,
    kind: usize,
    anchor: Vec3,
    time: f32,
    opacity: f32,
}

impl Field {
    fn new(seed: u32) -> Self {
        Self {
            particles: Vec::new(),
            seed,
            kind: usize::MAX,
            anchor: Vec3::default(),
            time: 0.0,
            opacity: 0.0,
        }
    }

    fn random(&mut self) -> f32 {
        self.seed ^= self.seed << 13;
        self.seed ^= self.seed >> 17;
        self.seed ^= self.seed << 5;
        (self.seed >> 8) as f32 / 16_777_216.0
    }

    fn reset(&mut self) {
        self.particles.clear();
        self.opacity = 0.0;
    }

    fn advance(&mut self, anchor: Vec3, kind: usize, count: usize, dt: f32, speed: f32, wind: f32) {
        let delta = anchor.sub(self.anchor);
        if self.kind != kind || delta.length() > TELEPORT_DISTANCE {
            self.reset();
            self.kind = kind;
        }
        self.anchor = anchor;
        self.time = (self.time + dt).rem_euclid(4096.0);
        self.opacity = (self.opacity + dt / FADE_SECONDS).min(1.0);
        self.particles.truncate(count);
        while self.particles.len() < count {
            let position = anchor.add(Vec3::new(
                (self.random() * 2.0 - 1.0) * RADIUS,
                (self.random() * 2.0 - 1.0) * RADIUS,
                (self.random() * 2.0 - 1.0) * HALF_HEIGHT,
            ));
            let phase = self.random() * std::f32::consts::TAU;
            let size = 0.6 + self.random() * 1.4;
            self.particles.push(Particle {
                position,
                phase,
                size,
            });
        }
        for p in &mut self.particles {
            let drift = (self.time * 0.8 + p.phase).sin();
            let velocity = match kind {
                0 => Vec3::new(wind * 80.0 + drift * 18.0, drift * 12.0, -38.0 * p.size),
                1 => Vec3::new(wind * 160.0, wind * 35.0, -650.0 * p.size),
                2 => Vec3::new(wind * 8.0, 2.0, -2.0),
                3 => Vec3::new(
                    drift * 18.0 + wind * 14.0,
                    (self.time + p.phase).cos() * 18.0,
                    drift * 8.0,
                ),
                _ => Vec3::new(wind * 50.0 + drift * 16.0, drift * 10.0, 45.0 * p.size),
            };
            p.position = p.position.add(velocity.scale(dt * speed));
            p.position.x = wrap(p.position.x, anchor.x, RADIUS);
            p.position.y = wrap(p.position.y, anchor.y, RADIUS);
            p.position.z = wrap(p.position.z, anchor.z, HALF_HEIGHT);
        }
    }

    fn render(&self, dl: &mut DrawList, matrix: &[f32; 16], color: Rgba) {
        let (width, height) = dl.size();
        for p in &self.particles {
            let Some(screen) = w2s(matrix, p.position, width, height) else {
                continue;
            };
            if screen.x < -DRAW_MARGIN
                || screen.y < -DRAW_MARGIN
                || screen.x > width + DRAW_MARGIN
                || screen.y > height + DRAW_MARGIN
            {
                continue;
            }
            let distance = p.position.distance_to(self.anchor);
            let edge = ((RADIUS - distance) / 140.0).clamp(0.0, 1.0);
            let shimmer = if self.kind >= 2 {
                0.35 + 0.65 * (self.time * 1.5 + p.phase).sin().powi(2)
            } else {
                1.0
            };
            let opacity = self.opacity * edge * shimmer;
            let tint = alpha(color, opacity);
            if tint[3] == 0 {
                continue;
            }
            let radius = (p.size * 110.0 / distance.max(60.0)).clamp(0.65, 2.5);
            if self.kind == 1 {
                let tail = p.position.add(Vec3::new(0.0, 0.0, 14.0 * p.size));
                if let Some(end) = w2s(matrix, tail, width, height) {
                    let dx = end.x - screen.x;
                    let dy = end.y - screen.y;
                    if dx * dx + dy * dy <= MAX_SEGMENT_PIXELS * MAX_SEGMENT_PIXELS {
                        dl.line(screen.x, screen.y, end.x, end.y, 1.0, tint);
                    }
                }
            } else {
                // Layered soft halo with a small bright core; no textures or GPU readback.
                dl.circle_filled(screen.x, screen.y, radius * 2.5, alpha(tint, 0.08));
                dl.circle_filled(screen.x, screen.y, radius * 1.5, alpha(tint, 0.2));
                dl.circle_filled(screen.x, screen.y, radius, tint);
                if self.kind == 2 {
                    dl.line(
                        screen.x - radius * 3.0,
                        screen.y,
                        screen.x + radius * 3.0,
                        screen.y,
                        0.6,
                        alpha(tint, 0.35),
                    );
                    dl.line(
                        screen.x,
                        screen.y - radius * 3.0,
                        screen.x,
                        screen.y + radius * 3.0,
                        0.6,
                        alpha(tint, 0.35),
                    );
                }
            }
        }
    }
}

fn alpha(mut color: Rgba, amount: f32) -> Rgba {
    color[3] = (color[3] as f32 * amount.clamp(0.0, 1.0)).round() as u8;
    color
}
fn wrap(value: f32, center: f32, half: f32) -> f32 {
    center + (value - center + half).rem_euclid(half * 2.0) - half
}
fn finite(value: f32, fallback: f32, min: f32, max: f32) -> f32 {
    if value.is_finite() {
        value.clamp(min, max)
    } else {
        fallback
    }
}

struct Atmosphere {
    weather: Field,
    ambient: Field,
    last: Instant,
}
impl Atmosphere {
    fn new() -> Self {
        Self {
            weather: Field::new(INITIAL_SEED),
            ambient: Field::new(INITIAL_SEED ^ 0xAC13_8437),
            last: Instant::now(),
        }
    }
    fn clear(&mut self) {
        self.weather.reset();
        self.ambient.reset();
        self.last = Instant::now();
    }
    fn frame(
        &mut self,
        dl: &mut DrawList,
        cfg: &AmbienceSettings,
        origin: Vec3,
        matrix: &[f32; 16],
    ) {
        let now = Instant::now();
        let dt = now.duration_since(self.last).as_secs_f32().min(MAX_DT);
        self.last = now;
        let speed = finite(cfg.weather_speed, 1.0, 0.1, 4.0);
        let wind = finite(cfg.weather_wind, 0.3, -2.0, 2.0);
        if cfg.weather_on {
            let count = (finite(cfg.weather_density, 1000.0, 0.0, DENSITY_LIMIT) / DENSITY_LIMIT
                * MAX_WEATHER as f32)
                .round() as usize;
            self.weather
                .advance(origin, cfg.weather.min(2), count, dt, speed, wind);
            self.weather.render(dl, matrix, cfg.weather_color);
        } else {
            self.weather.reset();
        }
        if cfg.ambient_particles {
            let count = finite(cfg.particle_density, 120.0, 0.0, MAX_AMBIENT as f32) as usize;
            self.ambient.advance(
                origin,
                3 + cfg.particle_style.min(1),
                count,
                dt,
                speed,
                wind,
            );
            self.ambient.render(dl, matrix, cfg.particle_color);
        } else {
            self.ambient.reset();
        }
    }
}

static ATMOSPHERE: OnceLock<Mutex<Atmosphere>> = OnceLock::new();

pub fn draw(dl: &mut DrawList, state: &GameState) {
    let cfg = state.settings().visual.ambience.clone();
    let active = cfg.weather_on || cfg.ambient_particles;
    if !active && ATMOSPHERE.get().is_none() {
        return;
    }
    let mut atmosphere = ATMOSPHERE
        .get_or_init(|| Mutex::new(Atmosphere::new()))
        .lock();
    let snapshot = state.snapshot();
    if !active
        || !snapshot.valid
        || !snapshot.local_origin.x.is_finite()
        || !snapshot.local_origin.y.is_finite()
        || !snapshot.local_origin.z.is_finite()
        || snapshot.view_matrix.iter().any(|x| !x.is_finite())
    {
        atmosphere.clear();
        return;
    }
    atmosphere.frame(dl, &cfg, snapshot.local_origin, &snapshot.view_matrix);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn field_stays_bounded_after_movement_and_long_simulation() {
        let mut field = Field::new(INITIAL_SEED);
        for tick in 0..3000 {
            let anchor = Vec3::new(tick as f32 * 0.5, -100.0, 60.0);
            field.advance(anchor, 1, MAX_WEATHER, 0.016, 4.0, -2.0);
            assert_eq!(field.particles.len(), MAX_WEATHER);
            for p in &field.particles {
                let d = p.position.sub(anchor);
                assert!(d.x.abs() <= RADIUS + 0.01 && d.y.abs() <= RADIUS + 0.01);
                assert!(d.z.abs() <= HALF_HEIGHT + 0.01);
            }
        }
        field.advance(Vec3::new(10000.0, 0.0, 0.0), 0, 12, 0.016, 1.0, 0.0);
        assert_eq!(field.particles.len(), 12);
        assert!(field.opacity < 0.1);
    }
}
