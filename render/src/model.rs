// Config block: own skinned agent renderer. GLB is parsed offline into CPU data once,
// GPU resources are created once per D3D11 device; no runtime D3DCompile, no game reads.
use parking_lot::Mutex;
use wallbang_config::collections::HashMap;
use std::ffi::c_void;
use windows::Win32::Graphics::Direct3D::D3D11_PRIMITIVE_TOPOLOGY_TRIANGLELIST;
use windows::Win32::Graphics::Direct3D11::*;
use windows::Win32::Graphics::Dxgi::Common::*;
pub mod inspect;

const VS_BYTECODE: &[u8] = include_bytes!("shaders/model_vs.cso");
const PS_BYTECODE: &[u8] = include_bytes!("shaders/model_ps.cso");
const PHOENIX_GLB: &[u8] = include_bytes!("assets/tm_phoenix.glb");
const SAS_GLB: &[u8] = include_bytes!("assets/ctm_sas.glb");

const MAX_BONES: usize = 128;
pub const BONE_COUNT: usize = 17;
const CAMERA_FOV: f32 = 35.0f32.to_radians();
const FIT_MARGIN: f32 = 1.06;
/// VRF exports `t_idle_layer01` as an additive layer. Without the inherited
/// base animation its root rotation lays the agent on its side, so the preview
/// uses the exported bind pose. Static pose is allowed by the acceptance criteria.
const USE_ANIMATION: bool = false;

/// Chams override for the owned preview. Material indices match
/// `wallbang_resources::CHAM_MATERIALS`; colours are RGBA bytes.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MaterialOverride {
    pub enabled: bool,
    pub material: usize,
    pub color: [u8; 4],
}

impl MaterialOverride {
    pub const fn empty() -> Self {
        Self { enabled: false, material: 0, color: [0, 0, 0, 0] }
    }

    #[inline]
    pub fn packed_color(self) -> u32 {
        u32::from_le_bytes(self.color)
    }

    #[inline]
    pub fn ignore_z(self) -> bool {
        (11..=18).contains(&self.material)
    }

    #[inline]
    pub fn wireframe(self) -> bool {
        self.enabled && self.material == 19
    }

    /// Own-preview approximation of the C++ cham materials. Textured entries
    /// keep the GLB albedo; every other entry renders a flat tinted surface.
    #[inline]
    pub fn flat(self) -> bool {
        self.enabled && self.material > 2
    }
}

/// ESP joint order shared with `preview.rs`.
pub const HEAD: usize = 0;
pub const NECK: usize = 1;
pub const CHEST: usize = 2;
pub const WAIST: usize = 3;
pub const SHOULDER_L: usize = 4;
pub const SHOULDER_R: usize = 5;
pub const ELBOW_L: usize = 6;
pub const ELBOW_R: usize = 7;
pub const HAND_L: usize = 8;
pub const HAND_R: usize = 9;
pub const HIP_L: usize = 10;
pub const HIP_R: usize = 11;
pub const KNEE_L: usize = 12;
pub const KNEE_R: usize = 13;
pub const FOOT_L: usize = 14;
pub const FOOT_R: usize = 15;
/// Artificial point above `head_0` used for the ESP box height.
pub const HEAD_TOP: usize = 16;

#[repr(C)]
#[derive(Clone, Copy, Default)]
struct ModelVertex {
    pos: [f32; 3],
    normal: [f32; 3],
    uv: [f32; 2],
    joints: [u8; 4],
    weights: [f32; 4],
}

#[derive(Clone, Copy)]
struct Mat4([f32; 16]);

impl Mat4 {
    fn identity() -> Self {
        let mut m = [0.0f32; 16];
        m[0] = 1.0;
        m[5] = 1.0;
        m[10] = 1.0;
        m[15] = 1.0;
        Self(m)
    }

    fn mul(a: Self, b: Self) -> Self {
        let mut out = [0.0f32; 16];
        for col in 0..4 {
            for row in 0..4 {
                let mut v = 0.0;
                for k in 0..4 {
                    v += a.0[k * 4 + row] * b.0[col * 4 + k];
                }
                out[col * 4 + row] = v;
            }
        }
        Self(out)
    }

    fn transform(self, p: [f32; 3]) -> [f32; 4] {
        let m = &self.0;
        [
            m[0] * p[0] + m[4] * p[1] + m[8] * p[2] + m[12],
            m[1] * p[0] + m[5] * p[1] + m[9] * p[2] + m[13],
            m[2] * p[0] + m[6] * p[1] + m[10] * p[2] + m[14],
            m[3] * p[0] + m[7] * p[1] + m[11] * p[2] + m[15],
        ]
    }

    fn trs(t: [f32; 3], q: [f32; 4], s: [f32; 3]) -> Self {
        let [x, y, z, w] = q;
        let xx = x * x;
        let yy = y * y;
        let zz = z * z;
        let xy = x * y;
        let xz = x * z;
        let yz = y * z;
        let wx = w * x;
        let wy = w * y;
        let wz = w * z;
        let r = [
            1.0 - 2.0 * (yy + zz),
            2.0 * (xy + wz),
            2.0 * (xz - wy),
            2.0 * (xy - wz),
            1.0 - 2.0 * (xx + zz),
            2.0 * (yz + wx),
            2.0 * (xz + wy),
            2.0 * (yz - wx),
            1.0 - 2.0 * (xx + yy),
        ];
        let mut m = Self::identity();
        m.0[0] = r[0] * s[0];
        m.0[1] = r[1] * s[0];
        m.0[2] = r[2] * s[0];
        m.0[4] = r[3] * s[1];
        m.0[5] = r[4] * s[1];
        m.0[6] = r[5] * s[1];
        m.0[8] = r[6] * s[2];
        m.0[9] = r[7] * s[2];
        m.0[10] = r[8] * s[2];
        m.0[12] = t[0];
        m.0[13] = t[1];
        m.0[14] = t[2];
        m
    }

    fn translation(x: f32, y: f32, z: f32) -> Self {
        let mut m = Self::identity();
        m.0[12] = x;
        m.0[13] = y;
        m.0[14] = z;
        m
    }

    fn scale(s: f32) -> Self {
        let mut m = Self::identity();
        m.0[0] = s;
        m.0[5] = s;
        m.0[10] = s;
        m
    }

    fn rotate_y(angle: f32) -> Self {
        let (s, c) = angle.sin_cos();
        let mut m = Self::identity();
        m.0[0] = c;
        m.0[2] = -s;
        m.0[8] = s;
        m.0[10] = c;
        m
    }

    fn perspective(fov: f32, aspect: f32, near: f32, far: f32) -> Self {
        let f = 1.0 / (fov * 0.5).tan();
        let mut m = [0.0f32; 16];
        m[0] = f / aspect;
        m[5] = f;
        m[10] = far / (near - far);
        m[11] = -1.0;
        m[14] = far * near / (near - far);
        Self(m)
    }

