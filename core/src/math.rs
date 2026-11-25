// Config block: MATRIX_SIZE, W2S_MIN_W из wallbang_config::constants.
use wallbang_config::constants::{MATRIX_SIZE, W2S_MIN_W};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    #[inline]
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    #[inline]
    pub fn add(self, o: Self) -> Self {
        Self::new(self.x + o.x, self.y + o.y)
    }
    #[inline]
    pub fn sub(self, o: Self) -> Self {
        Self::new(self.x - o.x, self.y - o.y)
    }
    #[inline]
    pub fn scale(self, s: f32) -> Self {
        Self::new(self.x * s, self.y * s)
    }
    #[inline]
    pub fn length(self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(C)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    #[inline]
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
    #[inline]
    pub fn add(self, o: Self) -> Self {
        Self::new(self.x + o.x, self.y + o.y, self.z + o.z)
    }
    #[inline]
    pub fn sub(self, o: Self) -> Self {
        Self::new(self.x - o.x, self.y - o.y, self.z - o.z)
    }
    #[inline]
    pub fn scale(self, s: f32) -> Self {
        Self::new(self.x * s, self.y * s, self.z * s)
    }
    #[inline]
    pub fn dot(self, o: Self) -> f32 {
        self.x * o.x + self.y * o.y + self.z * o.z
    }
    #[inline]
    pub fn cross(self, o: Self) -> Self {
        Self::new(
            self.y * o.z - self.z * o.y,
            self.z * o.x - self.x * o.z,
            self.x * o.y - self.y * o.x,
        )
    }
    #[inline]
    pub fn length_sqr(self) -> f32 {
        self.dot(self)
    }
    #[inline]
    pub fn length(self) -> f32 {
        self.length_sqr().sqrt()
    }
    #[inline]
    pub fn distance_to(self, o: Self) -> f32 {
        self.sub(o).length()
    }
    #[inline]
    pub fn normalized(self) -> Self {
        let len = self.length();
        if len < 0.000001 {
            Self::new(0.0, 0.0, 0.0)
        } else {
            self.scale(1.0 / len)
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct QAngle {
    pub pitch: f32,
    pub yaw: f32,
    pub roll: f32,
}

impl QAngle {
    #[inline]
    pub const fn new(pitch: f32, yaw: f32, roll: f32) -> Self {
        Self { pitch, yaw, roll }
    }
    #[inline]
    pub fn normalized(self) -> Self {
        Self::new(
            normalize_angle(self.pitch),
            normalize_angle(self.yaw),
            normalize_angle(self.roll),
        )
    }
    #[inline]
    pub fn forward(self) -> Vec3 {
        let p = self.pitch.to_radians();
        let y = self.yaw.to_radians();
        let cp = p.cos();
        Vec3::new(y.cos() * cp, y.sin() * cp, -p.sin())
    }
}

#[inline]
pub fn normalize_angle(deg: f32) -> f32 {
    let mut a = deg % 360.0;
    if a > 180.0 {
        a -= 360.0;
    } else if a < -180.0 {
        a += 360.0;
    }
    a
}

#[inline]
pub fn angle_diff(to: f32, from: f32) -> f32 {
    normalize_angle(to - from)
}

#[inline]
pub fn angle_to(from: Vec3, to: Vec3) -> QAngle {
    let d = to.sub(from);
    let horiz = (d.x * d.x + d.y * d.y).sqrt();
    QAngle::new(
        (-d.z).atan2(horiz).to_degrees(),
        d.y.atan2(d.x).to_degrees(),
        0.0,
    )
}

#[inline]
pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Mat3x4 {
    pub rows: [[f32; 4]; 3],
}

impl Mat3x4 {
    #[inline]
    pub fn from_raw(v: &[f32; 12]) -> Self {
        Self {
            rows: [[v[0], v[1], v[2], v[3]], [v[4], v[5], v[6], v[7]], [v[8], v[9], v[10], v[11]]],
        }
    }
    #[inline]
    pub fn position(self) -> Vec3 {
        Vec3::new(self.rows[0][3], self.rows[1][3], self.rows[2][3])
    }
    #[inline]
    pub fn transform_point(self, p: Vec3) -> Vec3 {
        Vec3::new(
            self.rows[0][0] * p.x + self.rows[0][1] * p.y + self.rows[0][2] * p.z + self.rows[0][3],
            self.rows[1][0] * p.x + self.rows[1][1] * p.y + self.rows[1][2] * p.z + self.rows[1][3],
            self.rows[2][0] * p.x + self.rows[2][1] * p.y + self.rows[2][2] * p.z + self.rows[2][3],
        )
    }
}

pub type Mat4 = [f32; MATRIX_SIZE];

// Буквальный порт данного сниппета: row-major 4x4, w из последней строки, Y negated.
#[inline]
pub fn w2s(m: &[f32; MATRIX_SIZE], p: Vec3, sw: f32, sh: f32) -> Option<Vec2> {
    let w = m[12] * p.x + m[13] * p.y + m[14] * p.z + m[15];
    if w < W2S_MIN_W {
        return None;
    }
    let sx = (sw * 0.5)
        + ((m[0] * p.x + m[1] * p.y + m[2] * p.z + m[3]) / w) * (sw * 0.5);
    let sy = (sh * 0.5)
        - ((m[4] * p.x + m[5] * p.y + m[6] * p.z + m[7]) / w) * (sh * 0.5);
    Some(Vec2::new(sx, sy))
}
