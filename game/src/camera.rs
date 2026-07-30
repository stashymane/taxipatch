use retour_utils::hook_impl;

#[repr(C)]
pub struct Camera {}

#[hook_impl]
impl Camera {
    #[ptr(offset = 0x0031c3c0)]
    pub const G_CAMERA: *mut Camera = std::ptr::null_mut();

    #[ptr(offset = 0x0031c3c8)]
    pub const G_FOV: f32 = 0.0;

    #[ptr(offset = 0x0031c420)]
    pub const G_ASPECT_RATIO: f32 = 0.0;

    #[ptr(offset = 0x0031c42c)]
    pub const G_CLIP_NEAR: f32 = 0.0;

    #[ptr(offset = 0x0031c430)]
    pub const G_CLIP_FAR: f32 = 0.0;

    #[hook(pub unsafe extern "cdecl" Camera_Init, offset = 0x000316a0, chain)]
    pub fn init(this: *mut Camera) {
        unsafe { Camera_Init.call(this) }
    }

    #[hook(pub unsafe extern "thiscall" Camera_SetPerspective, offset = 0x0001ee10)]
    pub fn set_perspective(
        camera_ptr: *mut Camera,
        fov: f32,
        aspect: f32,
        near_clip: f32,
        far_clip: f32,
    ) {
        unsafe { Camera_SetPerspective.call(camera_ptr, fov, aspect, near_clip, far_clip) }
    }

    #[hook(pub unsafe extern "stdcall" Camera_Update, offset = 0x000329d0)]
    pub fn update() {
        unsafe { Camera_Update.call() }
    }

    #[hook(pub unsafe extern "stdcall" Camera_UpdateFromGlobals, offset = 0x00031c80, chain)]
    pub fn update_camera_from_globals() {
        unsafe { Camera_UpdateFromGlobals.call() }
    }
}