    fn look_at(eye: [f32; 3], target: [f32; 3]) -> Self {
        let sub = |a: [f32; 3], b: [f32; 3]| [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
        let cross = |a: [f32; 3], b: [f32; 3]| {
            [
                a[1] * b[2] - a[2] * b[1],
                a[2] * b[0] - a[0] * b[2],
                a[0] * b[1] - a[1] * b[0],
            ]
        };
        let norm = |v: [f32; 3]| {
            let l = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt().max(1e-6);
            [v[0] / l, v[1] / l, v[2] / l]
        };
        let back = norm(sub(eye, target));
        let right = norm(cross([0.0, 1.0, 0.0], back));
        let up = cross(back, right);
        let dot = |a: [f32; 3], b: [f32; 3]| a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
        let mut m = Self::identity();
        m.0[0] = right[0];
        m.0[1] = up[0];
        m.0[2] = back[0];
        m.0[4] = right[1];
        m.0[5] = up[1];
        m.0[6] = back[1];
        m.0[8] = right[2];
        m.0[9] = up[2];
        m.0[10] = back[2];
        m.0[12] = -dot(right, eye);
        m.0[13] = -dot(up, eye);
        m.0[14] = -dot(back, eye);
        m
    }
}

#[derive(Clone, Copy)]
struct Primitive {
    index_offset: u32,
    index_count: u32,
    texture: usize,
    paintable: bool,
    alpha: bool,
}

#[derive(Clone, Copy, PartialEq)]
enum TrackProperty {
    Translation,
    Rotation,
    Scale,
}

struct Track {
    node: usize,
    property: TrackProperty,
    times: Vec<f32>,
    values: Vec<[f32; 4]>,
}

struct NodeData {
    parent: Option<usize>,
    translation: [f32; 3],
    rotation: [f32; 4],
    scale: [f32; 3],
}

struct TextureData {
    rgba: Vec<u8>,
    width: u32,
    height: u32,
}

/// Static GPU geometry plus the albedo textures referenced by its primitives.
struct Mesh {
    vertices: Vec<ModelVertex>,
    indices: Vec<u32>,
    primitives: Vec<Primitive>,
    textures: Vec<TextureData>,
}

/// Node hierarchy, skin joints and inverse bind pose for one agent.
struct Skeleton {
    nodes: Vec<NodeData>,
    joints: Vec<usize>,
    inverse_bind: Vec<Mat4>,
    bone_slots: HashMap<String, usize>,
    world: Mat4,
}

/// Sampled animation channels for one clip (idle in both exported agents).
struct Clip {
    tracks: Vec<Track>,
    duration: f32,
}

#[derive(Clone, Copy)]
struct Bounds {
    min: [f32; 3],
    max: [f32; 3],
}

impl Bounds {
    fn center(&self) -> [f32; 3] {
        [
            (self.min[0] + self.max[0]) * 0.5,
            (self.min[1] + self.max[1]) * 0.5,
            (self.min[2] + self.max[2]) * 0.5,
        ]
    }

    fn corners(&self) -> [[f32; 3]; 8] {
        let mut out = [[0.0; 3]; 8];
        for i in 0..8 {
            out[i] = [
                if i & 1 != 0 { self.max[0] } else { self.min[0] },
                if i & 2 != 0 { self.max[1] } else { self.min[1] },
                if i & 4 != 0 { self.max[2] } else { self.min[2] },
            ];
        }
        out
    }
}

struct AgentData {
    mesh: Mesh,
    skeleton: Skeleton,
    clip: Clip,
    bounds: Bounds,
    /// Horizontal vector from the head to the eyes: the model faces this way.
    forward: [f32; 3],
}

fn rgba(data: &gltf::image::Data) -> Option<TextureData> {
    let out = match data.format {
        gltf::image::Format::R8G8B8A8 => data.pixels.clone(),
        gltf::image::Format::R8G8B8 => {
            let mut px = Vec::with_capacity(data.width as usize * data.height as usize * 4);
            for c in data.pixels.chunks_exact(3) {
                px.extend_from_slice(&[c[0], c[1], c[2], 255]);
            }
            px
        }
        gltf::image::Format::R8 => {
            let mut px = Vec::with_capacity(data.width as usize * data.height as usize * 4);
            for &v in &data.pixels {
                px.extend_from_slice(&[v, v, v, 255]);
            }
            px
        }
        gltf::image::Format::R8G8 => {
            let mut px = Vec::with_capacity(data.width as usize * data.height as usize * 4);
            for c in data.pixels.chunks_exact(2) {
                px.extend_from_slice(&[c[0], c[0], c[0], c[1]]);
            }
            px
        }
        _ => return None,
    };
    Some(TextureData { rgba: out, width: data.width, height: data.height })
}

fn parse_agent(glb: &[u8]) -> Result<AgentData, String> {
    let (document, buffers, images) = gltf::import_slice(glb).map_err(|e| e.to_string())?;
    let get = |buffer: gltf::Buffer<'_>| buffers.get(buffer.index()).map(|d| d.0.as_slice());

    let mut nodes = Vec::new();
    let mut parents: Vec<Option<usize>> = Vec::new();
    for node in document.nodes() {
        nodes.push(NodeData {
            parent: None,
            translation: node.transform().decomposed().0,
            rotation: node.transform().decomposed().1,
            scale: node.transform().decomposed().2,
        });
        parents.push(None);
    }
    for node in document.nodes() {
        let parent = node.index();
        for child in node.children() {
            if let Some(slot) = parents.get_mut(child.index()) {
                *slot = Some(parent);
            }
        }
    }
    for (i, parent) in parents.iter().enumerate() {
        nodes[i].parent = *parent;
    }

    let mut textures = Vec::new();
    for image in images.iter() {
        if let Some(tex) = rgba(image) {
            textures.push(tex);
        }
    }
    if textures.is_empty() {
        textures.push(TextureData { rgba: vec![180, 170, 150, 255], width: 1, height: 1 });
    }

    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    let mut primitives = Vec::new();
    for mesh in document.meshes() {
        for primitive in mesh.primitives() {
            let reader = primitive.reader(get.clone());
            let positions: Vec<[f32; 3]> = reader.read_positions().map(|i| i.collect()).unwrap_or_default();
            if positions.is_empty() {
                continue;
            }
            let normals: Vec<[f32; 3]> = reader.read_normals().map(|i| i.collect()).unwrap_or_default();
            let uvs: Vec<[f32; 2]> = reader.read_tex_coords(0).map(|i| i.into_f32().collect()).unwrap_or_default();
            let joints_raw = reader.read_joints(0);
            let weights_raw = reader.read_weights(0);
            let index_base = vertices.len() as u32;
            for i in 0..positions.len() {
                let mut joints = [0u8; 4];
                if let Some(reader) = &joints_raw {
                    match reader {
                        gltf::mesh::util::ReadJoints::U8(iter) => {
                            if let Some(j) = iter.clone().nth(i) {
                                joints = j;
                            }
                        }
                        gltf::mesh::util::ReadJoints::U16(iter) => {
                            if let Some(j) = iter.clone().nth(i) {
                                joints = [j[0] as u8, j[1] as u8, j[2] as u8, j[3] as u8];
                            }
                        }
                    }
                }
                let mut weights = [0.0f32; 4];
                if let Some(reader) = &weights_raw {
                    if let Some(w) = reader.clone().into_f32().nth(i) {
                        weights = w;
                    }
                }
                if weights[0] + weights[1] + weights[2] + weights[3] <= 1e-6 {
                    weights[0] = 1.0;
                }
                // Joint indices are u8 in the vertex layout; never index past the cbuffer.
                joints = joints.map(|j| j.min((MAX_BONES - 1) as u8));
                vertices.push(ModelVertex {
                    pos: positions[i],
                    normal: *normals.get(i).unwrap_or(&[0.0, 1.0, 0.0]),
                    uv: *uvs.get(i).unwrap_or(&[0.0, 0.0]),
                    joints,
                    weights,
                });
            }
            let index_offset = indices.len() as u32;
            if let Some(iter) = reader.read_indices() {
                for value in iter.into_u32() {
                    indices.push(index_base + value);
                }
            } else {
                for i in 0..positions.len() as u32 {
                    indices.push(index_base + i);
                }
            }
            let texture = primitive
                .material()
                .pbr_metallic_roughness()
                .base_color_texture()
                .map(|info| info.texture().index())
                .unwrap_or(0);
            let texture = document
                .textures()
                .nth(texture)
                .map(|texture| texture.source().index())
                .unwrap_or(texture);
            primitives.push(Primitive {
                index_offset,
                index_count: (indices.len() as u32) - index_offset,
                texture: texture.min(textures.len() - 1),
                paintable: false,
                alpha: false,
            });
        }
    }

    let mut joints = Vec::new();
    let mut inverse_bind = Vec::new();
    let mut bone_slots = HashMap::default();
    if let Some(skin) = document.skins().next() {
        if let Some(matrices) = skin.reader(get.clone()).read_inverse_bind_matrices() {
            for matrix in matrices {
                let mut mat = [0.0f32; 16];
                let mut offset = 0;
                for column in matrix {
                    for value in column {
                        mat[offset] = value;
                        offset += 1;
                    }
                }
                inverse_bind.push(Mat4(mat));
            }
        }
        for (slot, node) in skin.joints().enumerate() {
            if let Some(name) = node.name() {
                bone_slots.insert(name.to_string(), slot);
            }
            joints.push(node.index());
        }
    }
    while inverse_bind.len() < joints.len() {
        inverse_bind.push(Mat4::identity());
    }
    inverse_bind.truncate(joints.len());
    joints.truncate(MAX_BONES);
    inverse_bind.truncate(MAX_BONES);

    let mut tracks = Vec::new();
    let mut duration = 0.0f32;
    for animation in document.animations() {
        for channel in animation.channels() {
            let node = channel.target().node().index();
            let property = match channel.target().property() {
                gltf::animation::Property::Translation => TrackProperty::Translation,
                gltf::animation::Property::Rotation => TrackProperty::Rotation,
                gltf::animation::Property::Scale => TrackProperty::Scale,
                gltf::animation::Property::MorphTargetWeights => continue,
            };
            let reader = channel.reader(get.clone());
            let Some(times) = reader.read_inputs() else { continue };
            let times: Vec<f32> = times.collect();
            if times.is_empty() {
                continue;
            }
            let Some(outputs) = reader.read_outputs() else { continue };
            let mut values = Vec::new();
            match outputs {
                gltf::animation::util::ReadOutputs::Translations(iter) => {
                    for v in iter {
                        values.push([v[0], v[1], v[2], 0.0]);
                    }
                }
                gltf::animation::util::ReadOutputs::Scales(iter) => {
                    for v in iter {
                        values.push([v[0], v[1], v[2], 0.0]);
                    }
                }
                gltf::animation::util::ReadOutputs::Rotations(rot) => {
                    for v in rot.into_f32() {
                        values.push(v);
                    }
                }
                gltf::animation::util::ReadOutputs::MorphTargetWeights(_) => continue,
            }
            if values.is_empty() {
                continue;
            }
            duration = duration.max(*times.last().unwrap_or(&0.0));
            tracks.push(Track { node, property, times, values });
        }
    }

    let mut data = AgentData {
        mesh: Mesh { vertices, indices, primitives, textures },
        skeleton: Skeleton {
            nodes,
            joints,
            inverse_bind,
            bone_slots,
            world: Mat4::identity(),
        },
        clip: Clip { tracks, duration },
        bounds: Bounds {
            min: [0.0; 3],
            max: [0.0; 3],
        },
        forward: [0.0, 0.0, 1.0],
    };
    let globals = data.globals(0.0);
    let mut min = [f32::INFINITY; 3];
    let mut max = [f32::NEG_INFINITY; 3];
    for name in ["pelvis", "head_0", "hand_L", "hand_R", "ankle_L", "ankle_R"] {
        let Some(&slot) = data.skeleton.bone_slots.get(name) else { continue };
        let node = data.skeleton.joints[slot];
        let p = globals[node].transform([0.0, 0.0, 0.0]);
        for i in 0..3 {
            min[i] = min[i].min(p[i]);
            max[i] = max[i].max(p[i]);
        }
    }
    if min[0].is_finite() && max[1] > min[1] {
        let scale = 1.78 / (max[1] - min[1]);
        let cx = (min[0] + max[0]) * 0.5;
        let cz = (min[2] + max[2]) * 0.5;
        let normalized = Mat4::mul(
            Mat4::translation(-cx * scale, -min[1] * scale, -cz * scale),
            Mat4::scale(scale),
        );
        data.skeleton.world = Mat4::mul(Mat4::rotate_y(std::f32::consts::PI), normalized);
        data.bounds = data.compute_bounds();
        data.forward = data.compute_forward();
    }
    Ok(data)
}

impl AgentData {
    fn world_vertex(&self, v: &ModelVertex, matrices: &[[f32; 16]; MAX_BONES]) -> [f32; 3] {
        let sum: f32 = v.weights.iter().sum();
        let mut p = [v.pos[0], v.pos[1], v.pos[2], 1.0];
        if sum > 0.0 {
            p = [0.0; 4];
            for k in 0..4 {
                let raw = matrices
                    .get(v.joints[k] as usize)
                    .copied()
                    .unwrap_or_else(|| Mat4::identity().0);
                let q = Mat4(raw).transform(v.pos);
                let w = v.weights[k] / sum;
                for i in 0..4 {
                    p[i] += q[i] * w;
                }
            }
        }
        let world = self.skeleton.world.transform([p[0], p[1], p[2]]);
        [world[0], world[1], world[2]]
    }

