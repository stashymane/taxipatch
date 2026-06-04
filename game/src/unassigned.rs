use crate::locator::{ModuleLocator, PtrLocator};
use retour_utils::hook_module;

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
impl Global {
    pub const DW_CREATION_WIDTH: PtrLocator<u32> =
        PtrLocator::offset(ModuleLocator::Current, 0x001ec5f8);
    pub const DW_CREATION_HEIGHT: PtrLocator<u32> =
        PtrLocator::offset(ModuleLocator::Current, 0x001ec5fc);

    pub const GAME_STAGE: PtrLocator<u32> = PtrLocator::offset(ModuleLocator::Current, 0x003bc330);
    pub const GAME_SUBSTAGE: PtrLocator<u32> =
        PtrLocator::offset(ModuleLocator::Current, 0x003bc334);

    pub const BOOT_LOGO_FRAME_COUNTER: PtrLocator<i32> =
        PtrLocator::offset(ModuleLocator::Current, 0x00317884);
}
