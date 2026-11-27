// Config block: MAX_VERTICES, VERTEX_STRIDE; D3D11 vtable Present=8/ResizeBuffers=13.
use crate::draw::Vertex;
use std::ffi::c_void;
use windows::Win32::Graphics::Direct3D::D3D11_PRIMITIVE_TOPOLOGY_TRIANGLELIST;
use windows::Win32::Graphics::Direct3D11::*;
use std::time::{Duration, Instant};
use windows::Win32::Graphics::Dxgi::Common::{
    DXGI_FORMAT, DXGI_FORMAT_B8G8R8A8_UNORM, DXGI_FORMAT_B8G8R8A8_UNORM_SRGB,
    DXGI_FORMAT_R10G10B10A2_UNORM, DXGI_FORMAT_R32G32_FLOAT, DXGI_FORMAT_R8G8B8A8_UNORM,
    DXGI_FORMAT_R8G8B8A8_UNORM_SRGB, DXGI_SAMPLE_DESC,
};
use windows::Win32::Graphics::Dxgi::IDXGISwapChain;

pub const MAX_VERTICES: usize = 262_144;
/// Сторона маркера #ff00ff в левом верхнем углу панели превью (в пикселях backbuffer).
/// JS рисует маркер такого размера; draw_preview обрезает эти пиксели при показе.
pub const MARKER_PX: u32 = 6;
// Структура Vertex: x,y,u,v (16 байт) + r,g,b,a (4 байта) = 20.
pub const VERTEX_STRIDE: u32 = std::mem::size_of::<Vertex>() as u32;

// Шейдеры компилируются fxc на этапе сборки и лежат рядом как .cso.
// Runtime D3DCompile намеренно не используется: под manual map он падал в
// d3dcompiler_47.dll до возврата HRESULT.
const VS_BYTECODE: &[u8] = include_bytes!("shaders/menu_vs.cso");
const PS_BYTECODE: &[u8] = include_bytes!("shaders/menu_ps.cso");

struct PreviewCapture {
    front: ID3D11Texture2D,
    front_srv: ID3D11ShaderResourceView,
    back: ID3D11Texture2D,
    back_srv: ID3D11ShaderResourceView,
    // Кольцо из двух staging-текстур: маркер читается с задержкой в 1 кадр
    // через DO_NOT_WAIT, чтобы Map не останавливал GPU каждый кадр.
    staging: [ID3D11Texture2D; 2],
    staging_valid: [bool; 2],
    fmt: DXGI_FORMAT,
    w: u32,
    h: u32,
    marker_ok: bool,
    marker_rgb: (u8, u8, u8),
    ready: bool,
}

/// Цвет маркера из первого пикселя staging-текстуры; None для неподдерживаемого формата.
fn marker_rgb(fmt: DXGI_FORMAT, px: &[u8]) -> Option<(u8, u8, u8)> {
    match fmt {
        DXGI_FORMAT_R8G8B8A8_UNORM | DXGI_FORMAT_R8G8B8A8_UNORM_SRGB => Some((px[0], px[1], px[2])),
        DXGI_FORMAT_B8G8R8A8_UNORM | DXGI_FORMAT_B8G8R8A8_UNORM_SRGB => Some((px[2], px[1], px[0])),
        DXGI_FORMAT_R10G10B10A2_UNORM => {
            let v = u32::from_le_bytes([px[0], px[1], px[2], px[3]]);
            Some((
                ((v & 0x3ff) >> 2) as u8,
                (((v >> 10) & 0x3ff) >> 2) as u8,
                (((v >> 20) & 0x3ff) >> 2) as u8,
            ))
        }
        _ => None,
    }
}

