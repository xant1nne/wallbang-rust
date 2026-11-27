// Headless GPU verification: the same owned renderer used by the menu.
use std::{
    thread,
    time::{Duration, Instant},
};
use wallbang_render::{model::inspect::WeaponRenderer, skin_inspect::Request};
use windows::{
    core::Interface,
    Win32::{
        Foundation::HMODULE,
        Graphics::{
            Direct3D::{D3D_DRIVER_TYPE_WARP, D3D_FEATURE_LEVEL_11_0},
            Direct3D11::*,
        },
    },
};
const TICK_SLEEP: Duration = Duration::from_millis(4);
unsafe fn pixels(context: &ID3D11DeviceContext, srv: &ID3D11ShaderResourceView) -> Vec<u8> {
    let resource = srv.GetResource().unwrap();
    let texture: ID3D11Texture2D = resource.cast().unwrap();
    let mut desc = D3D11_TEXTURE2D_DESC::default();
    texture.GetDesc(&mut desc);
    let mut staging = None;
    let device = context.GetDevice().unwrap();
    device
        .CreateTexture2D(
            &D3D11_TEXTURE2D_DESC {
                Usage: D3D11_USAGE_STAGING,
                BindFlags: 0,
                CPUAccessFlags: D3D11_CPU_ACCESS_READ.0 as u32,
                ..desc
            },
            None,
            Some(&mut staging),
        )
        .unwrap();
    let staging = staging.unwrap();
    context.PSSetShaderResources(0, Some(&[None]));
    context.OMSetRenderTargets(None, None::<&ID3D11DepthStencilView>);
    context.CopyResource(&staging, &texture);
    let mut mapped = D3D11_MAPPED_SUBRESOURCE::default();
    context
        .Map(&staging, 0, D3D11_MAP_READ, 0, Some(&mut mapped))
        .unwrap();
    let mut pixels = vec![0; desc.Width as usize * desc.Height as usize * 4];
    for y in 0..desc.Height as usize {
        std::ptr::copy_nonoverlapping(
            (mapped.pData as *const u8).add(y * mapped.RowPitch as usize),
            pixels.as_mut_ptr().add(y * desc.Width as usize * 4),
            desc.Width as usize * 4,
        );
    }
    context.Unmap(&staging, 0);
    pixels
}
fn main() {
    unsafe {
        let mut device = None;
        let mut context = None;
        D3D11CreateDevice(
            None,
            D3D_DRIVER_TYPE_WARP,
            HMODULE::default(),
            D3D11_CREATE_DEVICE_FLAG(0),
            Some(&[D3D_FEATURE_LEVEL_11_0]),
            D3D11_SDK_VERSION,
            Some(&mut device),
            None,
            Some(&mut context),
        )
        .unwrap();
        let device = device.unwrap();
        let context = context.unwrap();
        let mut renderer = WeaponRenderer::new(&device).expect("own DX11 pipeline");
        std::fs::create_dir_all("target/tmp/weapon-scene").unwrap();
        let mut previous = None;
        for (name, definition, paint, yaw, pitch) in [
            ("awp", 9, 0, 0., 0.),
            ("awp_rotated", 9, 0, 55., -15.),
            ("awp_asiimov", 9, 279, 0., 0.),
            ("ak_redline", 7, 282, 0., 0.),
            ("aug", 8, 0, 0., 0.),
            ("sg553", 39, 0, 0., 0.),
            ("butterfly", 515, 0, 0., 0.),
            ("gloves", 5030, 0, 0., 0.),
        ] {
            let request = Request {
                rect: [0., 0., 640., 320.],
                definition,
                paint,
                yaw,
                pitch,
                zoom: 1.1,
                show_model: true,
            };
            let load_start = Instant::now();
            let deadline = load_start + Duration::from_secs(15);
            let srv = loop {
                if let Some(srv) = renderer.render(&device, &context, &request) {
                    break srv;
                }
                assert!(Instant::now() < deadline, "model load timed out: {name}");
                thread::sleep(TICK_SLEEP);
            };
            let image = pixels(&context, &srv);
            let load_time = load_start.elapsed();
            let visible = image.chunks_exact(4).filter(|p| p[3] > 128).count();
            assert!(visible > 10000, "empty/edge-on scene: {name}");
            assert!(
                visible < 960 * 480 * 9 / 10,
                "clipped/full-screen scene: {name}"
            );
            let cached = renderer.render(&device, &context, &request).unwrap();
            assert_eq!(pixels(&context, &cached), image, "idle cache changed");
            let cache_start = Instant::now();
            for _ in 0..120 {
                assert!(renderer.render(&device, &context, &request).is_some());
            }
            let cached_us = cache_start.elapsed().as_micros() as f64 / 120.0;
            if name == "awp_rotated" {
                assert_ne!(
                    previous.as_ref().unwrap(),
                    &image,
                    "rotation must change geometry"
                );
            }
            if name == "awp_asiimov" {
                let orange = image
                    .chunks_exact(4)
                    .filter(|p| {
                        p[3] > 128 && p[0] as f32 > p[1] as f32 * 1.3 && p[1] > 40 && p[2] < 100
                    })
                    .count();
                assert!(
                    orange > 500,
                    "Asiimov finish must appear on real geometry, orange pixels={orange}"
                );
            }
            previous = Some(image.clone());
            // The owned RT uses premultiplied alpha; standard PNG uses straight.
            let mut exported = image.clone();
            for pixel in exported.chunks_exact_mut(4) {
                if pixel[3] > 0 && pixel[3] < 255 {
                    for c in 0..3 {
                        pixel[c] = (pixel[c] as u32 * 255 / pixel[3] as u32).min(255) as u8;
                    }
                }
            }
            image::save_buffer(
                format!("target/tmp/weapon-scene/{name}.png"),
                &exported,
                960,
                480,
                image::ColorType::Rgba8,
            )
            .unwrap();
            println!("{name}: visible={visible}, first={:.1}ms, idle={cached_us:.2}us; own render/cache passed",load_time.as_secs_f64()*1000.0);
        }
    }
}