    fn compute_bounds(&self) -> Bounds {
        let matrices = self.bones(0.0);
        let mut bounds = Bounds {
            min: [f32::INFINITY; 3],
            max: [f32::NEG_INFINITY; 3],
        };
        for vertex in &self.mesh.vertices {
            let p = self.world_vertex(vertex, &matrices);
            for i in 0..3 {
                bounds.min[i] = bounds.min[i].min(p[i]);
                bounds.max[i] = bounds.max[i].max(p[i]);
            }
        }
        if !bounds.min[0].is_finite() {
            bounds = Bounds { min: [-0.5, 0.0, -0.5], max: [0.5, 1.78, 0.5] };
        }
        bounds
    }

    fn node_world(&self, globals: &[Mat4], name: &str) -> Option<[f32; 3]> {
        let &slot = self.skeleton.bone_slots.get(name)?;
        let node = *self.skeleton.joints.get(slot)?;
        let local = globals.get(node)?.transform([0.0, 0.0, 0.0]);
        let world = self.skeleton.world.transform([local[0], local[1], local[2]]);
        Some([world[0], world[1], world[2]])
    }

    fn compute_forward(&self) -> [f32; 3] {
        let globals = self.globals(0.0);
        let (Some(head), Some(left), Some(right)) = (
            self.node_world(&globals, "head_0"),
            self.node_world(&globals, "eyeball_l"),
            self.node_world(&globals, "eyeball_r"),
        ) else {
            return [0.0, 0.0, 1.0];
        };
        let forward = [
            (left[0] + right[0]) * 0.5 - head[0],
            0.0,
            (left[2] + right[2]) * 0.5 - head[2],
        ];
        let len = (forward[0] * forward[0] + forward[2] * forward[2]).sqrt();
        if len < 1e-4 {
            [0.0, 0.0, 1.0]
        } else {
            [forward[0] / len, 0.0, forward[2] / len]
        }
    }