pub struct Dx11 {
    music_art: Option<(u64, ID3D11ShaderResourceView)>,
    icons: Option<crate::skin_icons::IconCache>,
    device: ID3D11Device,
    context: ID3D11DeviceContext,
    rtv: ID3D11RenderTargetView,
    vertex_buffer: ID3D11Buffer,
    input_layout: ID3D11InputLayout,
    vertex_shader: ID3D11VertexShader,
    pixel_shader: ID3D11PixelShader,
    atlas: ID3D11ShaderResourceView,
    sampler: ID3D11SamplerState,
    blend: ID3D11BlendState,
    premultiplied_blend: ID3D11BlendState,
    rasterizer: ID3D11RasterizerState,
    depth_stencil: ID3D11DepthStencilState,
    preview: Option<PreviewCapture>,
    weapon: Option<crate::model::inspect::WeaponRenderer>,
    weapon_initialization_failed: bool,
    model: Option<crate::model::ModelRenderer>,
    stage_idx: usize,
    last_log: Instant,
    pub width: u32,
    pub height: u32,
}

impl Dx11 {
    pub unsafe fn new(swapchain: &IDXGISwapChain) -> Option<Self> {
        crate::trace("dx11: new enter");
        let device = swapchain.GetDevice::<ID3D11Device>().ok()?;
        crate::trace("dx11: new got device");
        let context = device.GetImmediateContext().ok()?;
        crate::trace("dx11: new got context");
        let back_buffer = swapchain.GetBuffer::<ID3D11Texture2D>(0).ok()?;
        crate::trace("dx11: new got backbuffer");

        let mut rtv_slot = None;
        device
            .CreateRenderTargetView(&back_buffer, None, Some(&mut rtv_slot))
            .ok()?;
        let rtv = rtv_slot?;
        crate::trace("dx11: new got rtv");

        let vs_bytes = VS_BYTECODE;
        crate::trace(&format!("dx11: compiled vs, bytes={}", vs_bytes.len()));
        let mut vs_slot = None;
        device
            .CreateVertexShader(vs_bytes, None, Some(&mut vs_slot))
            .ok()?;
        let vertex_shader = vs_slot?;
        crate::trace("dx11: new created vs");
        let ps_bytes = PS_BYTECODE;
        crate::trace(&format!("dx11: compiled ps, bytes={}", ps_bytes.len()));
        let mut ps_slot = None;
        device
            .CreatePixelShader(ps_bytes, None, Some(&mut ps_slot))
            .ok()?;
        let pixel_shader = ps_slot?;
        crate::trace("dx11: new created ps");

        let semantic_position = windows::core::PCSTR(b"POSITION\0".as_ptr());
        let semantic_texcoord = windows::core::PCSTR(b"TEXCOORD\0".as_ptr());
        let semantic_color = windows::core::PCSTR(b"COLOR\0".as_ptr());
        let elements = [
            D3D11_INPUT_ELEMENT_DESC {
                SemanticName: semantic_position,
                SemanticIndex: 0,
                Format: DXGI_FORMAT_R32G32_FLOAT,
                InputSlot: 0,
                AlignedByteOffset: 0,
                InputSlotClass: D3D11_INPUT_PER_VERTEX_DATA,
                InstanceDataStepRate: 0,
            },
            D3D11_INPUT_ELEMENT_DESC {
                SemanticName: semantic_texcoord,
                SemanticIndex: 0,
                Format: DXGI_FORMAT_R32G32_FLOAT,
                InputSlot: 0,
                AlignedByteOffset: 8,
                InputSlotClass: D3D11_INPUT_PER_VERTEX_DATA,
                InstanceDataStepRate: 0,
            },
            D3D11_INPUT_ELEMENT_DESC {
                SemanticName: semantic_color,
                SemanticIndex: 0,
                Format: DXGI_FORMAT_R8G8B8A8_UNORM,
                InputSlot: 0,
                AlignedByteOffset: 16,
                InputSlotClass: D3D11_INPUT_PER_VERTEX_DATA,
                InstanceDataStepRate: 0,
            },
        ];
        let mut layout_slot = None;
        device
            .CreateInputLayout(&elements, vs_bytes, Some(&mut layout_slot))
            .ok()?;
        let input_layout = layout_slot?;

        let buffer_desc = D3D11_BUFFER_DESC {
            ByteWidth: (MAX_VERTICES * std::mem::size_of::<Vertex>()) as u32,
            Usage: D3D11_USAGE_DYNAMIC,
            BindFlags: D3D11_BIND_VERTEX_BUFFER.0 as u32,
            CPUAccessFlags: D3D11_CPU_ACCESS_WRITE.0 as u32,
            MiscFlags: 0,
            StructureByteStride: 0,
        };
        let mut buffer_slot = None;
        device.CreateBuffer(&buffer_desc, None, Some(&mut buffer_slot)).ok()?;
        let vertex_buffer = buffer_slot?;

        let (atlas_bytes, atlas_w, atlas_h) = crate::font::atlas_rgba();
        let tex_desc = D3D11_TEXTURE2D_DESC {
            Width: atlas_w as u32,
            Height: atlas_h as u32,
            MipLevels: 1,
            ArraySize: 1,
            Format: DXGI_FORMAT_R8G8B8A8_UNORM,
            SampleDesc: windows::Win32::Graphics::Dxgi::Common::DXGI_SAMPLE_DESC {
                Count: 1,
                Quality: 0,
            },
            Usage: D3D11_USAGE_DEFAULT,
            BindFlags: D3D11_BIND_SHADER_RESOURCE.0 as u32,
            CPUAccessFlags: 0,
            MiscFlags: 0,
        };
        let sub = D3D11_SUBRESOURCE_DATA {
            pSysMem: atlas_bytes.as_ptr() as *const c_void,
            SysMemPitch: (atlas_w * 4) as u32,
            SysMemSlicePitch: 0,
        };
        let mut texture_slot = None;
        device
            .CreateTexture2D(&tex_desc, Some(&sub), Some(&mut texture_slot))
            .ok()?;
        let atlas_texture = texture_slot?;
        let mut srv_slot = None;
        device
            .CreateShaderResourceView(&atlas_texture, None, Some(&mut srv_slot))
            .ok()?;
        let atlas = srv_slot?;

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
        let mut sampler_slot = None;
        device
            .CreateSamplerState(&sampler_desc, Some(&mut sampler_slot))
            .ok()?;
        let sampler = sampler_slot?;

        let target = D3D11_RENDER_TARGET_BLEND_DESC {
            BlendEnable: true.into(),
            SrcBlend: D3D11_BLEND_SRC_ALPHA,
            DestBlend: D3D11_BLEND_INV_SRC_ALPHA,
            BlendOp: D3D11_BLEND_OP_ADD,
            SrcBlendAlpha: D3D11_BLEND_ONE,
            DestBlendAlpha: D3D11_BLEND_INV_SRC_ALPHA,
            BlendOpAlpha: D3D11_BLEND_OP_ADD,
            RenderTargetWriteMask: D3D11_COLOR_WRITE_ENABLE_ALL.0 as u8,
        };
        let blend_desc = D3D11_BLEND_DESC {
            AlphaToCoverageEnable: false.into(),
            IndependentBlendEnable: false.into(),
            RenderTarget: [target, target, target, target, target, target, target, target],
        };
        let mut blend_slot = None;
        device.CreateBlendState(&blend_desc, Some(&mut blend_slot)).ok()?;
        let blend = blend_slot?;
        let mut premultiplied_desc=blend_desc;
        premultiplied_desc.RenderTarget[0].SrcBlend=D3D11_BLEND_ONE;
        let mut premultiplied_slot=None;
        device.CreateBlendState(&premultiplied_desc,Some(&mut premultiplied_slot)).ok()?;
        let premultiplied_blend=premultiplied_slot?;

        let rast_desc = D3D11_RASTERIZER_DESC {
            FillMode: D3D11_FILL_SOLID,
            CullMode: D3D11_CULL_NONE,
            FrontCounterClockwise: false.into(),
            DepthBias: 0,
            DepthBiasClamp: 0.0,
            SlopeScaledDepthBias: 0.0,
            DepthClipEnable: false.into(),
            ScissorEnable: false.into(),
            MultisampleEnable: false.into(),
            AntialiasedLineEnable: false.into(),
        };
        let mut rast_slot = None;
        device
            .CreateRasterizerState(&rast_desc, Some(&mut rast_slot))
            .ok()?;
        let rasterizer = rast_slot?;

        let ds_desc = D3D11_DEPTH_STENCIL_DESC {
            DepthEnable: false.into(),
            DepthWriteMask: D3D11_DEPTH_WRITE_MASK_ZERO,
            DepthFunc: D3D11_COMPARISON_ALWAYS,
            StencilEnable: false.into(),
            ..Default::default()
        };
        let mut ds_slot = None;
        device
            .CreateDepthStencilState(&ds_desc, Some(&mut ds_slot))
            .ok()?;
        let depth_stencil = ds_slot?;

        let model = crate::model::ModelRenderer::new(&device);
        let desc = swapchain.GetDesc().ok()?;

        Some(Self {
            music_art: None,
            icons: None,
            device,
            context,
            rtv,
            vertex_buffer,
            input_layout,
            vertex_shader,
            pixel_shader,
            atlas,
            sampler,
            blend,
            premultiplied_blend,
            rasterizer,
            depth_stencil,
            preview: None,
            weapon: None,
            weapon_initialization_failed: false,
            model,
            stage_idx: 0,
            last_log: Instant::now(),
            width: desc.BufferDesc.Width.max(1),
            height: desc.BufferDesc.Height.max(1),
        })
    }

