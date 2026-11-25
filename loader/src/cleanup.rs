// Config block: unmapping is explicit only; the loader never calls it automatically,
// because freeing a mapped DLL while game threads still run would crash the process.

use windows::Win32::{
    Foundation::HANDLE,
    System::Memory::{VirtualFreeEx, MEM_RELEASE},
};

use crate::map::MappedImage;

/// Release a previously mapped image. Call this only when the target process is about to exit.
pub unsafe fn unmap(handle: HANDLE, image: &MappedImage) {
    if image.base.is_null() || image.size == 0 {
        return;
    }
    // MEM_RELEASE requires dwSize == 0.
    if let Err(error) = VirtualFreeEx(handle, image.base, 0, MEM_RELEASE) {
        eprintln!("VirtualFreeEx failed: {error}");
    }
}
