// Offline only: bake the glTF bind pose and keep native UVs. No game offsets.
use image::{
    codecs::png::{CompressionType, FilterType, PngEncoder},
    ExtendedColorType, ImageEncoder,
};
use std::{fs, io::Write, path::Path};
const TEXTURE_SIZE: u32 = 1024;
const FINISH_JPEG_QUALITY: u8 = 95;
const PACK_WORKERS: usize = 4;
type Matrix = [[f32; 4]; 4];
fn identity() -> Matrix {
    [
        [1., 0., 0., 0.],
        [0., 1., 0., 0.],
        [0., 0., 1., 0.],
        [0., 0., 0., 1.],
    ]
}
fn mul(a: Matrix, b: Matrix) -> Matrix {
    let mut o = [[0.; 4]; 4];
    for c in 0..4 {
        for r in 0..4 {
            for k in 0..4 {
                o[c][r] += a[k][r] * b[c][k];
            }
        }
    }
    o
}
fn transform(m: Matrix, p: [f32; 3], w: f32) -> [f32; 3] {
    std::array::from_fn(|r| m[0][r] * p[0] + m[1][r] * p[1] + m[2][r] * p[2] + m[3][r] * w)
}
fn global(index: usize, nodes: &[Matrix], parents: &[Option<usize>]) -> Matrix {
    let mut result = nodes[index];
    let mut p = parents[index];
    let mut depth = 0;
    while let Some(i) = p {
        assert!(depth < nodes.len(), "cyclic node hierarchy");
        result = mul(nodes[i], result);
        p = parents[i];
        depth += 1;
    }
    result
}
fn u32le(out: &mut Vec<u8>, v: usize) {
    out.extend_from_slice(&(v as u32).to_le_bytes());
}
fn export(input: &Path, output: &Path, legacy: bool) -> Result<(), Box<dyn std::error::Error>> {
    let (doc, buffers, images) = gltf::import(input)?;
    let get = |b: gltf::Buffer<'_>| buffers.get(b.index()).map(|v| v.0.as_slice());
    let nodes: Vec<_> = doc.nodes().map(|n| n.transform().matrix()).collect();
    let mut parents = vec![None; nodes.len()];
    for node in doc.nodes() {
        for c in node.children() {
            parents[c.index()] = Some(node.index());
        }
    }
    let globals: Vec<_> = (0..nodes.len())
        .map(|i| global(i, &nodes, &parents))
        .collect();
    let has_hd = doc
        .meshes()
        .any(|m| m.name().unwrap_or("").ends_with(".body_hd"));
    let has_legacy = doc
        .meshes()
        .any(|m| m.name().unwrap_or("").ends_with(".body_legacy"));
    let has_viewmodel = doc
        .meshes()
        .any(|m| m.name().unwrap_or("").ends_with(".viewmodel"));
    let mut vertices = Vec::<[f32; 8]>::new();
    let mut indices = Vec::<u32>::new();
    let mut primitives = Vec::new();
    let mut textures = Vec::new();
    for node in doc.nodes() {
        let Some(mesh) = node.mesh() else { continue };
        let name = mesh.name().unwrap_or("");
        if name.ends_with(".eholster") || (has_viewmodel && name.ends_with(".worldmodel")) {
            continue;
        }
        if has_hd
            && has_legacy
            && ((legacy && name.ends_with(".body_hd"))
                || (!legacy && name.ends_with(".body_legacy")))
        {
            continue;
        }
        let skin = node.skin().map(|s| {
            let inv: Vec<_> = s
                .reader(get)
                .read_inverse_bind_matrices()
                .map(|m| m.collect())
                .unwrap_or_default();
            s.joints()
                .enumerate()
                .map(|(i, j)| {
                    mul(
                        globals[j.index()],
                        inv.get(i).copied().unwrap_or_else(identity),
                    )
                })
                .collect::<Vec<_>>()
        });
        for primitive in mesh.primitives() {
            if primitive
                .material()
                .name()
                .unwrap_or("")
                .starts_with("bare_arm")
            {
                continue;
            }
            if primitive.mode() != gltf::mesh::Mode::Triangles {
                continue;
            }
            let reader = primitive.reader(get);
            let pos: Vec<_> = reader
                .read_positions()
                .ok_or("missing positions")?
                .collect();
            let normal: Vec<_> = reader
                .read_normals()
                .map(|v| v.collect())
                .unwrap_or_default();
            let uv: Vec<_> = reader
                .read_tex_coords(0)
                .map(|v| v.into_f32().collect())
                .unwrap_or_default();
            let joints: Vec<_> = reader
                .read_joints(0)
                .map(|v| v.into_u16().collect())
                .unwrap_or_default();
            let weights: Vec<_> = reader
                .read_weights(0)
                .map(|v| v.into_f32().collect())
                .unwrap_or_default();
            let first_vertex = vertices.len() as u32;
            for (i, p) in pos.iter().enumerate() {
                let mut m = globals[node.index()];
                if let (Some(skin), Some(js), Some(ws)) = (&skin, joints.get(i), weights.get(i)) {
                    m = [[0.; 4]; 4];
                    for k in 0..4 {
                        let joint = skin.get(js[k] as usize).ok_or("bad joint")?;
                        for c in 0..4 {
                            for r in 0..4 {
                                m[c][r] += joint[c][r] * ws[k];
                            }
                        }
                    }
                }
                let p = transform(m, *p, 1.);
                let mut n = transform(m, *normal.get(i).unwrap_or(&[0., 1., 0.]), 0.);
                let len = n.iter().map(|v| v * v).sum::<f32>().sqrt().max(1e-6);
                for v in &mut n {
                    *v /= len;
                }
                let uv = uv.get(i).copied().unwrap_or([0.; 2]);
                vertices.push([p[0], p[1], p[2], n[0], n[1], n[2], uv[0], uv[1]]);
            }
            let first = indices.len();
            if let Some(values) = reader.read_indices() {
                indices.extend(values.into_u32().map(|i| first_vertex + i));
            } else {
                indices.extend((0..pos.len() as u32).map(|i| first_vertex + i));
            }
            let pbr = primitive.material().pbr_metallic_roughness();
            let factor = pbr.base_color_factor();
            let mut rgba = if let Some(info) = pbr.base_color_texture() {
                let image = &images[info.texture().source().index()];
                match image.format {
                    gltf::image::Format::R8G8B8A8 => {
                        image::RgbaImage::from_raw(image.width, image.height, image.pixels.clone())
                            .ok_or("bad RGBA")?
                    }
                    gltf::image::Format::R8G8B8 => image::DynamicImage::ImageRgb8(
                        image::RgbImage::from_raw(image.width, image.height, image.pixels.clone())
                            .ok_or("bad RGB")?,
                    )
                    .to_rgba8(),
                    _ => return Err("unsupported texture format".into()),
                }
            } else {
                image::RgbaImage::from_pixel(1, 1, image::Rgba([255; 4]))
            };
            if rgba.width() > TEXTURE_SIZE || rgba.height() > TEXTURE_SIZE {
                rgba = image::DynamicImage::ImageRgba8(rgba)
                    .resize(
                        TEXTURE_SIZE,
                        TEXTURE_SIZE,
                        image::imageops::FilterType::Lanczos3,
                    )
                    .to_rgba8();
            }
            for p in rgba.pixels_mut() {
                for c in 0..4 {
                    p[c] = (p[c] as f32 * factor[c]).round().clamp(0., 255.) as u8;
                }
            }
            let mut png = Vec::new();
            PngEncoder::new_with_quality(&mut png, CompressionType::Best, FilterType::Adaptive)
                .write_image(
                    rgba.as_raw(),
                    rgba.width(),
                    rgba.height(),
                    ExtendedColorType::Rgba8,
                )?;
            let material = primitive
                .material()
                .name()
                .unwrap_or("")
                .to_ascii_lowercase();
            let alpha = primitive.material().alpha_mode() != gltf::material::AlphaMode::Opaque;
            let paintable = !alpha
                && !["scope", "lens", "glass", "bullet", "shell", "arms", "skin"]
                    .iter()
                    .any(|word| material.contains(word));
            let t = textures.len();
            textures.push(png);
            primitives.push((
                first,
                indices.len() - first,
                t,
                usize::from(paintable) | (usize::from(alpha) << 1),
            ));
        }
    }
    if vertices.is_empty() || indices.is_empty() {
        return Err("empty weapon".into());
    }
    // glTF accessors may expose an entire shared vertex buffer even when a
    // primitive uses only a small subset. Unused arm vertices must not affect
    // the glove camera bounds, and should not be uploaded to the GPU either.
    let mut remap = vec![u32::MAX; vertices.len()];
    let mut compact = Vec::new();
    for index in &mut indices {
        let old = *index as usize;
        let slot = &mut remap[old];
        if *slot == u32::MAX {
            *slot = compact.len() as u32;
            compact.push(vertices[old]);
        }
        *index = *slot;
    }
    vertices = compact;
    let mut out = b"WBI2".to_vec();
    for count in [
        vertices.len(),
        indices.len(),
        primitives.len(),
        textures.len(),
    ] {
        u32le(&mut out, count);
    }
    for v in &vertices {
        for f in v {
            out.extend_from_slice(&f.to_le_bytes());
        }
    }
    for i in &indices {
        out.extend_from_slice(&i.to_le_bytes());
    }
    for (a, b, c, d) in primitives {
        for n in [a, b, c, d] {
            u32le(&mut out, n);
        }
    }
    for png in textures {
        u32le(&mut out, png.len());
        out.extend(png);
    }
    let mut file = fs::File::create(output)?;
    file.write_all(&out)?;
    println!(
        "{}: vertices={} triangles={} bytes={}",
        output.display(),
        vertices.len(),
        indices.len() / 3,
        out.len()
    );
    Ok(())
}
fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() == 3 && args[1] == "--textures" {
        let jobs: serde_json::Value = serde_json::from_slice(&fs::read(&args[2]).unwrap()).unwrap();
        for job in jobs.as_array().unwrap() {
            let image = image::open(job["input"].as_str().unwrap()).unwrap();
            let mut image = image
                .resize(
                    TEXTURE_SIZE,
                    TEXTURE_SIZE,
                    image::imageops::FilterType::Lanczos3,
                )
                .to_rgba8();
            for p in image.pixels_mut() {
                p[3] = 255;
            }
            let file = fs::File::create(job["output"].as_str().unwrap()).unwrap();
            image::codecs::jpeg::JpegEncoder::new_with_quality(file, FINISH_JPEG_QUALITY)
                .encode_image(&image::DynamicImage::ImageRgba8(image).to_rgb8())
                .unwrap();
        }
        return;
    }
    if args.len() != 2 {
        eprintln!("usage: wallbang_weaponpack <jobs.json> | --textures <jobs.json>");
        std::process::exit(2);
    }
    let jobs: serde_json::Value = serde_json::from_slice(&fs::read(&args[1]).unwrap()).unwrap();
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
    let jobs = jobs.as_array().unwrap();
    let cursor = AtomicUsize::new(0);
    let failed = AtomicBool::new(false);
    std::thread::scope(|scope| {
        for _ in 0..PACK_WORKERS {
            let (cursor, failed) = (&cursor, &failed);
            scope.spawn(move || loop {
                let index = cursor.fetch_add(1, Ordering::Relaxed);
                let Some(job) = jobs.get(index) else {
                    break;
                };
                if let Err(e) = export(
                    Path::new(job["input"].as_str().unwrap()),
                    Path::new(job["output"].as_str().unwrap()),
                    job["legacy"].as_bool().unwrap_or(false),
                ) {
                    eprintln!("{job}: {e}");
                    failed.store(true, Ordering::Relaxed);
                }
            });
        }
    });
    if failed.load(Ordering::Relaxed) {
        std::process::exit(1);
    }
}