    pub unsafe fn draw_music_art(&mut self) {
        let Some((rect, track)) = crate::music::art_request() else { return; };
        let Some(art) = track.art else { self.music_art = None; return; };
        if self.music_art.as_ref().is_none_or(|(revision, _)| *revision != track.revision) {
            let desc = D3D11_TEXTURE2D_DESC { Width: art.width(), Height: art.height(), MipLevels: 1, ArraySize: 1,
                Format: DXGI_FORMAT_R8G8B8A8_UNORM, SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
                Usage: D3D11_USAGE_IMMUTABLE, BindFlags: D3D11_BIND_SHADER_RESOURCE.0 as u32, ..Default::default() };
            let data = D3D11_SUBRESOURCE_DATA { pSysMem: art.as_raw().as_ptr().cast(), SysMemPitch: art.width()*4, SysMemSlicePitch: 0 };
            let mut texture = None;
            if self.device.CreateTexture2D(&desc, Some(&data), Some(&mut texture)).is_ok() {
                if let Some(texture) = texture {
                    let mut srv = None;
                    if self.device.CreateShaderResourceView(&texture, None, Some(&mut srv)).is_ok() {
                        self.music_art = srv.map(|srv| (track.revision, srv));
                    }
                }
            }
        }
        if let Some((_, srv)) = &self.music_art { self.draw_preview(rect, Some(srv)); }
    }

