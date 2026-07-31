use retour_utils::hook_impl;

#[repr(C)]
pub struct Map {}

#[hook_impl]
impl Map {
    #[ptr(offset = 0x003239d4)]
    pub const DRAW_RADIUS: f32 = 0.0;

    #[ptr(offset = 0x003239d8)]
    pub const MAX_MESH_COUNT: i32 = 0;

    #[ptr(offset = 0x00324df0)]
    pub const LOD_DISTANCE: f32 = 0.0;

    #[hook(unsafe extern "C" Map_UpdateDrawDistance, offset = 0x00047ed0, chain)]
    pub fn update_draw_distance() {
        unsafe { Map_UpdateDrawDistance.call() }
    }
}
