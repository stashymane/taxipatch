use retour_utils::hook_impl;

#[repr(C)]
pub struct Camera {}

#[repr(C)]
pub struct CameraProjection {
    _private: [u8; 0],
}

#[hook_impl]
impl Camera {
    #[ptr(offset = 0x0031c3c8)]
    pub const G_FOV: f32 = 0.0;
    #[ptr(offset = 0x0031c420)]
    pub const G_ASPECT_RATIO: f32 = 0.0;

    #[hook(pub unsafe extern "thiscall" Camera_SetPerspective, offset = 0x0001ee10, chain)]
    pub fn set_perspective(
        camera_projection_ptr: *mut CameraProjection,
        fov: f32,
        aspect: f32,
        near_clip: f32,
        far_clip: f32,
    ) {
        unsafe {
            Camera_SetPerspective.call(camera_projection_ptr, fov, aspect, near_clip, far_clip)
        }
    }
}