    pub unsafe fn draw_inventory_icons(&mut self, requests:&[crate::skin_icons::IconRequest]) {
        if requests.is_empty() {return;}
        let cache=self.icons.get_or_insert_with(crate::skin_icons::IconCache::new);
        for icon in requests {cache.request((icon.definition,icon.paint));}
        cache.update(&self.device);
        for icon in requests {
            let texture=self.icons.as_mut().and_then(|cache|cache.texture((icon.definition,icon.paint)));
            if let Some((srv,width,height))=texture {
                if let Some(rect)=crate::skin_icons::fit(icon.rect,width,height) {self.draw_preview(rect,Some(&srv));}
            }
        }
    }

    pub unsafe fn recreate(&mut self, swapchain: &IDXGISwapChain) {
        // GetDesc каждый кадр дешёвый; CreateRenderTargetView — нет.
        // Пересоздаём RTV только когда реально изменился размер backbuffer.
        let Ok(desc) = swapchain.GetDesc() else {
            return;
        };
        let width = desc.BufferDesc.Width.max(1);
        let height = desc.BufferDesc.Height.max(1);
        if width == self.width && height == self.height {
            return;
        }
        let Ok(back_buffer) = swapchain.GetBuffer::<ID3D11Texture2D>(0) else {
            return;
        };
        let mut slot = None;
        if self
            .device
            .CreateRenderTargetView(&back_buffer, None, Some(&mut slot))
            .is_ok()
        {
            if let Some(rtv) = slot {
                self.rtv = rtv;
                self.width = width;
                self.height = height;
            }
        }
    }

