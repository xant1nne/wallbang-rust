// Offline alpha crop and lossless PNG optimization. No game/runtime dependency.
use image::{ImageEncoder,ExtendedColorType};
use image::codecs::png::{PngEncoder,CompressionType,FilterType};
use std::sync::{Arc,atomic::{AtomicUsize,AtomicU64,Ordering}};
use std::path::Path;

fn optimize(path:&Path)->Result<(u64,u64),String> {
    let source=std::fs::read(path).map_err(|e|e.to_string())?;
    let image=image::load_from_memory_with_format(&source,image::ImageFormat::Png).map_err(|e|e.to_string())?.to_rgba8();
    let (width,height)=image.dimensions();let (mut x0,mut y0,mut x1,mut y1)=(width,height,0u32,0u32);
    for (x,y,pixel) in image.enumerate_pixels() {if pixel[3]!=0 {x0=x0.min(x);y0=y0.min(y);x1=x1.max(x+1);y1=y1.max(y+1);}}
    if x1<=x0||y1<=y0 {return Ok((source.len() as u64,source.len() as u64));}
    x0=x0.saturating_sub(2);y0=y0.saturating_sub(2);x1=(x1+2).min(width);y1=(y1+2).min(height);
    let cropped=image::imageops::crop_imm(&image,x0,y0,x1-x0,y1-y0).to_image();
    let mut output=Vec::new();
    PngEncoder::new_with_quality(&mut output,CompressionType::Best,FilterType::Adaptive)
        .write_image(cropped.as_raw(),cropped.width(),cropped.height(),ExtendedColorType::Rgba8).map_err(|e|e.to_string())?;
    std::fs::write(path,&output).map_err(|e|e.to_string())?;
    Ok((source.len() as u64,output.len() as u64))
}
fn main() {
    let folder=std::env::args_os().nth(1).expect("usage: wallbang_iconpack <offline-png-directory>");
    let files:Vec<_>=std::fs::read_dir(folder).expect("read input directory").map(|e|e.unwrap().path())
        .filter(|p|p.extension().is_some_and(|e|e=="png")).collect();
    let files=Arc::new(files);let next=AtomicUsize::new(0);let before=AtomicU64::new(0);let after=AtomicU64::new(0);let errors=AtomicUsize::new(0);
    std::thread::scope(|scope| {
        for _ in 0..4 {
            let (files,next,before,after,errors)=(files.clone(),&next,&before,&after,&errors);
            scope.spawn(move|| loop {
                let index=next.fetch_add(1,Ordering::Relaxed);if index>=files.len() {break;}
                match optimize(&files[index]) {
                    Ok((a,b))=>{before.fetch_add(a,Ordering::Relaxed);after.fetch_add(b,Ordering::Relaxed);},
                    Err(e)=>{eprintln!("{}: {e}",files[index].display());errors.fetch_add(1,Ordering::Relaxed);}
                }
            });
        }
    });
    println!("files={} before={} after={} errors={}",files.len(),before.load(Ordering::Relaxed),after.load(Ordering::Relaxed),errors.load(Ordering::Relaxed));
    if errors.load(Ordering::Relaxed)!=0 {std::process::exit(1);}
}
