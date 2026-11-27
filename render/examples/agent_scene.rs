// GPU visual check for both owned agent previews.
use wallbang_render::model::{ModelRenderer, MaterialOverride};
use windows::{core::Interface, Win32::{Foundation::HMODULE, Graphics::{Direct3D::{D3D_DRIVER_TYPE_WARP,D3D_FEATURE_LEVEL_11_0},Direct3D11::*}}};
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

fn main() { unsafe {
let mut device=None;let mut context=None;
D3D11CreateDevice(None,D3D_DRIVER_TYPE_WARP,HMODULE::default(),D3D11_CREATE_DEVICE_FLAG(0),Some(&[D3D_FEATURE_LEVEL_11_0]),D3D11_SDK_VERSION,Some(&mut device),None,Some(&mut context)).unwrap();
let device=device.unwrap();let context=context.unwrap();let mut renderer=ModelRenderer::new(&device).unwrap();
std::fs::create_dir_all("target/tmp/agent-scene").unwrap();
for (name,enemy) in [("t",true),("ct",false)] {
let srv=renderer.render(&context,(0.0,0.0,400.0,560.0),0.0,enemy,MaterialOverride::empty()).unwrap();
let bytes=pixels(&context,&srv); image::save_buffer(format!("target/tmp/agent-scene/{name}.png"),&bytes,800,1120,image::ColorType::Rgba8).unwrap();
}
} }