    /// Fixed front-facing camera framed from the exported mesh bounds.
    /// Returns `(eye, focus)`; the same pair is used for bone projection.
    fn camera(&self, aspect: f32) -> ([f32; 3], [f32; 3]) {
        let center = self.bounds.center();
        let dot = |a: [f32; 3], b: [f32; 3]| a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
        let cross = |a: [f32; 3], b: [f32; 3]| {
            [
                a[1] * b[2] - a[2] * b[1],
                a[2] * b[0] - a[0] * b[2],
                a[0] * b[1] - a[1] * b[0],
            ]
        };
        let norm = |v: [f32; 3]| {
            let len = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt().max(1e-6);
            [v[0] / len, v[1] / len, v[2] / len]
        };
        let forward = norm(self.forward);
        let right = norm(cross([0.0, 1.0, 0.0], forward));
        let up = cross(forward, right);
        let mut half_w = 0.0f32;
        let mut half_h = 0.0f32;
        let mut depth = 0.0f32;
        for corner in self.bounds.corners() {
            let d = [
                corner[0] - center[0],
                corner[1] - center[1],
                corner[2] - center[2],
            ];
            half_w = half_w.max(dot(d, right).abs());
            half_h = half_h.max(dot(d, up).abs());
            depth = depth.max(dot(d, forward));
        }
        let tan = (CAMERA_FOV * 0.5).tan();
        let distance = (half_h / tan)
            .max(half_w / (tan * aspect.max(0.1)))
            * FIT_MARGIN
            + depth
            + 0.2;
        let eye = [
            center[0] + forward[0] * distance,
            center[1] + forward[1] * distance,
            center[2] + forward[2] * distance,
        ];
        (eye, center)
    }

    fn globals(&self, time: f32) -> Vec<Mat4> {
        let n = self.skeleton.nodes.len();
        let mut locals = Vec::with_capacity(n);
        for node in &self.skeleton.nodes {
            locals.push(Mat4::trs(node.translation, node.rotation, node.scale));
        }
        for track in self.clip.tracks.iter().filter(|_| USE_ANIMATION) {
            if track.times.len() < 2 || track.node >= n {
                continue;
            }
            let t = if self.clip.duration > 0.0 { time % self.clip.duration } else { 0.0 };
            let mut hi = 1usize;
            while hi < track.times.len() && track.times[hi] < t {
                hi += 1;
            }
            let lo = hi - 1;
            let t0 = track.times[lo];
            let t1 = track.times[hi];
            let f = if t1 > t0 { ((t - t0) / (t1 - t0)).clamp(0.0, 1.0) } else { 0.0 };
            let a = track.values[lo];
            let b = track.values[hi];
            let node = &self.skeleton.nodes[track.node];
            let local = match track.property {
                TrackProperty::Translation => Mat4::trs(
                    [
                        a[0] + (b[0] - a[0]) * f,
                        a[1] + (b[1] - a[1]) * f,
                        a[2] + (b[2] - a[2]) * f,
                    ],
                    node.rotation,
                    node.scale,
                ),
                TrackProperty::Scale => Mat4::trs(
                    node.translation,
                    node.rotation,
                    [
                        a[0] + (b[0] - a[0]) * f,
                        a[1] + (b[1] - a[1]) * f,
                        a[2] + (b[2] - a[2]) * f,
                    ],
                ),
                TrackProperty::Rotation => {
                    let dot = a[0] * b[0] + a[1] * b[1] + a[2] * b[2] + a[3] * b[3];
                    let mut bb = b;
                    if dot < 0.0 {
                        for v in &mut bb {
                            *v = -*v;
                        }
                    }
                    let mut q = [
                        a[0] + (bb[0] - a[0]) * f,
                        a[1] + (bb[1] - a[1]) * f,
                        a[2] + (bb[2] - a[2]) * f,
                        a[3] + (bb[3] - a[3]) * f,
                    ];
                    let len = (q[0] * q[0] + q[1] * q[1] + q[2] * q[2] + q[3] * q[3])
                        .sqrt()
                        .max(1e-6);
                    for v in &mut q {
                        *v /= len;
                    }
                    Mat4::trs(node.translation, q, node.scale)
                }
            };
            locals[track.node] = local;
        }

        if !USE_ANIMATION {
            self.apply_idle_pose(&mut locals, time);
        }

        let mut globals = vec![Mat4::identity(); n];
        let mut done = vec![false; n];
        for i in 0..n {
            Self::compute_global(i, &self.skeleton.nodes, &locals, &mut globals, &mut done);
        }
        globals
    }

    fn apply_idle_pose(&self, locals: &mut [Mat4], time: f32) {
        let euler_quat = |rx: f32, ry: f32, rz: f32| -> [f32; 4] {
            let (sx, cx) = (rx * 0.5).sin_cos();
            let (sy, cy) = (ry * 0.5).sin_cos();
            let (sz, cz) = (rz * 0.5).sin_cos();
            [
                sx * cy * cz - cx * sy * sz,
                cx * sy * cz + sx * cy * sz,
                cx * cy * sz - sx * sy * cz,
                cx * cy * cz + sx * sy * sz,
            ]
        };
        let quat_mul = |a: [f32; 4], b: [f32; 4]| -> [f32; 4] {
            [
                a[3] * b[0] + a[0] * b[3] + a[1] * b[2] - a[2] * b[1],
                a[3] * b[1] - a[0] * b[2] + a[1] * b[3] + a[2] * b[0],
                a[3] * b[2] + a[0] * b[1] - a[1] * b[0] + a[2] * b[3],
                a[3] * b[3] - a[0] * b[0] - a[1] * b[1] - a[2] * b[2],
            ]
        };
        let mut rot_bone = |name: &str, rx: f32, ry: f32, rz: f32| {
            let Some(&slot) = self.skeleton.bone_slots.get(name) else { return };
            let Some(&node_idx) = self.skeleton.joints.get(slot) else { return };
            let Some(node) = self.skeleton.nodes.get(node_idx) else { return };
            let delta = euler_quat(rx, ry, rz);
            let q = quat_mul(node.rotation, delta);
            locals[node_idx] = Mat4::trs(node.translation, q, node.scale);
        };

        let breath = (time * 1.6).sin();
        let sway = (time * 0.85).sin();
        rot_bone("spine_1", breath * 0.02, 0.0, 0.0);
        rot_bone("spine_2", breath * 0.025, sway * 0.015, 0.0);
        rot_bone("head_0", -breath * 0.015, -0.06 + sway * 0.03, 0.0);
        // Relaxed combat idle stance: lower both arms symmetrically and bend elbows slightly forward
        rot_bone("arm_upper_L", 0.12, -0.15, 0.72);
        rot_bone("arm_upper_R", 0.12, 0.15, 0.72);
        rot_bone("arm_lower_L", 0.0, -0.28, 0.10);
        rot_bone("arm_lower_R", 0.0, 0.28, 0.10);
    }

    fn compute_global(
        index: usize,
        nodes: &[NodeData],
        locals: &[Mat4],
        globals: &mut [Mat4],
        done: &mut [bool],
    ) {
        if done[index] {
            return;
        }
        done[index] = true;
        if let Some(parent) = nodes[index].parent {
            if parent < nodes.len() {
                Self::compute_global(parent, nodes, locals, globals, done);
                globals[index] = Mat4::mul(globals[parent], locals[index]);
                return;
            }
        }
        globals[index] = locals[index];
    }

    fn bones(&self, time: f32) -> [[f32; 16]; MAX_BONES] {
        let globals = self.globals(time);
        let mut out = [[0.0f32; 16]; MAX_BONES];
        for i in 0..MAX_BONES {
            out[i] = Mat4::identity().0;
        }
        for (slot, &node) in self.skeleton.joints.iter().enumerate().take(MAX_BONES) {
            out[slot] = Mat4::mul(globals[node], self.skeleton.inverse_bind[slot]).0;
        }
        out
    }