    pub unsafe fn apply_postprocess(
        &self,
        swap_chain: *mut core::ffi::c_void,
        cfg: &wallbang_config::settings::PostProcessSettings,
    ) {
        if swap_chain.is_null() {
            return;
        }
        let Some(sc) = <IDXGISwapChain as windows::core::Interface>::from_raw_borrowed(&swap_chain) else {
            return;
        };
        let Ok(backbuffer) = sc.GetBuffer::<ID3D11Texture2D>(0) else {
            return;
        };
        crate::postprocess::apply(&self.device, &self.context, &backbuffer, &self.rtv, cfg);
    }

    pub unsafe fn draw(&self, vertices: &[Vertex]) {
        if vertices.is_empty() || vertices.len() > MAX_VERTICES {
            return;
        }
        // D3D11_USAGE_DYNAMIC требует Map(WRITE_DISCARD) + Unmap (UpdateSubresource невалиден для DYNAMIC).
        let mut mapped = D3D11_MAPPED_SUBRESOURCE::default();
        if self
            .context
            .Map(&self.vertex_buffer, 0, D3D11_MAP_WRITE_DISCARD, 0, Some(&mut mapped))
            .is_err()
        {
            return;
        }
        std::ptr::copy_nonoverlapping(
            vertices.as_ptr(),
            mapped.pData as *mut Vertex,
            vertices.len(),
        );
        self.context.Unmap(&self.vertex_buffer, 0);

        let stride = VERTEX_STRIDE;
        let offset = 0u32;
        let targets = [Some(self.rtv.clone())];
        self.context.OMSetRenderTargets(Some(&targets), None::<&ID3D11DepthStencilView>);
        self.context.OMSetDepthStencilState(&self.depth_stencil, 0);
        self.context.OMSetBlendState(&self.blend, Some(&[0.0; 4]), u32::MAX);
        self.context.RSSetState(&self.rasterizer);
        let viewport = D3D11_VIEWPORT {
            TopLeftX: 0.0,
            TopLeftY: 0.0,
            Width: self.width as f32,
            Height: self.height as f32,
            MinDepth: 0.0,
            MaxDepth: 1.0,
        };
        self.context.RSSetViewports(Some(&[viewport]));

        let buffers = [Some(self.vertex_buffer.clone())];
        self.context.IASetVertexBuffers(0, 1, Some(buffers.as_ptr()), Some(&stride), Some(&offset));
        self.context.IASetPrimitiveTopology(D3D11_PRIMITIVE_TOPOLOGY_TRIANGLELIST);
        self.context.IASetInputLayout(&self.input_layout);
        self.context.VSSetShader(&self.vertex_shader, None);
        self.context.PSSetShader(&self.pixel_shader, None);
        self.context.GSSetShader(None, None);
        self.context.HSSetShader(None, None);
        self.context.DSSetShader(None, None);
        let views = [Some(self.atlas.clone())];
        let samplers = [Some(self.sampler.clone())];
        self.context.PSSetShaderResources(0, Some(&views));
        self.context.PSSetSamplers(0, Some(&samplers));
        self.context.Draw(vertices.len() as u32, 0);
    }

