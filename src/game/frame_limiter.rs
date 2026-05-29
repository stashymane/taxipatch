use static_assertions::assert_eq_size;

assert_eq_size!(FrameLimiter, [u8; 0x2f]);

#[repr(C, packed)]
// #[derive(Debug)]
pub struct FrameLimiter {
    pub first_frame: bool,
    _pad: [u8; 3],
    pub last_elapsed: f32,
    pub fast_frame_accum: f32,
    pub fast_frame_count: u32,
    pub slow_frame_accum: f32,
    pub slow_frame_count: u32,
    pub timer: TscTimer,
    pub timing_window_frames: u32,
    pub frame_counter: u32,
    _config_stamp: u32,
    pub limiter_enabled: bool,
    pub adaptive_mode: bool,
    pub sleep_before_frame: bool,
}

assert_eq_size!(TscTimer, [u8; 0x8]);

#[repr(C, packed)]
#[derive(Debug)]
pub struct TscTimer {
    _low: u32,
    _high: u32,
}
