use retour_utils::{hook_impl, hook_module};

#[hook_module]
pub mod hooks {
    #[hook(unsafe extern "stdcall" GameTick, offset = 0x0007a5c0, chain)]
    pub fn game_tick() {
        unsafe { GameTick.call() }
    }

    #[hook(unsafe extern "stdcall" BootLogoSequenceUpdate, offset = 0x0002e160, chain)]
    pub fn boot_logo_sequence_update() {
        unsafe { BootLogoSequenceUpdate.call() }
    }
}

pub struct Global {}

#[hook_impl]
impl Global {
    #[ptr(offset = 0x001ec5f8)]
    pub const DW_CREATION_WIDTH: u32 = 0;

    #[ptr(offset = 0x001ec5fc)]
    pub const DW_CREATION_HEIGHT: u32 = 0;

    #[ptr(offset = 0x003bc330)]
    pub const GAME_STAGE: u32 = 0;

    #[ptr(offset = 0x003bc334)]
    pub const GAME_SUBSTAGE: u32 = 0;

    #[ptr(offset = 0x00317884)]
    pub const BOOT_LOGO_FRAME_COUNTER: i32 = 0;
}