    unsafe fn ensure_preview(&mut self, w: u32, h: u32, fmt: DXGI_FORMAT) -> Option<()> {
        if let Some(p) = &self.preview {
            if p.w == w && p.h == h && p.fmt == fmt {
                return Some(());
            }
        }
        // Формат берём у backbuffer: CopySubresourceRegion требует совместимых форматов.
        let tex_desc = |usage: D3D11_USAGE, bind: u32, cpu: u32| D3D11_TEXTURE2D_DESC {
            Width: w,
            Height: h,
            MipLevels: 1,
            ArraySize: 1,
            Format: fmt,
            SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
            Usage: usage,
            BindFlags: bind,
            CPUAccessFlags: cpu,
            MiscFlags: 0,
        };
        let make = |desc: &D3D11_TEXTURE2D_DESC| -> Option<(ID3D11Texture2D, ID3D11ShaderResourceView)> {
            let mut tex = None;
            self.device.CreateTexture2D(desc, None, Some(&mut tex)).ok()?;
            let tex = tex?;
            let mut srv = None;
            self.device.CreateShaderResourceView(&tex, None, Some(&mut srv)).ok()?;
            Some((tex, srv?))
        };
        let desc_default = tex_desc(D3D11_USAGE_DEFAULT, D3D11_BIND_SHADER_RESOURCE.0 as u32, 0);
        let (front, front_srv) = make(&desc_default)?;
        let (back, back_srv) = make(&desc_default)?;
        let desc_staging = tex_desc(D3D11_USAGE_STAGING, 0, D3D11_CPU_ACCESS_READ.0 as u32);
        let mut st0 = None;
        self.device.CreateTexture2D(&desc_staging, None, Some(&mut st0)).ok()?;
        let mut st1 = None;
        self.device.CreateTexture2D(&desc_staging, None, Some(&mut st1)).ok()?;
        self.preview = Some(PreviewCapture {
            front,
            front_srv,
            back,
            back_srv,
            staging: [st0?, st1?],
            staging_valid: [false, false],
            fmt,
            w,
            h,
            marker_ok: false,
            marker_rgb: (0, 0, 0),
            ready: false,
        });
        self.stage_idx = 0;
        Some(())
    }

    /// Забирает область backbuffer, где Panorama отрисовала модель и ESP.
    /// Возвращает true, если свежий кадр захвачен (маркер найден).
    pub unsafe fn capture_preview(&mut self, swapchain: &IDXGISwapChain, rect: (f32, f32, f32, f32)) -> bool {
        let m = MARKER_PX;
        let x = rect.0.max(0.0) as u32;
        let y = rect.1.max(0.0) as u32;
        let w = rect.2.max(1.0) as u32;
        let h = rect.3.max(1.0) as u32;
        if w <= m * 2 || h <= m * 2 || x + w > self.width || y + h > self.height {
            if let Some(p) = &mut self.preview {
                p.ready = false;
                p.marker_ok = false;
            }
            return false;
        }
        let Ok(back_buffer) = swapchain.GetBuffer::<ID3D11Texture2D>(0) else { return false };
        let mut desc = D3D11_TEXTURE2D_DESC::default();
        back_buffer.GetDesc(&mut desc);
        if desc.SampleDesc.Count != 1 || x + w > desc.Width || y + h > desc.Height {
            return false;
        }
        if marker_rgb(desc.Format, &[0u8; 4]).is_none() {
            if self.last_log.elapsed() > Duration::from_secs(1) {
                self.last_log = Instant::now();
                crate::trace(&format!("preview: unsupported backbuffer format {:?}", desc.Format));
            }
            return false;
        }
        if self.ensure_preview(w, h, desc.Format).is_none() {
            return false;
        }
        let cur = self.stage_idx;
        let prev = cur ^ 1;
        let Some(p) = self.preview.as_mut() else { return false };

        // 1. Результат маркера с прошлого кадра. DO_NOT_WAIT: если GPU ещё не
        //    закончил копирование, оставляем прошлое решение и не блокируемся.
        if p.staging_valid[prev] {
            let mut mapped = D3D11_MAPPED_SUBRESOURCE::default();
            if self
                .context
                .Map(&p.staging[prev], 0, D3D11_MAP_READ, D3D11_MAP_FLAG_DO_NOT_WAIT.0 as u32, Some(&mut mapped))
                .is_ok()
            {
                let px = std::slice::from_raw_parts(mapped.pData as *const u8, 4);
                if let Some(rgb) = marker_rgb(p.fmt, px) {
                    p.marker_rgb = rgb;
                    p.marker_ok = rgb.0 > 240 && rgb.1 < 16 && rgb.2 > 240;
                }
                self.context.Unmap(&p.staging[prev], 0);
                p.staging_valid[prev] = false;
            }
        }

        // 2. Маркер этого кадра уходит в staging, прочитаем его на следующем.
        //    Читаем пиксель (x+1, y+1): он внутри маркера MARKER_PX x MARKER_PX.
        let marker_box = D3D11_BOX { left: x + 1, top: y + 1, front: 0, right: x + 2, bottom: y + 2, back: 1 };
        self.context
            .CopySubresourceRegion(&p.staging[cur], 0, 0, 0, 0, &back_buffer, 0, Some(&marker_box));
        p.staging_valid[cur] = true;
        self.stage_idx = prev;

        // 3. Сам кадр копируем только если маркер подтверждён.
        if p.marker_ok {
            let area = D3D11_BOX { left: x, top: y, front: 0, right: x + w, bottom: y + h, back: 1 };
            self.context.CopySubresourceRegion(&p.back, 0, 0, 0, 0, &back_buffer, 0, Some(&area));
            std::mem::swap(&mut p.front, &mut p.back);
            std::mem::swap(&mut p.front_srv, &mut p.back_srv);
            p.ready = true;
        } else {
            p.ready = false;
        }

        if self.last_log.elapsed() > Duration::from_secs(1) {
            self.last_log = Instant::now();
            crate::trace(&format!(
                "preview: rect=({x},{y},{w},{h}) bb={}x{} fmt={:?} marker=({},{},{}) ok={} ready={}",
                desc.Width, desc.Height, desc.Format,
                p.marker_rgb.0, p.marker_rgb.1, p.marker_rgb.2, p.marker_ok, p.ready
            ));
        }
        p.ready
    }