    fn joint_positions(
        &self,
        time: f32,
        view_proj: Mat4,
        rect: (f32, f32, f32, f32),
    ) -> [Option<(f32, f32)>; BONE_COUNT] {
        let globals = self.globals(time);
        let names = [
            "head_0",
            "neck_0",
            "spine_2",
            "pelvis",
            "arm_upper_L",
            "arm_upper_R",
            "arm_lower_L",
            "arm_lower_R",
            "hand_L",
            "hand_R",
            "leg_upper_L",
            "leg_upper_R",
            "leg_lower_L",
            "leg_lower_R",
            "ankle_L",
            "ankle_R",
            "head_0",
        ];
        let mut out = [None; BONE_COUNT];
        for (i, name) in names.iter().enumerate() {
            let Some(&slot) = self.skeleton.bone_slots.get(*name) else { continue };
            let node = self.skeleton.joints[slot];
            let local = globals[node].transform([0.0, 0.0, 0.0]);
            let world = self.skeleton.world.transform([local[0], local[1], local[2]]);
            let clip = view_proj.transform([world[0], world[1], world[2]]);
            if clip[3] <= 0.0001 {
                continue;
            }
            let ndc_x = clip[0] / clip[3];
            let ndc_y = clip[1] / clip[3];
            if !ndc_x.is_finite() || !ndc_y.is_finite() {
                continue;
            }
            let x = rect.0 + (ndc_x * 0.5 + 0.5) * rect.2;
            let mut y = rect.1 + (1.0 - (ndc_y * 0.5 + 0.5)) * rect.3;
            if i == HEAD_TOP {
                y -= 18.0;
            }
            if x.is_finite() && y.is_finite() {
                out[i] = Some((x, y));
            }
        }
        out
    }
}

#[repr(C)]
#[allow(dead_code)] // Raw bytes are uploaded with UpdateSubresource.
struct SceneBuffer {
    view_proj: [f32; 16],
    world: [f32; 16],
    light: [f32; 4],
    camera: [f32; 4],
    tint: [f32; 4],
}

struct GpuAgent {
    vertex_buffer: ID3D11Buffer,
    index_buffer: ID3D11Buffer,
    bone_buffer: ID3D11Buffer,
    textures: Vec<ID3D11ShaderResourceView>,
    primitives: Vec<Primitive>,
    data: AgentData,
}

#[allow(dead_code)] // Texture/depth are kept alive for the RTV/DSV/SRV.
struct Target {
    texture: ID3D11Texture2D,
    rtv: ID3D11RenderTargetView,
    srv: ID3D11ShaderResourceView,
    depth: ID3D11Texture2D,
    dsv: ID3D11DepthStencilView,
    w: u32,
    h: u32,
}

pub struct ModelRenderer {
    vertex_shader: ID3D11VertexShader,
    pixel_shader: ID3D11PixelShader,
    input_layout: ID3D11InputLayout,
    scene_buffer: ID3D11Buffer,
    sampler: ID3D11SamplerState,
    blend: ID3D11BlendState,
    rasterizer: ID3D11RasterizerState,
    rasterizer_wireframe: ID3D11RasterizerState,
    depth_state: ID3D11DepthStencilState,
    depth_ignore_z: ID3D11DepthStencilState,
    white: ID3D11ShaderResourceView,
    phoenix: Option<GpuAgent>,
    sas: Option<GpuAgent>,
    target: Option<Target>,
    /// Static pose: the GPU pass only needs to run when model/size changes.
    cache_key: Option<(u32, u32, bool, u32, u32, u32)>,
    cached_srv: Option<ID3D11ShaderResourceView>,
    last_projected: [Option<(f32, f32)>; BONE_COUNT],
    last_ok: bool,
}

static PROJECTED: Mutex<[Option<(f32, f32)>; BONE_COUNT]> = Mutex::new([None; BONE_COUNT]);

/// ESP bones projected by the last owned-model pass, in preview-window pixels.
pub fn projected_bones() -> [Option<(f32, f32)>; BONE_COUNT] {
    *PROJECTED.lock()
}

pub fn available() -> bool {
    *RESULT.lock()
}

pub fn bone_count() -> usize {
    BONE_COUNT
}

static RESULT: Mutex<bool> = Mutex::new(false);

/// Constant buffers are updated with UpdateSubresource once per frame, so they
/// must be DEFAULT allocations; DYNAMIC buffers are written through Map.
unsafe fn create_constant_buffer(device: &ID3D11Device, bytes: usize) -> Option<ID3D11Buffer> {
    let desc = D3D11_BUFFER_DESC {
        ByteWidth: bytes as u32,
        Usage: D3D11_USAGE_DEFAULT,
        BindFlags: D3D11_BIND_CONSTANT_BUFFER.0 as u32,
        CPUAccessFlags: 0,
        MiscFlags: 0,
        StructureByteStride: 0,
    };
    let mut slot = None;
    device.CreateBuffer(&desc, None, Some(&mut slot)).ok()?;
    slot
}

unsafe fn create_buffer(
    device: &ID3D11Device,
    bytes: usize,
    bind: u32,
    data: Option<&[u8]>,
) -> Option<ID3D11Buffer> {
    let desc = D3D11_BUFFER_DESC {
        ByteWidth: bytes as u32,
        Usage: if data.is_some() {
            D3D11_USAGE_DEFAULT
        } else {
            D3D11_USAGE_DYNAMIC
        },
        BindFlags: bind,
        CPUAccessFlags: if data.is_some() {
            0
        } else {
            D3D11_CPU_ACCESS_WRITE.0 as u32
        },
        MiscFlags: 0,
        StructureByteStride: 0,
    };
    let sub = data.map(|bytes| D3D11_SUBRESOURCE_DATA {
        pSysMem: bytes.as_ptr() as *const c_void,
        SysMemPitch: bytes.len() as u32,
        SysMemSlicePitch: 0,
    });
    let sub_ptr = sub.as_ref().map(|data| data as *const D3D11_SUBRESOURCE_DATA);
    let mut slot = None;
    device.CreateBuffer(&desc, sub_ptr, Some(&mut slot)).ok()?;
    slot
}

unsafe fn create_texture_srv(
    device: &ID3D11Device,
    rgba: &[u8],
    width: u32,
    height: u32,
) -> Option<ID3D11ShaderResourceView> {
    let desc = D3D11_TEXTURE2D_DESC {
        Width: width,
        Height: height,
        MipLevels: 1,
        ArraySize: 1,
        Format: DXGI_FORMAT_R8G8B8A8_UNORM,
        SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
        Usage: D3D11_USAGE_DEFAULT,
        BindFlags: D3D11_BIND_SHADER_RESOURCE.0 as u32,
        CPUAccessFlags: 0,
        MiscFlags: 0,
    };
    let sub = D3D11_SUBRESOURCE_DATA {
        pSysMem: rgba.as_ptr() as *const c_void,
        SysMemPitch: width * 4,
        SysMemSlicePitch: 0,
    };
    let mut texture = None;
    device.CreateTexture2D(&desc, Some(&sub), Some(&mut texture)).ok()?;
    let texture = texture?;
    let mut srv = None;
    device.CreateShaderResourceView(&texture, None, Some(&mut srv)).ok()?;
    srv
}

impl ModelRenderer {
    pub unsafe fn new(device: &ID3D11Device) -> Option<Self> {
        Self::new_pipeline(device, true)
    }
    unsafe fn new_pipeline(device: &ID3D11Device, load_agents: bool) -> Option<Self> {
        let mut vs = None;
        device.CreateVertexShader(VS_BYTECODE, None, Some(&mut vs)).ok()?;
        let mut ps = None;
        device.CreatePixelShader(PS_BYTECODE, None, Some(&mut ps)).ok()?;
        let elements = [
            D3D11_INPUT_ELEMENT_DESC {
                SemanticName: windows::core::PCSTR(b"POSITION\0".as_ptr()),
                SemanticIndex: 0,
                Format: DXGI_FORMAT_R32G32B32_FLOAT,
                InputSlot: 0,
                AlignedByteOffset: 0,
                InputSlotClass: D3D11_INPUT_PER_VERTEX_DATA,
                InstanceDataStepRate: 0,
            },
            D3D11_INPUT_ELEMENT_DESC {
                SemanticName: windows::core::PCSTR(b"NORMAL\0".as_ptr()),
                SemanticIndex: 0,
                Format: DXGI_FORMAT_R32G32B32_FLOAT,
                InputSlot: 0,
                AlignedByteOffset: 12,
                InputSlotClass: D3D11_INPUT_PER_VERTEX_DATA,
                InstanceDataStepRate: 0,
            },
            D3D11_INPUT_ELEMENT_DESC {
                SemanticName: windows::core::PCSTR(b"TEXCOORD\0".as_ptr()),
                SemanticIndex: 0,
                Format: DXGI_FORMAT_R32G32_FLOAT,
                InputSlot: 0,
                AlignedByteOffset: 24,
                InputSlotClass: D3D11_INPUT_PER_VERTEX_DATA,
                InstanceDataStepRate: 0,
            },
            D3D11_INPUT_ELEMENT_DESC {
                SemanticName: windows::core::PCSTR(b"JOINTS\0".as_ptr()),
                SemanticIndex: 0,
                Format: DXGI_FORMAT_R8G8B8A8_UINT,
                InputSlot: 0,
                AlignedByteOffset: 32,
                InputSlotClass: D3D11_INPUT_PER_VERTEX_DATA,
                InstanceDataStepRate: 0,
            },
            D3D11_INPUT_ELEMENT_DESC {
                SemanticName: windows::core::PCSTR(b"WEIGHTS\0".as_ptr()),
                SemanticIndex: 0,
                Format: DXGI_FORMAT_R32G32B32A32_FLOAT,
                InputSlot: 0,
                AlignedByteOffset: 36,
                InputSlotClass: D3D11_INPUT_PER_VERTEX_DATA,
                InstanceDataStepRate: 0,
            },
        ];
        let mut layout = None;
        device.CreateInputLayout(&elements, VS_BYTECODE, Some(&mut layout)).ok()?;

        let scene_buffer = create_constant_buffer(device, 176)?;
        let sampler_desc = D3D11_SAMPLER_DESC {
            Filter: D3D11_FILTER_MIN_MAG_MIP_LINEAR,
            AddressU: D3D11_TEXTURE_ADDRESS_CLAMP,
            AddressV: D3D11_TEXTURE_ADDRESS_CLAMP,
            AddressW: D3D11_TEXTURE_ADDRESS_CLAMP,
            MipLODBias: 0.0,
            MaxAnisotropy: 1,
            ComparisonFunc: D3D11_COMPARISON_NEVER,
            BorderColor: [0.0; 4],
            MinLOD: 0.0,
            MaxLOD: 0.0,
        };
        let mut sampler = None;
        device.CreateSamplerState(&sampler_desc, Some(&mut sampler)).ok()?;
        let mut blend = None;
        device
            .CreateBlendState(&D3D11_BLEND_DESC {
                AlphaToCoverageEnable: false.into(),
                IndependentBlendEnable: false.into(),
                RenderTarget: [D3D11_RENDER_TARGET_BLEND_DESC {
                    BlendEnable: false.into(),
                    SrcBlend: D3D11_BLEND_ONE,
                    DestBlend: D3D11_BLEND_ZERO,
                    BlendOp: D3D11_BLEND_OP_ADD,
                    SrcBlendAlpha: D3D11_BLEND_ONE,
                    DestBlendAlpha: D3D11_BLEND_ZERO,
                    BlendOpAlpha: D3D11_BLEND_OP_ADD,
                    RenderTargetWriteMask: D3D11_COLOR_WRITE_ENABLE_ALL.0 as u8,
                }; 8],
            }, Some(&mut blend))
            .ok()?;
        let mut rasterizer = None;
        device
            .CreateRasterizerState(&D3D11_RASTERIZER_DESC {
                FillMode: D3D11_FILL_SOLID,
                CullMode: D3D11_CULL_NONE,
                FrontCounterClockwise: false.into(),
                DepthBias: 0,
                DepthBiasClamp: 0.0,
                SlopeScaledDepthBias: 0.0,
                DepthClipEnable: true.into(),
                ScissorEnable: false.into(),
                MultisampleEnable: false.into(),
                AntialiasedLineEnable: false.into(),
            }, Some(&mut rasterizer))
            .ok()?;
        let mut rasterizer_wireframe = None;
        device
            .CreateRasterizerState(&D3D11_RASTERIZER_DESC {
                FillMode: D3D11_FILL_WIREFRAME,
                CullMode: D3D11_CULL_NONE,
                FrontCounterClockwise: false.into(),
                DepthBias: 0,
                DepthBiasClamp: 0.0,
                SlopeScaledDepthBias: 0.0,
                DepthClipEnable: true.into(),
                ScissorEnable: false.into(),
                MultisampleEnable: false.into(),
                AntialiasedLineEnable: false.into(),
            }, Some(&mut rasterizer_wireframe))
            .ok()?;
        let mut depth_state = None;
        device
            .CreateDepthStencilState(&D3D11_DEPTH_STENCIL_DESC {
                DepthEnable: true.into(),
                DepthWriteMask: D3D11_DEPTH_WRITE_MASK_ALL,
                DepthFunc: D3D11_COMPARISON_LESS,
                StencilEnable: false.into(),
                StencilReadMask: 0,
                StencilWriteMask: 0,
                FrontFace: D3D11_DEPTH_STENCILOP_DESC {
                    StencilFailOp: D3D11_STENCIL_OP_KEEP,
                    StencilDepthFailOp: D3D11_STENCIL_OP_KEEP,
                    StencilPassOp: D3D11_STENCIL_OP_KEEP,
                    StencilFunc: D3D11_COMPARISON_ALWAYS,
                },
                BackFace: D3D11_DEPTH_STENCILOP_DESC {
                    StencilFailOp: D3D11_STENCIL_OP_KEEP,
                    StencilDepthFailOp: D3D11_STENCIL_OP_KEEP,
                    StencilPassOp: D3D11_STENCIL_OP_KEEP,
                    StencilFunc: D3D11_COMPARISON_ALWAYS,
                },
            }, Some(&mut depth_state))
            .ok()?;
        let mut depth_ignore_z = None;
        device
            .CreateDepthStencilState(&D3D11_DEPTH_STENCIL_DESC {
                DepthEnable: false.into(),
                DepthWriteMask: D3D11_DEPTH_WRITE_MASK_ZERO,
                DepthFunc: D3D11_COMPARISON_ALWAYS,
                StencilEnable: false.into(),
                StencilReadMask: 0,
                StencilWriteMask: 0,
                FrontFace: D3D11_DEPTH_STENCILOP_DESC {
                    StencilFailOp: D3D11_STENCIL_OP_KEEP,
                    StencilDepthFailOp: D3D11_STENCIL_OP_KEEP,
                    StencilPassOp: D3D11_STENCIL_OP_KEEP,
                    StencilFunc: D3D11_COMPARISON_ALWAYS,
                },
                BackFace: D3D11_DEPTH_STENCILOP_DESC {
                    StencilFailOp: D3D11_STENCIL_OP_KEEP,
                    StencilDepthFailOp: D3D11_STENCIL_OP_KEEP,
                    StencilPassOp: D3D11_STENCIL_OP_KEEP,
                    StencilFunc: D3D11_COMPARISON_ALWAYS,
                },
            }, Some(&mut depth_ignore_z))
            .ok()?;
        let white = create_texture_srv(device, &[190, 180, 160, 255], 1, 1)?;

        let phoenix = if load_agents { match parse_agent(PHOENIX_GLB) {
            Ok(data) => Self::upload(device, data),
            Err(error) => {
                crate::trace(&format!("model: tm_phoenix parse failed: {error}"));
                None
            }
        }} else { None };
        let sas = if load_agents { match parse_agent(SAS_GLB) {
            Ok(data) => Self::upload(device, data),
            Err(error) => {
                crate::trace(&format!("model: ctm_sas parse failed: {error}"));
                None
            }
        }} else { None };
        let ok = phoenix.is_some() && sas.is_some();
        if load_agents { *RESULT.lock() = ok; }
        if ok {
            crate::trace("model: own agents ready tm_phoenix/ctm_sas");
        }
        Some(Self {
            vertex_shader: vs?,
            pixel_shader: ps?,
            input_layout: layout?,
            scene_buffer,
            sampler: sampler?,
            blend: blend?,
            rasterizer: rasterizer?,
            rasterizer_wireframe: rasterizer_wireframe?,
            depth_state: depth_state?,
            depth_ignore_z: depth_ignore_z?,
            white,
            phoenix,
            sas,
            target: None,
            cache_key: None,
            cached_srv: None,
            last_projected: [None; BONE_COUNT],
            last_ok: false,
        })
    }

