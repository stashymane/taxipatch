use static_assertions::assert_eq_size;

assert_eq_size!(KeyboardSettings, [u8; 0x34]);

/// DirectInput keyboard bindings (`DIK_*` scancodes).
#[repr(C)]
pub struct KeyboardSettings {
    pub menu_up: i32,
    pub menu_down: i32,
    pub left: i32,
    pub right: i32,
    pub enter: i32,
    pub accelerate: i32,
    pub brake: i32,
    pub drive: i32,
    pub reverse: i32,
    pub jump: i32,
    pub dest_reminder: i32,
    pub headlights: i32,
    pub arrow_mode: i32,
}
