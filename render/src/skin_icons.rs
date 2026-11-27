// Embedded inventory icons. Decode off-thread; upload a bounded number per frame.
use std::collections::VecDeque;
use std::sync::Arc;
use parking_lot::{Condvar,Mutex};
use wallbang_config::collections::{HashMap,HashSet};
use windows::Win32::Graphics::Direct3D11::*;
use windows::Win32::Graphics::Dxgi::Common::{DXGI_FORMAT_R8G8B8A8_UNORM,DXGI_SAMPLE_DESC};

type Key=(i16,u32);
const MAX_QUEUED: usize=32;
const MAX_READY: usize=24;
const GPU_BUDGET: usize=64*1024*1024;
const UPLOADS_PER_FRAME: usize=2;

#[derive(Clone,Copy,Debug)]
pub struct IconRequest { pub definition:i16,pub paint:u32,pub rect:(f32,f32,f32,f32) }
struct Decoded {width:u32,height:u32,rgba:Vec<u8>}
#[derive(Default)]
struct Queue {
    requests:VecDeque<Key>, pending:HashSet<Key>, ready:VecDeque<(Key,Option<Decoded>)>,stop:bool,
}
struct Shared {queue:Mutex<Queue>,signal:Condvar}
struct Texture {srv:ID3D11ShaderResourceView,width:u32,height:u32,bytes:usize,last:u64}
pub struct IconCache {
    shared:Arc<Shared>,worker:Option<wallbang_core::threading::ThreadHandle>,
    textures:HashMap<Key,Texture>,failed:HashSet<Key>,bytes:usize,frame:u64,
}
impl IconCache {
    pub fn new()->Self {
        let shared=Arc::new(Shared {queue:Mutex::new(Queue::default()),signal:Condvar::new()});
        let task=shared.clone();
        let worker=wallbang_core::threading::spawn(move|| loop {
            let key={
                let mut queue=task.queue.lock();
                while !queue.stop && (queue.requests.is_empty()||queue.ready.len()>=MAX_READY) {task.signal.wait(&mut queue);}
                if queue.stop {break;}
                queue.requests.pop_front().unwrap()
            };
            let decoded=decode(key);
            let mut queue=task.queue.lock();
            if queue.stop {break;}
            queue.ready.push_back((key,decoded));
        });
        Self {shared,worker:Some(worker),textures:HashMap::default(),failed:HashSet::default(),bytes:0,frame:0}
    }
    pub fn request(&mut self,key:Key) {
        if self.worker.as_ref().is_none_or(|worker|!worker.started()) {self.failed.insert(key);return;}
        if self.textures.contains_key(&key)||self.failed.contains(&key) {return;}
        if wallbang_resources::skin_icons::png(key.0,key.1).is_none() {self.failed.insert(key);return;}
        let mut queue=self.shared.queue.lock();
        if queue.pending.len()>=MAX_QUEUED||!queue.pending.insert(key) {return;}
        queue.requests.push_back(key);self.shared.signal.notify_one();
    }
    pub unsafe fn update(&mut self,device:&ID3D11Device) {
        self.frame=self.frame.wrapping_add(1);
        for _ in 0..UPLOADS_PER_FRAME {
            let next={let mut queue=self.shared.queue.lock();let next=queue.ready.pop_front();if let Some((key,_))=&next {queue.pending.remove(key);}next};
            let Some((key,decoded))=next else {break;};
            self.shared.signal.notify_one();
            let Some(decoded)=decoded else {self.failed.insert(key);continue;};
            let size=decoded.rgba.len();
            if size>GPU_BUDGET {self.failed.insert(key);continue;}
            while self.bytes+size>GPU_BUDGET {
                let Some(oldest)=self.textures.iter().min_by_key(|(_,v)|v.last).map(|(k,_)|*k) else {break;};
                self.bytes-=self.textures.remove(&oldest).unwrap().bytes;
            }
            let desc=D3D11_TEXTURE2D_DESC {Width:decoded.width,Height:decoded.height,MipLevels:1,ArraySize:1,
                Format:DXGI_FORMAT_R8G8B8A8_UNORM,SampleDesc:DXGI_SAMPLE_DESC {Count:1,Quality:0},
                Usage:D3D11_USAGE_IMMUTABLE,BindFlags:D3D11_BIND_SHADER_RESOURCE.0 as u32,CPUAccessFlags:0,MiscFlags:0};
            let data=D3D11_SUBRESOURCE_DATA {pSysMem:decoded.rgba.as_ptr().cast(),SysMemPitch:decoded.width*4,SysMemSlicePitch:0};
            let mut texture=None;
            if device.CreateTexture2D(&desc,Some(&data),Some(&mut texture)).is_err() {self.failed.insert(key);continue;}
            let Some(texture)=texture else {self.failed.insert(key);continue;};
            let mut srv=None;
            if device.CreateShaderResourceView(&texture,None,Some(&mut srv)).is_err() {self.failed.insert(key);continue;}
            let Some(srv)=srv else {self.failed.insert(key);continue;};
            if let Some(old)=self.textures.insert(key,Texture {srv,width:decoded.width,height:decoded.height,bytes:size,last:self.frame}) {self.bytes-=old.bytes;}
            self.bytes+=size;
        }
    }
    pub fn texture(&mut self,key:Key)->Option<(ID3D11ShaderResourceView,u32,u32)> {
        let entry=self.textures.get_mut(&key)?;entry.last=self.frame;
        Some((entry.srv.clone(),entry.width,entry.height))
    }
}
impl Drop for IconCache {
    fn drop(&mut self) {
        {let mut queue=self.shared.queue.lock();queue.stop=true;queue.requests.clear();}
        self.shared.signal.notify_all();
        if let Some(worker)=self.worker.take() {worker.join();}
    }
}
fn decode(key:Key)->Option<Decoded> {
    let image=decode_embedded(key.0,key.1)?;
    let (width,height)=image.dimensions();
    if width==0||height==0||width>2048||height>2048 {return None;}
    Some(Decoded {width,height,rgba:image.into_raw()})
}
pub fn decode_embedded(definition:i16,paint:u32)->Option<image::RgbaImage> {
    let png=wallbang_resources::skin_icons::png(definition,paint)?;
    image::load_from_memory_with_format(png,image::ImageFormat::Png).ok().map(|image|image.to_rgba8())
}
pub fn fit(rect:(f32,f32,f32,f32),width:u32,height:u32)->Option<(f32,f32,f32,f32)> {
    if width==0||height==0||![rect.0,rect.1,rect.2,rect.3].iter().all(|v|v.is_finite())||rect.2<=0.0||rect.3<=0.0 {return None;}
    let scale=(rect.2/width as f32).min(rect.3/height as f32);
    let (w,h)=(width as f32*scale,height as f32*scale);
    Some((rect.0+(rect.2-w)*0.5,rect.1+(rect.3-h)*0.5,w,h))
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn preserve_icon_aspect_and_reject_invalid_viewports() {
        let (x,y,w,h)=fit((0.0,0.0,160.0,80.0),512,384).unwrap();
        assert!((x+w*0.5-80.0).abs()<0.001);
        assert!((w/h-512.0/384.0).abs()<0.001);
        assert_eq!((y,h),(0.0,80.0));
        assert!(fit((0.0,0.0,0.0,80.0),512,384).is_none());
    }
    #[test]
    fn decode_real_weapon_and_finish_from_embedded_bytes() {
        for key in [(7,0),(7,282),(507,418),(38,0)] {
            let decoded=decode(key).expect("required icon embedded");
            assert!(decoded.width>=128&&decoded.height>=64);
            assert_eq!(decoded.rgba.len(),decoded.width as usize*decoded.height as usize*4);
        }
    }
    #[test]
    fn native_worker_decodes_and_coalesces_duplicate_requests() {
        let mut cache=IconCache::new();
        cache.request((7,0));cache.request((7,0));
        let deadline=std::time::Instant::now()+std::time::Duration::from_secs(5);
        loop {
            {
                let queue=cache.shared.queue.lock();
                assert_eq!(queue.pending.len(),1);
                if let Some((key,decoded))=queue.ready.front() {
                    assert_eq!(*key,(7,0));assert!(decoded.is_some());break;
                }
            }
            assert!(std::time::Instant::now()<deadline,"native icon worker timed out");
            std::thread::sleep(std::time::Duration::from_millis(5));
        }
    }
}