    unsafe fn upload(device: &ID3D11Device, data: AgentData) -> Option<GpuAgent> {
        let vertices = data.mesh.vertices.as_ptr() as *const c_void;
        let vertex_bytes = std::mem::size_of_val(data.mesh.vertices.as_slice());
        let indices = data.mesh.indices.as_ptr() as *const c_void;
        let index_bytes = std::mem::size_of_val(data.mesh.indices.as_slice());
        let vertex_buffer = create_buffer(
            device,
            vertex_bytes,
            D3D11_BIND_VERTEX_BUFFER.0 as u32,
            Some(std::slice::from_raw_parts(vertices as *const u8, vertex_bytes)),
        )?;
        let index_buffer = create_buffer(
            device,
            index_bytes,
            D3D11_BIND_INDEX_BUFFER.0 as u32,
            Some(std::slice::from_raw_parts(indices as *const u8, index_bytes)),
        )?;
        let bone_buffer = create_constant_buffer(device, MAX_BONES * 64)?;
        let mut textures = Vec::with_capacity(data.mesh.textures.len());
        for tex in &data.mesh.textures {
            if let Some(srv) = create_texture_srv(device, &tex.rgba, tex.width, tex.height) {
                textures.push(srv);
            }
        }
        if textures.is_empty() {
            return None;
        }
        Some(GpuAgent {
            vertex_buffer,
            index_buffer,
            bone_buffer,
            textures,
            primitives: data.mesh.primitives.clone(),
            data,
        })
    }

