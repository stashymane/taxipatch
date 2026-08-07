use core::ffi::c_void;
use retour_utils::hook_impl;

#[repr(C)]
pub struct Render {}

#[hook_impl]
impl Render {
    /// `g_d3dDevice` used by the renderer (global version of `CD3DApplication::d3d9device`).
    #[ptr(offset = 0x00314e74)]
    pub const D3D_DEVICE: *mut c_void = std::ptr::null_mut();

    /// Batches queued 2D sprites/fonts to the device.
    #[hook(unsafe extern "thiscall" SpriteRenderQueue_Flush, offset = 0x00024990, chain)]
    pub fn flush_sprites(this: *mut c_void) {
        unsafe { SpriteRenderQueue_Flush.call(this) }
    }
}
