// Fullscreen D3D11 Post-Processing pass (NVIDIA-style Color Grading, Nightmode, Vignette, Sharpen).
use core::ffi::c_void;
use parking_lot::Mutex;
use wallbang_config::settings::PostProcessSettings;
use windows::Win32::Graphics::Direct3D::D3D11_PRIMITIVE_TOPOLOGY_TRIANGLELIST;
use windows::Win32::Graphics::Direct3D11::*;
use windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT;

const POSTPROCESS_VS_BC: &[u8] = include_bytes!("shaders/postprocess_vs.cso");
const POSTPROCESS_PS_BC: &[u8] = include_bytes!("shaders/postprocess_ps.cso");

#[repr(C)]
#[derive(Clone, Copy)]
struct Params {
    p0: [f32; 4], // exposure, contrast, saturation, vibrance
    p1: [f32; 4], // temperature, tint, gamma, nightmode
    p2: [f32; 4], // vignette, sharpen, inv_w, inv_h
    p3: [f32; 4], // world_tint rgb + strength
    p4: [f32; 4], // camera blur UV xy, radial velocity blur, enabled
}

struct PostProcessState {
    timing: GpuTiming,
    vs: ID3D11VertexShader,
    ps: ID3D11PixelShader,
    cb: ID3D11Buffer,
    sampler: ID3D11SamplerState,
    rasterizer: ID3D11RasterizerState,
    copy_tex: Option<ID3D11Texture2D>,
    copy_srv: Option<ID3D11ShaderResourceView>,
    width: u32,
    height: u32,
    format: DXGI_FORMAT,
    camera: Option<(f32, f32, std::time::Instant)>,
}

unsafe impl Send for PostProcessState {}

static STATE: Mutex<Option<PostProcessState>> = Mutex::new(None);

struct GpuSample { disjoint: ID3D11Query, start: ID3D11Query, end: ID3D11Query, pending: bool }
struct GpuTiming { samples: Vec<GpuSample>, cursor: usize, count: u32, total: f64, peak: f64 }
impl GpuTiming {
    unsafe fn new(device: &ID3D11Device) -> Self {
        let query = |kind| {
            let mut result = None;
            device.CreateQuery(&D3D11_QUERY_DESC { Query: kind, MiscFlags: 0 }, Some(&mut result)).ok()?;
            result
        };
        let mut samples = Vec::new();
        for _ in 0..4 {
            if let (Some(disjoint),Some(start),Some(end)) = (query(D3D11_QUERY_TIMESTAMP_DISJOINT),query(D3D11_QUERY_TIMESTAMP),query(D3D11_QUERY_TIMESTAMP)) {
                samples.push(GpuSample {disjoint,start,end,pending:false});
            }
        }
        Self { samples, cursor: 0, count: 0, total: 0.0, peak: 0.0 }
    }
    unsafe fn begin(&mut self, context: &ID3D11DeviceContext) -> Option<usize> {
        if self.samples.is_empty() { return None; }
        let index = self.cursor;
        self.cursor = (self.cursor + 1) % self.samples.len();
        let sample = &mut self.samples[index];
        if sample.pending {
            let mut disjoint = D3D11_QUERY_DATA_TIMESTAMP_DISJOINT::default();
            let mut start = 0u64;
            let mut end = 0u64;
            let _ = context.GetData(&sample.disjoint, Some((&mut disjoint as *mut D3D11_QUERY_DATA_TIMESTAMP_DISJOINT).cast()), std::mem::size_of_val(&disjoint) as u32, D3D11_ASYNC_GETDATA_DONOTFLUSH.0 as u32);
            let _ = context.GetData(&sample.start, Some((&mut start as *mut u64).cast()), 8, D3D11_ASYNC_GETDATA_DONOTFLUSH.0 as u32);
            let _ = context.GetData(&sample.end, Some((&mut end as *mut u64).cast()), 8, D3D11_ASYNC_GETDATA_DONOTFLUSH.0 as u32);
            if disjoint.Frequency == 0 || start == 0 || end == 0 { return None; }
            if !disjoint.Disjoint.as_bool() && end >= start {
                let ms = (end-start) as f64 * 1000.0 / disjoint.Frequency as f64;
                self.count += 1; self.total += ms; self.peak = self.peak.max(ms);
                if self.count >= 120 {
                    crate::trace(&format!("postprocess GPU: mean={:.3}ms peak={:.3}ms (nonblocking timestamps)",self.total/self.count as f64,self.peak));
                    self.count = 0; self.total = 0.0; self.peak = 0.0;
                }
            }
            sample.pending = false;
        }
        context.Begin(&sample.disjoint);
        context.End(&sample.start);
        Some(index)
    }
    unsafe fn end(&mut self, context: &ID3D11DeviceContext, index: usize) {
        let sample = &mut self.samples[index];
        context.End(&sample.end); context.End(&sample.disjoint); sample.pending = true;
    }
}