    unsafe fn ensure_target(&mut self, context: &ID3D11DeviceContext, w: u32, h: u32) -> Option<()> {
        let w = w.max(2);
        let h = h.max(2);
        if let Some(target) = &self.target {
            if target.w == w && target.h == h {
                return Some(());
            }
        }
        let device = context.GetDevice().ok()?;
        let tex_desc = D3D11_TEXTURE2D_DESC {
            Width: w,
            Height: h,
            MipLevels: 1,
            ArraySize: 1,
            Format: DXGI_FORMAT_R8G8B8A8_UNORM,
            SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
            Usage: D3D11_USAGE_DEFAULT,
            BindFlags: (D3D11_BIND_RENDER_TARGET.0 | D3D11_BIND_SHADER_RESOURCE.0) as u32,
            CPUAccessFlags: 0,
            MiscFlags: 0,
        };
        let mut texture = None;
        device.CreateTexture2D(&tex_desc, None, Some(&mut texture)).ok()?;
        let texture = texture?;
        let mut rtv = None;
        device.CreateRenderTargetView(&texture, None, Some(&mut rtv)).ok()?;
        let rtv = rtv?;
        let mut srv = None;
        device.CreateShaderResourceView(&texture, None, Some(&mut srv)).ok()?;
        let srv = srv?;
        let depth_desc = D3D11_TEXTURE2D_DESC {
            Width: w,
            Height: h,
            MipLevels: 1,
            ArraySize: 1,
            Format: DXGI_FORMAT_D24_UNORM_S8_UINT,
            SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
            Usage: D3D11_USAGE_DEFAULT,
            BindFlags: D3D11_BIND_DEPTH_STENCIL.0 as u32,
            CPUAccessFlags: 0,
            MiscFlags: 0,
        };
        let mut depth = None;
        device.CreateTexture2D(&depth_desc, None, Some(&mut depth)).ok()?;
        let depth = depth?;
        let mut dsv = None;
        device.CreateDepthStencilView(&depth, None, Some(&mut dsv)).ok()?;
        let dsv = dsv?;
        self.target = Some(Target { texture, rtv, srv, depth, dsv, w, h });
        self.cache_key = None;
        self.cached_srv = None;
        Some(())
    }

