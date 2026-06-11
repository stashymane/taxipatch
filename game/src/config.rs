use retour_utils::hook_module;
use static_assertions::assert_eq_size;

assert_eq_size!(GameSettings, [u8; 0x20]);

#[repr(C)]
pub struct GameSettings {
    pub resolution_index: i32,
    _unused_0x4: i32,
    _unused_0x8: i32,
    pub color_depth: i32,
    _pad_0x10: [u8; 0xb],
    pub language: u8, //is used as an enum, but changing it does nothing?
    _pad_0x1d: [u8; 3],
}

assert_eq_size!(KeyboardSettings, [u8; 0x34]);

#[repr(C)]
pub struct KeyboardSettings {
    pub menu_up: i32,
    pub menu_down: i32,
    pub left: i32,
    pub right: i32,
    pub enter: i32,
    pub accelerate: i32, //unsure
    pub brake: i32,
    pub gear_drive: i32,
    pub gear_reverse: i32,
    pub jump: i32,
    _unassigned: i32,
    _mode1: i32,
    _mode2: i32,
}

assert_eq_size!(ControllerSettings, [u8; 0x60]);

#[repr(C)]
pub struct ControllerSettings {
    _unknown: [u8; 0x60],
}

#[hook_module]
pub mod ct3config {
    use crate::{ControllerSettings, GameSettings, KeyboardSettings};

    #[hook(unsafe extern "C" LoadConfig, offset = 0x000077e0, chain)]
    pub fn load() {
        unsafe { LoadConfig.call() }
    }

    #[ptr(offset = 0x00314e50)]
    pub const GAME_CONFIG: GameSettings = null_mut();
    #[ptr(offset = 0x00317398)]
    pub const KEYBOARD_SETTINGS: KeyboardSettings = null_mut();
    #[ptr(offset = 0x00316ca0)]
    pub const CONTROLLER_SETTINGS: ControllerSettings = null_mut();
}
