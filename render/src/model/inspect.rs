// Config: owned D3D11 weapon scene; no game offsets, pointers or engine calls.
use super::*;
use crate::skin_inspect::Request;
use parking_lot::Condvar;
use std::sync::Arc;
const MAX_GPU_MODELS: usize = 4;
const SUPERSAMPLE: f32 = 1.5;
const MAX_TARGET_SIZE: u32 = 2048;
const MAX_MODEL_VERTICES: usize = 250_000;
const MAX_MODEL_INDICES: usize = 750_000;
const MAX_TEXTURE_BYTES: usize = 32 * 1024 * 1024;
const PRESENTATION_YAW: f32 = -12.0;
const PRESENTATION_PITCH: f32 = -6.0;
const WEAPON_PS: &[u8] = include_bytes!("../shaders/weapon_ps.cso");
type Key = (i16, bool, u32);
#[derive(Default)]
struct Queue {
    wanted: Option<Key>,
    loading: Option<Key>,
    ready: Option<(Key, Result<AgentData, String>)>,
    stop: bool,
}
struct Shared {
    queue: Mutex<Queue>,
    signal: Condvar,
}
pub struct WeaponRenderer {
    pipeline: ModelRenderer,
    transparent_blend: ID3D11BlendState,
    transparent_depth: ID3D11DepthStencilState,
    shared: Arc<Shared>,
    worker: Option<wallbang_core::threading::ThreadHandle>,
    models: HashMap<Key, (GpuAgent, u64)>,
    failed: HashMap<Key, String>,
    clock: u64,
    last: Option<(Key, u32, u32, u32, u32, u32)>,
}
struct Reader<'a> {
    bytes: &'a [u8],
    position: usize,
}
impl<'a> Reader<'a> {
    fn take(&mut self, n: usize) -> Result<&'a [u8], String> {
        let end = self.position.checked_add(n).ok_or("asset overflow")?;
        let b = self
            .bytes
            .get(self.position..end)
            .ok_or("truncated weapon asset")?;
        self.position = end;
        Ok(b)
    }
    fn u32(&mut self) -> Result<u32, String> {
        Ok(u32::from_le_bytes(self.take(4)?.try_into().unwrap()))
    }
    fn float(&mut self) -> Result<f32, String> {
        let f = f32::from_le_bytes(self.take(4)?.try_into().unwrap());
        if f.is_finite() {
            Ok(f)
        } else {
            Err("non-finite asset coordinate".into())
        }
    }
}
fn decode(bytes: &[u8]) -> Result<AgentData, String> {
    let mut r = Reader { bytes, position: 0 };
    if r.take(4)? != b"WBI2" {
        return Err("invalid weapon format".into());
    }
    let (nv, ni, np, nt) = (
        r.u32()? as usize,
        r.u32()? as usize,
        r.u32()? as usize,
        r.u32()? as usize,
    );
    if nv == 0
        || nv > MAX_MODEL_VERTICES
        || ni == 0
        || ni > MAX_MODEL_INDICES
        || np == 0
        || np > 128
        || nt == 0
        || nt > 128
    {
        return Err("invalid weapon counts".into());
    }
    let mut vertices = Vec::with_capacity(nv);
    let mut bounds = Bounds {
        min: [f32::INFINITY; 3],
        max: [f32::NEG_INFINITY; 3],
    };
    for _ in 0..nv {
        let pos = [r.float()?, r.float()?, r.float()?];
        let normal = [r.float()?, r.float()?, r.float()?];
        let uv = [r.float()?, r.float()?];
        for i in 0..3 {
            bounds.min[i] = bounds.min[i].min(pos[i]);
            bounds.max[i] = bounds.max[i].max(pos[i]);
        }
        vertices.push(ModelVertex {
            pos,
            normal,
            uv,
            joints: [0; 4],
            weights: [1., 0., 0., 0.],
        });
    }
    let mut indices = Vec::with_capacity(ni);
    for _ in 0..ni {
        let i = r.u32()?;
        if i as usize >= nv {
            return Err("weapon index out of bounds".into());
        }
        indices.push(i);
    }
    let mut primitives = Vec::with_capacity(np);
    for _ in 0..np {
        let (first, count, texture) = (r.u32()?, r.u32()?, r.u32()? as usize);
        let flags = r.u32()?;
        let paintable = flags & 1 != 0;
        let alpha = flags & 2 != 0;
        if first as usize + count as usize > ni || count % 3 != 0 || texture >= nt {
            return Err("invalid weapon primitive".into());
        }
        primitives.push(Primitive {
            index_offset: first,
            index_count: count,
            texture,
            paintable,
            alpha,
        });
    }
    let mut textures = Vec::with_capacity(nt);
    let mut texture_bytes = 0usize;
    for _ in 0..nt {
        let size = r.u32()? as usize;
        let image = image::load_from_memory_with_format(r.take(size)?, image::ImageFormat::Png)
            .map_err(|e| e.to_string())?
            .to_rgba8();
        let (width, height) = image.dimensions();
        if width == 0 || height == 0 || width > 2048 || height > 2048 {
            return Err("invalid texture dimensions".into());
        }
        texture_bytes += image.len();
        if texture_bytes > MAX_TEXTURE_BYTES {
            return Err("weapon texture budget exceeded".into());
        }
        textures.push(TextureData {
            rgba: image.into_raw(),
            width,
            height,
        });
    }
    if r.position != bytes.len() {
        return Err("trailing weapon asset bytes".into());
    }
    let center = bounds.center();
    let longest = (0..3)
        .map(|i| bounds.max[i] - bounds.min[i])
        .fold(0., f32::max);
    if longest < 1e-6 {
        return Err("empty weapon bounds".into());
    }
    // Face the broad side toward the camera for guns AND knives. Mapping just
    // their long axis put thin knife blades edge-on. Keep a right-handed basis.
    let mut axes = [0usize, 1, 2];
    axes.sort_by(|&a, &b| {
        (bounds.max[b] - bounds.min[b]).total_cmp(&(bounds.max[a] - bounds.min[a]))
    });
    let inversions = (0..3)
        .flat_map(|a| (a + 1..3).map(move |b| (a, b)))
        .filter(|&(a, b)| axes[a] > axes[b])
        .count();
    let parity = if inversions % 2 == 0 { 1.0 } else { -1.0 };
    let primary_sign = if axes[0] == 2 { -1.0 } else { 1.0 };
    let orient = |v: [f32; 3]| {
        [
            v[axes[0]] * primary_sign,
            v[axes[1]],
            v[axes[2]] * primary_sign * parity,
        ]
    };
    let scale = 2.2 / longest;
    bounds = Bounds {
        min: [f32::INFINITY; 3],
        max: [f32::NEG_INFINITY; 3],
    };
    for vertex in &mut vertices {
        let p = orient(std::array::from_fn(|i| (vertex.pos[i] - center[i]) * scale));
        vertex.pos = p;
        vertex.normal = orient(vertex.normal);
        for i in 0..3 {
            bounds.min[i] = bounds.min[i].min(p[i]);
            bounds.max[i] = bounds.max[i].max(p[i]);
        }
    }
    Ok(AgentData {
        mesh: Mesh {
            vertices,
            indices,
            primitives,
            textures,
        },
        skeleton: Skeleton {
            nodes: Vec::new(),
            joints: Vec::new(),
            inverse_bind: Vec::new(),
            bone_slots: HashMap::default(),
            world: Mat4::identity(),
        },
        clip: Clip {
            tracks: Vec::new(),
            duration: 0.,
        },
        bounds,
        forward: [0., 0., 1.],
    })
}
impl WeaponRenderer {
    pub unsafe fn new(device: &ID3D11Device) -> Option<Self> {
        let mut pipeline = ModelRenderer::new_pipeline(device, false)?;
        let mut ps = None;
        device
            .CreatePixelShader(WEAPON_PS, None, Some(&mut ps))
            .ok()?;
        pipeline.pixel_shader = ps?;
        let mut blend_desc = D3D11_BLEND_DESC::default();
        pipeline.blend.GetDesc(&mut blend_desc);
        blend_desc.RenderTarget[0].BlendEnable = true.into();
        blend_desc.RenderTarget[0].SrcBlend = D3D11_BLEND_SRC_ALPHA;
        blend_desc.RenderTarget[0].DestBlend = D3D11_BLEND_INV_SRC_ALPHA;
        blend_desc.RenderTarget[0].SrcBlendAlpha = D3D11_BLEND_ONE;
        blend_desc.RenderTarget[0].DestBlendAlpha = D3D11_BLEND_INV_SRC_ALPHA;
        let mut transparent_blend = None;
        device
            .CreateBlendState(&blend_desc, Some(&mut transparent_blend))
            .ok()?;
        let mut depth_desc = D3D11_DEPTH_STENCIL_DESC::default();
        pipeline.depth_state.GetDesc(&mut depth_desc);
        depth_desc.DepthWriteMask = D3D11_DEPTH_WRITE_MASK_ZERO;
        let mut transparent_depth = None;
        device
            .CreateDepthStencilState(&depth_desc, Some(&mut transparent_depth))
            .ok()?;
        let shared = Arc::new(Shared {
            queue: Mutex::new(Queue::default()),
            signal: Condvar::new(),
        });
        let task = shared.clone();
        let worker = wallbang_core::threading::spawn(move || loop {
            let key = {
                let mut q = task.queue.lock();
                while !q.stop && q.wanted.is_none() {
                    task.signal.wait(&mut q);
                }
                if q.stop {
                    break;
                }
                let k = q.wanted.take().unwrap();
                q.loading = Some(k);
                k
            };
            let result = wallbang_resources::weapon_models::model(key.0, key.1)
                .ok_or_else(|| "Weapon model not imported".to_owned())
                .and_then(decode)
                .and_then(|mut data| {
                    if key.2 != 0 {
                        let png = wallbang_resources::weapon_finishes::texture(key.2)
                            .ok_or("Finish texture unavailable")?;
                        let image = image::load_from_memory(png)
                            .map_err(|e| e.to_string())?
                            .to_rgba8();
                        let (width, height) = image.dimensions();
                        let texture = data.mesh.textures.len();
                        data.mesh.textures.push(TextureData {
                            width,
                            height,
                            rgba: image.into_raw(),
                        });
                        for primitive in &mut data.mesh.primitives {
                            if primitive.paintable {
                                primitive.texture = texture;
                            }
                        }
                    }
                    // Keep only referenced images: replaced substrate textures
                    // must not consume CPU/GPU memory alongside the finish.
                    let mut source: Vec<_> = data.mesh.textures.drain(..).map(Some).collect();
                    let mut remap = HashMap::default();
                    for primitive in &mut data.mesh.primitives {
                        let index = *remap.entry(primitive.texture).or_insert_with(|| {
                            let index = data.mesh.textures.len();
                            data.mesh
                                .textures
                                .push(source[primitive.texture].take().unwrap());
                            index
                        });
                        primitive.texture = index;
                    }
                    Ok(data)
                });
            let mut q = task.queue.lock();
            if q.stop {
                break;
            }
            q.loading = None;
            q.ready = Some((key, result));
        });
        Some(Self {
            pipeline,
            transparent_blend: transparent_blend?,
            transparent_depth: transparent_depth?,
            shared,
            worker: Some(worker),
            models: HashMap::default(),
            failed: HashMap::default(),
            clock: 0,
            last: None,
        })
    }
    pub unsafe fn render(
        &mut self,
        device: &ID3D11Device,
        context: &ID3D11DeviceContext,
        request: &Request,
    ) -> Option<ID3D11ShaderResourceView> {
        if !request
            .rect
            .iter()
            .chain([&request.yaw, &request.pitch, &request.zoom])
            .all(|v| v.is_finite())
            || request.rect[2] <= 0.0
            || request.rect[3] <= 0.0
        {
            crate::skin_inspect::report_error("Invalid preview viewport".into());
            return None;
        }
        self.clock = self.clock.wrapping_add(1);
        let legacy =
            wallbang_config::skin_catalog::paint_kit(request.paint).is_some_and(|p| p.legacy_model);
        let paint = if wallbang_config::skin_catalog::compatible(request.definition, request.paint)
            && wallbang_resources::weapon_finishes::texture(request.paint).is_some()
        {
            request.paint
        } else {
            0
        };
        let key = (request.definition, legacy, paint);
        let ready = self.shared.queue.lock().ready.take();
        if let Some((k, result)) = ready.filter(|(k, _)| *k == key) {
            match result.and_then(|data| {
                ModelRenderer::upload(device, data).ok_or_else(|| "DX11 model upload failed".into())
            }) {
                Ok(mut gpu) => {
                    gpu.data.mesh.textures.clear();
                    gpu.data.mesh.vertices = Vec::new();
                    gpu.data.mesh.indices = Vec::new();
                    if !self.models.contains_key(&k) && self.models.len() >= MAX_GPU_MODELS {
                        if let Some(old) =
                            self.models.iter().min_by_key(|(_, v)| v.1).map(|(k, _)| *k)
                        {
                            self.models.remove(&old);
                        }
                    }
                    self.models.insert(k, (gpu, self.clock));
                    self.last = None;
                }
                Err(error) => {
                    crate::trace(&format!("DX11 inspect {:?}: {error}", k));
                    self.failed.insert(k, error);
                }
            }
        }
        if !self.models.contains_key(&key) {
            if let Some(error) = self.failed.get(&key) {
                crate::skin_inspect::report_error(error.clone());
                return None;
            }
            if self.worker.as_ref().is_some_and(|w| w.started()) {
                let mut q = self.shared.queue.lock();
                if q.loading != Some(key) && q.ready.as_ref().is_none_or(|v| v.0 != key) {
                    q.wanted = Some(key);
                    self.shared.signal.notify_one();
                }
            } else {
                crate::skin_inspect::report_error("Model worker unavailable".into());
            }
            return None;
        }
        let w = (request.rect[2] * SUPERSAMPLE).clamp(64., MAX_TARGET_SIZE as f32) as u32;
        let h = (request.rect[3] * SUPERSAMPLE).clamp(64., MAX_TARGET_SIZE as f32) as u32;
        self.pipeline.ensure_target(context, w, h)?;
        let cache = (
            key,
            w,
            h,
            request.yaw.to_bits(),
            request.pitch.to_bits(),
            request.zoom.to_bits(),
        );
        if self.last == Some(cache) {
            self.models.get_mut(&key)?.1 = self.clock;
            return Some(self.pipeline.target.as_ref()?.srv.clone());
        }
        let (model, last) = self.models.get_mut(&key)?;
        *last = self.clock;
        let aspect = w as f32 / h as f32;
        let yaw = Mat4::rotate_y((request.yaw + PRESENTATION_YAW).to_radians());
        let pitch = Mat4::trs(
            [0.; 3],
            [
                ((request.pitch + PRESENTATION_PITCH).to_radians() * 0.5).sin(),
                0.,
                0.,
                ((request.pitch + PRESENTATION_PITCH).to_radians() * 0.5).cos(),
            ],
            [1.; 3],
        );
        let world = Mat4::mul(pitch, yaw);
        let tan_v = (CAMERA_FOV * 0.5).tan();
        let tan_h = tan_v * aspect;
        let distance = model
            .data
            .bounds
            .corners()
            .iter()
            .map(|p| {
                let p = world.transform(*p);
                p[2] + (p[0].abs() / tan_h).max(p[1].abs() / tan_v)
            })
            .fold(0., f32::max)
            * 1.15
            / request.zoom.clamp(0.65, 1.8);
        let mut scene = SceneBuffer {
            view_proj: Mat4::mul(
                Mat4::perspective(CAMERA_FOV, aspect, 0.01, 50.),
                Mat4::look_at([0., 0., distance], [0.; 3]),
            )
            .0,
            world: world.0,
            light: [-0.4, 0.7, 0.65, 0.],
            camera: [0., 0., distance, 1.],
            tint: [1.; 4],
        };
        context.UpdateSubresource(
            &self.pipeline.scene_buffer,
            0,
            None,
            (&scene as *const SceneBuffer).cast(),
            0,
            0,
        );
        let bones = [Mat4::identity().0; MAX_BONES];
        context.UpdateSubresource(&model.bone_buffer, 0, None, bones.as_ptr().cast(), 0, 0);
        let p = &self.pipeline;
        let target = p.target.as_ref()?;
        // Unbind the previous preview SRV before rebinding this texture as an RTV.
        context.PSSetShaderResources(0, Some(&[None]));
        context.OMSetRenderTargets(Some(&[Some(target.rtv.clone())]), Some(&target.dsv));
        context.ClearRenderTargetView(&target.rtv, &[0., 0., 0., 0.]);
        context.ClearDepthStencilView(&target.dsv, D3D11_CLEAR_DEPTH.0, 1., 0);
        context.RSSetViewports(Some(&[D3D11_VIEWPORT {
            TopLeftX: 0.,
            TopLeftY: 0.,
            Width: w as f32,
            Height: h as f32,
            MinDepth: 0.,
            MaxDepth: 1.,
        }]));
        context.RSSetState(&p.rasterizer);
        context.OMSetDepthStencilState(&p.depth_state, 0);
        context.OMSetBlendState(&p.blend, Some(&[0.; 4]), u32::MAX);
        context.IASetInputLayout(&p.input_layout);
        context.IASetPrimitiveTopology(D3D11_PRIMITIVE_TOPOLOGY_TRIANGLELIST);
        context.VSSetShader(&p.vertex_shader, None);
        context.PSSetShader(&p.pixel_shader, None);
        context.VSSetConstantBuffers(
            0,
            Some(&[
                Some(p.scene_buffer.clone()),
                Some(model.bone_buffer.clone()),
            ]),
        );
        context.PSSetConstantBuffers(0, Some(&[Some(p.scene_buffer.clone())]));
        context.PSSetSamplers(0, Some(&[Some(p.sampler.clone())]));
        let stride = std::mem::size_of::<ModelVertex>() as u32;
        let buffers = [Some(model.vertex_buffer.clone())];
        context.IASetVertexBuffers(0, 1, Some(buffers.as_ptr()), Some(&stride), Some(&0));
        context.IASetIndexBuffer(&model.index_buffer, DXGI_FORMAT_R32_UINT, 0);
        // Transparent stickers/lenses follow the opaque geometry, retain their
        // source alpha, and never write depth. Opaque texture alpha is data.
        for alpha in [false, true] {
            if !model
                .primitives
                .iter()
                .any(|primitive| primitive.alpha == alpha)
            {
                continue;
            }
            if alpha {
                scene.tint[3] = 0.0;
                context.UpdateSubresource(
                    &p.scene_buffer,
                    0,
                    None,
                    (&scene as *const SceneBuffer).cast(),
                    0,
                    0,
                );
                context.OMSetBlendState(&self.transparent_blend, Some(&[0.; 4]), u32::MAX);
                context.OMSetDepthStencilState(&self.transparent_depth, 0);
            }
            for primitive in model
                .primitives
                .iter()
                .filter(|primitive| primitive.alpha == alpha)
            {
                context.PSSetShaderResources(
                    0,
                    Some(&[Some(model.textures.get(primitive.texture)?.clone())]),
                );
                context.DrawIndexed(primitive.index_count, primitive.index_offset, 0);
            }
        }
        self.last = Some(cache);
        Some(target.srv.clone())
    }
}
impl Drop for WeaponRenderer {
    fn drop(&mut self) {
        {
            let mut q = self.shared.queue.lock();
            q.stop = true;
            q.wanted = None;
        }
        self.shared.signal.notify_all();
        if let Some(worker) = self.worker.take() {
            worker.join();
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn imported_models_have_real_geometry_and_bounded_textures() {
        for &(definition, legacy, _, _) in wallbang_resources::weapon_models::INDEX {
            let data =
                decode(wallbang_resources::weapon_models::model(definition, legacy).unwrap())
                    .unwrap();
            assert!(!data.mesh.vertices.is_empty());
            assert!(!data.mesh.indices.is_empty());
            assert!(data
                .bounds
                .min
                .iter()
                .chain(&data.bounds.max)
                .all(|v| v.is_finite() && v.abs() < 1.2));
        }
    }
    #[test]
    fn corrupt_assets_fail_without_panicking() {
        for bytes in [b"".as_slice(), b"WBI1", b"not a weapon"] {
            assert!(decode(bytes).is_err());
        }
    }
    #[test]
    fn transparent_lenses_and_sticker_gaps_keep_their_own_material() {
        for definition in [7, 8, 39] {
            let data = decode(wallbang_resources::weapon_models::model(definition, false).unwrap())
                .unwrap();
            assert!(data.mesh.primitives.iter().any(|p| p.alpha));
            assert!(data
                .mesh
                .primitives
                .iter()
                .all(|p| !p.alpha || !p.paintable));
        }
    }
}
