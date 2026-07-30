use retour_utils::hook_impl;

#[repr(C)]
pub struct Effects {}

#[hook_impl]
impl Effects {
    #[hook(unsafe extern "stdcall" Effects_DrawDestinationBorders, offset = 0x00074520, chain)]
    pub fn draw_destination_borders() {
        unsafe { Effects_DrawDestinationBorders.call() }
    }
    
    #[hook(unsafe extern "stdcall" Effects_DrawParticles, offset = 0x00062fa0, chain)]
    pub fn draw_particles() {
        unsafe { Effects_DrawParticles.call() }
    }
}