impl PostProcessState {
    unsafe fn create(device: &ID3D11Device) -> Option<Self> {
        let mut vs = None;
        device.CreateVertexShader(POSTPROCESS_VS_BC, None, Some(&mut vs)).ok()?;
        let mut ps = None;
        device.CreatePixelShader(POSTPROCESS_PS_BC, None, Some(&mut ps)).ok()?;

        let cb_desc = D3D11_BUFFER_DESC {
            ByteWidth: core::mem::size_of::<Params>() as u32,
            Usage: D3D11_USAGE_DEFAULT,
            BindFlags: D3D11_BIND_CONSTANT_BUFFER.0 as u32,
            ..Default::default()
        };
        let mut cb = None;
        device.CreateBuffer(&cb_desc, None, Some(&mut cb)).ok()?;

        let samp_desc = D3D11_SAMPLER_DESC {
            Filter: D3D11_FILTER_MIN_MAG_MIP_LINEAR,
            AddressU: D3D11_TEXTURE_ADDRESS_CLAMP,
            AddressV: D3D11_TEXTURE_ADDRESS_CLAMP,
            AddressW: D3D11_TEXTURE_ADDRESS_CLAMP,
            MaxLOD: f32::MAX,
            ..Default::default()
        };
        let mut sampler = None;
        device.CreateSamplerState(&samp_desc, Some(&mut sampler)).ok()?;

        let rs_desc = D3D11_RASTERIZER_DESC {
            FillMode: D3D11_FILL_SOLID,
            CullMode: D3D11_CULL_NONE,
            DepthClipEnable: false.into(),
            ..Default::default()
        };
        let mut rasterizer = None;
        device.CreateRasterizerState(&rs_desc, Some(&mut rasterizer)).ok()?;

        Some(Self {
            timing: GpuTiming::new(device),
            vs: vs?,
            ps: ps?,
            cb: cb?,
            sampler: sampler?,
            rasterizer: rasterizer?,
            copy_tex: None,
            copy_srv: None,
            width: 0,
            height: 0,
            format: DXGI_FORMAT(0),
            camera: None,
        })
    }

    unsafe fn ensure_copy(&mut self, device: &ID3D11Device, backbuffer: &ID3D11Texture2D) -> bool {
        let mut desc = D3D11_TEXTURE2D_DESC::default();
        backbuffer.GetDesc(&mut desc);
        if self.copy_tex.is_some()
            && self.width == desc.Width
            && self.height == desc.Height
            && self.format == desc.Format
        {
            return true;
        }
        self.copy_tex = None;
        self.camera = None;
        self.copy_srv = None;
        desc.BindFlags = D3D11_BIND_SHADER_RESOURCE.0 as u32;
        desc.Usage = D3D11_USAGE_DEFAULT;
        desc.CPUAccessFlags = 0;
        desc.MiscFlags = 0;
        let mut tex = None;
        if device.CreateTexture2D(&desc, None, Some(&mut tex)).is_err() {
            return false;
        }
        let Some(tex) = tex else { return false; };
        let mut srv = None;
        if device.CreateShaderResourceView(&tex, None, Some(&mut srv)).is_err() {
            return false;
        }
        let Some(srv) = srv else { return false; };
        self.copy_tex = Some(tex);
        self.copy_srv = Some(srv);
        self.width = desc.Width;
        self.height = desc.Height;
        self.format = desc.Format;
        true
    }
}

