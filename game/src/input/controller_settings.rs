use static_assertions::assert_eq_size;

assert_eq_size!(ControllerSettings, [u8; 0x60]);

/// Controller binding config from `TAXI3.CFG`.
///
/// Bind field encoding (from CT3Config display/capture):
/// - `-1` — unbound
/// - `-2` — stick left (digital)
/// - `-3` — stick right (digital)
/// - `-4` — axis negative / accel axis
/// - `-5` — axis positive / brake axis
/// - `>= 0` — `rgbButtons` index
///
/// `axis_index_*` / `axis_*_calib_*` are dword indices into the device state blob
/// plus calibration endpoints for wheel devices (type `0x16`, subtype 2/3).
#[repr(C)]
pub struct ControllerSettings {
    pub guid: [u8; 16],
    /// Non-`-1` enables vibration (CT3Config checkbox) / FF effect in `InitDInput`.
    pub config_type: i32,
    /// Direction bind (`-1`/`-4`/…); any non-`-1` enables digital UP in the game.
    pub up_bind: i32,
    /// Direction bind; any non-`-1` enables digital DOWN.
    pub down_bind: i32,
    /// Direction bind (`-2` = stick left); also gates negative steering from `lX`.
    pub left_bind: i32,
    /// Direction bind (`-3` = stick right); also gates positive steering from `lX`.
    pub right_bind: i32,
    pub enter_btn: i32,
    pub accelerate_bind: i32,
    pub brake_bind: i32,
    pub drive_btn: i32,
    pub reverse_btn: i32,
    pub jump_btn: i32,
    pub dest_reminder_btn: i32,
    pub headlights_btn: i32,
    pub arrow_mode_btn: i32,
    pub axis_index_a: i32,
    pub axis_index_b: i32,
    pub axis_a_calib_lo: i32,
    pub axis_b_calib_lo: i32,
    pub axis_a_calib_hi: i32,
    pub axis_b_calib_hi: i32,
}