    /// Own DX11 geometry into an owned RTV; no backbuffer capture or game scene.
    pub unsafe fn render_weapon(&mut self, request: &crate::skin_inspect::Request) -> Option<ID3D11ShaderResourceView> {
        let start=Instant::now();
        if self.weapon.is_none() && !self.weapon_initialization_failed {
            self.weapon=crate::model::inspect::WeaponRenderer::new(&self.device);
            self.weapon_initialization_failed=self.weapon.is_none();
        }
        if self.weapon_initialization_failed {
            crate::skin_inspect::report_error("DX11 preview initialization failed".into());
        }
        let result=self.weapon.as_mut().and_then(|renderer|renderer.render(&self.device,&self.context,request));
        // The owned pass must never leave its render target bound in the game.
        self.context.OMSetRenderTargets(Some(&[Some(self.rtv.clone())]), None::<&ID3D11DepthStencilView>);
        self.context.RSSetViewports(Some(&[D3D11_VIEWPORT {TopLeftX:0.0,TopLeftY:0.0,Width:self.width as f32,Height:self.height as f32,MinDepth:0.0,MaxDepth:1.0}]));
        crate::skin_inspect::profile(start);
        result
    }

    /// Owned-model pass into the offscreen target. Returns its SRV for the preview window.
    pub unsafe fn render_model(
        &mut self,
        rect: (f32, f32, f32, f32),
        clock: f32,
        enemy: bool,
        material: crate::model::MaterialOverride,
    ) -> Option<ID3D11ShaderResourceView> {
        let context = self.context.clone();
        let result = self
            .model
            .as_mut()?
            .render(&context, rect, clock, enemy, material);
        // Owned RT must be unbound immediately so the game backbuffer is active
        // again before the rest of the Present hook draws the menu.
        let targets = [Some(self.rtv.clone())];
        self.context
            .OMSetRenderTargets(Some(&targets), None::<&ID3D11DepthStencilView>);
        let viewport = D3D11_VIEWPORT {
            TopLeftX: 0.0,
            TopLeftY: 0.0,
            Width: self.width as f32,
            Height: self.height as f32,
            MinDepth: 0.0,
            MaxDepth: 1.0,
        };
        self.context.RSSetViewports(Some(&[viewport]));
        if self.last_log.elapsed() > Duration::from_secs(1) {
            self.last_log = Instant::now();
            crate::trace(&format!(
                "preview: rect=({:.0},{:.0},{:.0},{:.0}) fmt={:?} marker=(0,0,0) ok={} ready={}",
                rect.0,
                rect.1,
                rect.2,
                rect.3,
                DXGI_FORMAT_R8G8B8A8_UNORM,
                self.model.is_some() as usize,
                result.is_some() as usize
            ));
        }
        result
    }