    /// Renders the selected agent into the offscreen target and returns its SRV.
    ///
    /// The pose is static, so repeated frames with the same model/size reuse the
    /// cached texture. Bone projections are still refreshed for a moved rect.
    pub unsafe fn render(
        &mut self,
        context: &ID3D11DeviceContext,
        rect: (f32, f32, f32, f32),
        clock: f32,
        enemy: bool,
        material: MaterialOverride,
    ) -> Option<ID3D11ShaderResourceView> {
        let ss = 2.0f32;
        let w = (rect.2 * ss).max(64.0) as u32;
        let h = (rect.3 * ss).max(96.0) as u32;
        if self.ensure_target(context, w, h).is_none() {
            self.last_ok = false;
            return None;
        }
        let agent = if enemy { self.phoenix.as_ref() } else { self.sas.as_ref() }?;
        let aspect = w as f32 / h as f32;
        let proj = Mat4::perspective(CAMERA_FOV, aspect, 0.05, 40.0);
        let (eye, focus) = agent.data.camera(aspect);
        let view = Mat4::look_at(eye, focus);
        let view_proj = Mat4::mul(proj, view);
        let anim_key = if USE_ANIMATION { (clock * 20.0) as u32 } else { 0 };
        let material_key = if material.enabled {
            (material.material as u32, material.packed_color())
        } else {
            (0, 0)
        };
        let key = (w, h, enemy, anim_key, material_key.0, material_key.1);

        if self.cache_key == Some(key) && self.cached_srv.is_some() && self.last_ok {
            let projected = agent.data.joint_positions(clock, view_proj, rect);
            self.last_projected = projected;
            *PROJECTED.lock() = projected;
            return self.cached_srv.clone();
        }

        let target = self.target.as_ref()?;
        let bones = agent.data.bones(clock);
        context.UpdateSubresource(
            &agent.bone_buffer,
            0,
            None,
            bones.as_ptr() as *const c_void,
            (MAX_BONES * 64) as u32,
            0,
        );
        let base_tint = if enemy { [1.04, 0.98, 0.94, 1.0] } else { [0.94, 1.0, 1.06, 1.0] };
        let tint = if material.enabled {
            let boost = if material.flat() { 1.35 } else { 1.0 };
            [
                material.color[0] as f32 / 255.0 * boost,
                material.color[1] as f32 / 255.0 * boost,
                material.color[2] as f32 / 255.0 * boost,
                1.0,
            ]
        } else {
            base_tint
        };
        let scene = SceneBuffer {
            view_proj: view_proj.0,
            world: agent.data.skeleton.world.0,
            light: [0.46, 0.72, 0.52, 0.0],
            camera: [eye[0], eye[1], eye[2], 1.0],
            tint,
        };
        context.UpdateSubresource(
            &self.scene_buffer,
            0,
            None,
            &scene as *const SceneBuffer as *const c_void,
            176,
            0,
        );

        let clear = D3D11_VIEWPORT { TopLeftX: 0.0, TopLeftY: 0.0, Width: w as f32, Height: h as f32, MinDepth: 0.0, MaxDepth: 1.0 };
        context.RSSetViewports(Some(&[clear]));
        let targets = [Some(target.rtv.clone())];
        context.OMSetRenderTargets(Some(&targets), Some(&target.dsv));
        context.ClearRenderTargetView(&target.rtv, &[0.0, 0.0, 0.0, 0.0]);
        context.ClearDepthStencilView(&target.dsv, D3D11_CLEAR_DEPTH.0, 1.0, 0);
        if material.enabled && material.ignore_z() {
            context.OMSetDepthStencilState(&self.depth_ignore_z, 0);
        } else {
            context.OMSetDepthStencilState(&self.depth_state, 0);
        }
        context.OMSetBlendState(&self.blend, Some(&[0.0; 4]), u32::MAX);
        if material.wireframe() {
            context.RSSetState(&self.rasterizer_wireframe);
        } else {
            context.RSSetState(&self.rasterizer);
        }
        context.IASetInputLayout(&self.input_layout);
        context.VSSetShader(&self.vertex_shader, None);
        context.PSSetShader(&self.pixel_shader, None);
        let constant_buffers = [Some(self.scene_buffer.clone()), Some(agent.bone_buffer.clone())];
        context.VSSetConstantBuffers(0, Some(&constant_buffers));
        context.PSSetConstantBuffers(0, Some(&constant_buffers[..1]));
        let samplers = [Some(self.sampler.clone())];
        context.PSSetSamplers(0, Some(&samplers));

        let stride = std::mem::size_of::<ModelVertex>() as u32;
        let offset = 0u32;
        let vertex_buffers = [Some(agent.vertex_buffer.clone())];
        context.IASetVertexBuffers(0, 1, Some(vertex_buffers.as_ptr()), Some(&stride), Some(&offset));
        context.IASetIndexBuffer(&agent.index_buffer, DXGI_FORMAT_R32_UINT, 0);
        context.IASetPrimitiveTopology(D3D11_PRIMITIVE_TOPOLOGY_TRIANGLELIST);
        for primitive in &agent.primitives {
            let srv = if material.flat() {
                self.white.clone()
            } else {
                agent
                    .textures
                    .get(primitive.texture)
                    .cloned()
                    .unwrap_or_else(|| self.white.clone())
            };
            let views = [Some(srv)];
            context.PSSetShaderResources(0, Some(&views));
            context.DrawIndexed(primitive.index_count, primitive.index_offset, 0);
        }

        let srv = target.srv.clone();
        let projected = agent.data.joint_positions(clock, view_proj, rect);
        self.last_projected = projected;
        *PROJECTED.lock() = projected;
        self.cache_key = Some(key);
        self.cached_srv = Some(srv.clone());
        self.last_ok = true;
        Some(srv)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn material_override_matches_cham_combo() {
        let flat = MaterialOverride { enabled: true, material: 3, color: [173, 192, 255, 150] };
        assert!(flat.flat());
        assert!(!flat.wireframe());
        assert!(!flat.ignore_z());
        assert_eq!(flat.packed_color(), u32::from_le_bytes(flat.color));

        let overlay_wire = MaterialOverride { enabled: true, material: 19, color: [255, 0, 0, 255] };
        assert!(overlay_wire.flat());
        assert!(overlay_wire.wireframe());
        assert!(!overlay_wire.ignore_z());

        let ignore_z = MaterialOverride { enabled: true, material: 11, color: [0, 0, 0, 0] };
        assert!(ignore_z.ignore_z());
        assert!(!MaterialOverride::empty().flat());
    }

    #[test]
    fn both_agents_parse_with_required_bones() {
        const REQUIRED: &[&str] = &[
            "pelvis",
            "spine_1",
            "spine_2",
            "spine_3",
            "neck_0",
            "head_0",
            "arm_upper_L",
            "arm_upper_R",
            "arm_lower_L",
            "arm_lower_R",
            "hand_L",
            "hand_R",
            "leg_upper_L",
            "leg_upper_R",
            "leg_lower_L",
            "leg_lower_R",
            "ankle_L",
            "ankle_R",
        ];
        for glb in [PHOENIX_GLB, SAS_GLB] {
            let data = parse_agent(glb).expect("GLB must parse");
            assert!(!data.mesh.vertices.is_empty(), "no vertices");
            assert!(!data.mesh.indices.is_empty(), "no indices");
            assert!(!data.mesh.primitives.is_empty(), "no primitives");
            assert!(!data.mesh.textures.is_empty(), "no albedo textures");
            for name in REQUIRED {
                assert!(data.skeleton.bone_slots.contains_key(*name), "missing bone {name}");
            }
            let bones = data.bones(0.0);
            assert!(bones.iter().any(|m| *m != Mat4::identity().0), "no skin matrices");
            assert!(data.clip.duration > 0.0, "idle clip has no duration");
        }
    }

    #[test]
    fn mesh_projection_fits_preview_rect() {
        let rect = (0.0, 0.0, 296.0, 438.0);
        let aspect = rect.2 / rect.3;
        for glb in [PHOENIX_GLB, SAS_GLB] {
            let data = parse_agent(glb).expect("GLB must parse");
            let (eye, focus) = data.camera(aspect);
            let proj = Mat4::perspective(CAMERA_FOV, aspect, 0.05, 40.0);
            let view_proj = Mat4::mul(proj, Mat4::look_at(eye, focus));
            let matrices = data.bones(0.0);
            let mut min = [f32::INFINITY; 2];
            let mut max = [f32::NEG_INFINITY; 2];
            for v in &data.mesh.vertices {
                let world = data.world_vertex(v, &matrices);
                let clip = view_proj.transform(world);
                if clip[3] <= 0.0 {
                    continue;
                }
                let x = rect.0 + (clip[0] / clip[3] * 0.5 + 0.5) * rect.2;
                let y = rect.1 + (1.0 - (clip[1] / clip[3] * 0.5 + 0.5)) * rect.3;
                min[0] = min[0].min(x);
                min[1] = min[1].min(y);
                max[0] = max[0].max(x);
                max[1] = max[1].max(y);
            }
            eprintln!("mesh bounds: min={min:?} max={max:?}");
            assert!(min[0] > rect.0, "mesh touches left edge");
            assert!(max[0] < rect.0 + rect.2, "mesh touches right edge");
            assert!(min[1] > rect.1, "mesh touches top edge");
            assert!(max[1] < rect.1 + rect.3, "mesh touches bottom edge");
        }
    }

    #[test]
    fn projected_bones_fall_inside_preview_rect() {
        let rect = (12.0, 20.0, 296.0, 378.0);
        let aspect = rect.2 / rect.3;
        for glb in [PHOENIX_GLB, SAS_GLB] {
            let data = parse_agent(glb).expect("GLB must parse");
            let (eye, focus) = data.camera(aspect);
            let proj = Mat4::perspective(CAMERA_FOV, aspect, 0.05, 40.0);
            let view_proj = Mat4::mul(proj, Mat4::look_at(eye, focus));
            let points = data.joint_positions(0.0, view_proj, rect);
            let inside = points
                .iter()
                .flatten()
                .filter(|(x, y)| {
                    x.is_finite()
                        && y.is_finite()
                        && *x >= rect.0
                        && *x <= rect.0 + rect.2
                        && *y >= rect.1
                        && *y <= rect.1 + rect.3
                })
                .count();
            assert!(inside >= 8, "only {inside} bones inside preview rect");
        }
    }
}
