use static_assertions::assert_eq_size;

assert_eq_size!(PlayerInputState, [u8; 0xcc]);

/// Bits in [`PlayerInputState::held_directions`] / `pressed_directions`.
pub mod direction_bits {
    pub const UP: u16 = 0x1;
    pub const DOWN: u16 = 0x2;
    pub const LEFT: u16 = 0x4;
    pub const RIGHT: u16 = 0x8;
}

/// Bits in [`PlayerInputState::held_buttons`] / `pressed_buttons`.
pub mod button_bits {
    pub const REVERSE: u16 = 0x1;
    pub const DRIVE: u16 = 0x2;
    pub const DEST_REMINDER: u16 = 0x4;
    pub const JUMP: u16 = 0x8;
    pub const ARROW_MODE: u16 = 0x20;
    pub const HEADLIGHTS: u16 = 0x200;
    pub const ENTER: u16 = 0x1000;
}

#[repr(C)]
pub struct PlayerInputState {
    pub connected: i16,
    _pad_0x2: u16,
    pub held_directions: u16,
    pub held_buttons: u16,
    pub analog0: u16,
    pub analog1: u16,
    pub pressed_directions: u16,
    pub pressed_buttons: u16,
    pub analog2: u16,
    pub analog3: u16,
    _pad_0x14: [u8; 6],
    /// Merged keyboard and controller brake intensity.
    pub brake_analog: u8,
    /// Merged keyboard and controller acceleration intensity.
    pub accelerate_analog: u8,
    pub analog_x: i16,
    pub analog_y: i16,
    pub analog4: u16,
    pub analog5: u16,
    _reserved: [u8; 168],
}