    /// Рисует захваченную текстуру в прямоугольник вьюпорта превью.
    /// `override_srv` используется собственным рендером агента.
    pub unsafe fn draw_preview(
        &self,
        rect: (f32, f32, f32, f32),
        override_srv: Option<&ID3D11ShaderResourceView>,
    ) {
        self.draw_preview_impl(rect,override_srv,false);
    }
    pub unsafe fn draw_weapon_preview(&self,rect:(f32,f32,f32,f32),srv:&ID3D11ShaderResourceView) {
        self.draw_preview_impl(rect,Some(srv),true);
    }
    unsafe fn draw_preview_impl(&self,rect:(f32,f32,f32,f32),override_srv:Option<&ID3D11ShaderResourceView>,premultiplied:bool) {
        let (srv, pw, ph) = if let Some(srv) = override_srv {
            (srv.clone(), rect.2.max(1.0), rect.3.max(1.0))
        } else {
            let Some(p) = self.preview.as_ref().filter(|p| p.ready) else { return };
            (p.front_srv.clone(), p.w as f32, p.h as f32)
        };
        let (x, y, w, h) = rect;
        // Маркер занимает левый верхний угол захвата Panorama: собственный RT его не рисует.
        let m = if override_srv.is_some() { 0.0 } else { MARKER_PX as f32 };
        let (u0, v0) = (m / pw, m / ph);
        let (x, y, w, h) = (x + m, y + m, w - m, h - m);
        let ndc = |px: f32, py: f32| {
            (px / self.width as f32 * 2.0 - 1.0, 1.0 - py / self.height as f32 * 2.0)
        };
        let c: u8 = 255;
        let mut verts = [Vertex::default(); 6];
        let corners = [
            (x, y, u0, v0),
            (x + w, y, 1.0, v0),
            (x + w, y + h, 1.0, 1.0),
            (x, y, u0, v0),
            (x + w, y + h, 1.0, 1.0),
            (x, y + h, u0, 1.0),
        ];
        for (v, (px, py, u, vv)) in verts.iter_mut().zip(corners) {
            let (nx, ny) = ndc(px, py);
            *v = Vertex { x: nx, y: ny, u, v: vv, r: c, g: c, b: c, a: c };
        }

        let mut mapped = D3D11_MAPPED_SUBRESOURCE::default();
        if self.context.Map(&self.vertex_buffer, 0, D3D11_MAP_WRITE_DISCARD, 0, Some(&mut mapped)).is_err() {
            return;
        }
        std::ptr::copy_nonoverlapping(verts.as_ptr(), mapped.pData as *mut Vertex, verts.len());
        self.context.Unmap(&self.vertex_buffer, 0);

        let targets = [Some(self.rtv.clone())];
        self.context.OMSetRenderTargets(Some(&targets), None::<&ID3D11DepthStencilView>);
        self.context.OMSetDepthStencilState(&self.depth_stencil, 0);
        self.context.OMSetBlendState(if premultiplied {&self.premultiplied_blend} else {&self.blend}, Some(&[0.0; 4]), u32::MAX);
        self.context.RSSetState(&self.rasterizer);
        let viewport = D3D11_VIEWPORT {
            TopLeftX: 0.0,
            TopLeftY: 0.0,
            Width: self.width as f32,
            Height: self.height as f32,
            MinDepth: 0.0,
            MaxDepth: 1.0,
        };
        self.context.RSSetViewports(Some(&[viewport]));
        let stride = VERTEX_STRIDE;
        let offset = 0u32;
        let buffers = [Some(self.vertex_buffer.clone())];
        self.context.IASetVertexBuffers(0, 1, Some(buffers.as_ptr()), Some(&stride), Some(&offset));
        self.context.IASetPrimitiveTopology(D3D11_PRIMITIVE_TOPOLOGY_TRIANGLELIST);
        self.context.IASetInputLayout(&self.input_layout);
        self.context.VSSetShader(&self.vertex_shader, None);
        self.context.PSSetShader(&self.pixel_shader, None);
        let views = [Some(srv)];
        let samplers = [Some(self.sampler.clone())];
        self.context.PSSetShaderResources(0, Some(&views));
        self.context.PSSetSamplers(0, Some(&samplers));
        self.context.Draw(6, 0);
    }
}