/// Applies fullscreen color grading / post-processing to the swapchain backbuffer.
/// Must be called inside Present before drawing the menu and ESP overlay.
pub unsafe fn apply(
    device: &ID3D11Device,
    context: &ID3D11DeviceContext,
    backbuffer: &ID3D11Texture2D,
    rtv: &ID3D11RenderTargetView,
    cfg: &PostProcessSettings,
) {
    let game = wallbang_core::state();
    let blur = game.settings().misc.motion_blur.clone();
    let snapshot = game.snapshot();
    let velocity = snapshot.local_velocity;
    let speed = (velocity.x * velocity.x + velocity.y * velocity.y).sqrt();
    let matrix = snapshot.view_matrix;
    let valid = snapshot.valid;
    drop(snapshot);
    let mut guard = STATE.lock();
    if !cfg.enabled && !blur.enabled {
        if let Some(state) = guard.as_mut() { state.camera = None; }
        return;
    }
    if guard.is_none() {
        *guard = PostProcessState::create(device);
    }
    let Some(st) = guard.as_mut() else { return; };
    if !st.ensure_copy(device, backbuffer) {
        return;
    }
    let (Some(copy_tex), Some(copy_srv)) = (st.copy_tex.as_ref(), st.copy_srv.as_ref()) else {
        return;
    };

    let forward = [matrix[12], matrix[13], matrix[14]];
    let yaw = forward[1].atan2(forward[0]);
    let pitch = forward[2].atan2((forward[0]*forward[0] + forward[1]*forward[1]).sqrt());
    let now = std::time::Instant::now();
    let strength = if blur.strength.is_finite() { blur.strength.clamp(0.0, 1.0) } else { 0.0 };
    let active = blur.enabled && valid && (!blur.on_velocity || speed > 5.0);
    let mut motion = [0.0; 4];
    if let Some((last_yaw, last_pitch, last_time)) = st.camera {
        let dt = now.saturating_duration_since(last_time).as_secs_f32();
        let angle = |a: f32| (a + std::f32::consts::PI).rem_euclid(std::f32::consts::TAU) - std::f32::consts::PI;
        if active && dt > 0.0001 && dt < 0.1 {
            let shutter = (1.0 / 120.0) / dt;
            motion = [
                (angle(yaw-last_yaw) * shutter * strength * 0.55).clamp(-0.07, 0.07),
                ((pitch-last_pitch) * shutter * strength * 0.55).clamp(-0.07, 0.07),
                (speed / 320.0).clamp(0.0, 2.0) * strength * 0.025,
                1.0,
            ];
        }
    }
    st.camera = valid.then_some((yaw, pitch, now));
    if motion[0].abs() + motion[1].abs() + motion[2].abs() < 0.000_5 { motion = [0.0; 4]; }
    if !cfg.enabled && motion[3] == 0.0 { return; }
    let gpu_sample = st.timing.begin(context);
    context.CopyResource(copy_tex, backbuffer);
    let neutral = PostProcessSettings::default();
    let cfg = if cfg.enabled { cfg } else { &neutral };

    let params = Params {
        p0: [cfg.exposure, cfg.contrast, cfg.saturation, cfg.vibrance],
        p1: [cfg.temperature, cfg.tint, cfg.gamma, cfg.nightmode],
        p2: [
            cfg.vignette,
            cfg.sharpen,
            1.0 / st.width.max(1) as f32,
            1.0 / st.height.max(1) as f32,
        ],
        p3: [
            cfg.world_tint[0] as f32 / 255.0,
            cfg.world_tint[1] as f32 / 255.0,
            cfg.world_tint[2] as f32 / 255.0,
            cfg.world_tint_strength,
        ],
        p4: motion,
    };
    context.UpdateSubresource(
        &st.cb,
        0,
        None,
        &params as *const Params as *const c_void,
        core::mem::size_of::<Params>() as u32,
        0,
    );

    let vp = D3D11_VIEWPORT {
        TopLeftX: 0.0,
        TopLeftY: 0.0,
        Width: st.width as f32,
        Height: st.height as f32,
        MinDepth: 0.0,
        MaxDepth: 1.0,
    };
    let targets = [Some(rtv.clone())];
    context.OMSetRenderTargets(Some(&targets), None);
    context.RSSetViewports(Some(&[vp]));
    context.RSSetState(&st.rasterizer);
    context.OMSetBlendState(None, None, u32::MAX);
    context.OMSetDepthStencilState(None, 0);
    context.IASetInputLayout(None);
    context.IASetPrimitiveTopology(D3D11_PRIMITIVE_TOPOLOGY_TRIANGLELIST);
    context.VSSetShader(&st.vs, None);
    context.PSSetShader(&st.ps, None);
    let cbs = [Some(st.cb.clone())];
    context.PSSetConstantBuffers(0, Some(&cbs));
    let srvs = [Some(copy_srv.clone())];
    context.PSSetShaderResources(0, Some(&srvs));
    let samps = [Some(st.sampler.clone())];
    context.PSSetSamplers(0, Some(&samps));

    context.Draw(3, 0);

    // Unbind SRV to avoid D3D11 hazard on next CopyResource
    let null_srvs: [Option<ID3D11ShaderResourceView>; 1] = [None];
    context.PSSetShaderResources(0, Some(&null_srvs));
    if let Some(index) = gpu_sample { st.timing.end(context, index); }
}

pub fn reset() {
    *STATE.lock() = None;
}
