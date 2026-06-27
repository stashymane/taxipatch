use static_assertions::assert_eq_size;

assert_eq_size!(TscTimer, [u8; 0x8]);

#[repr(C, packed)]
#[derive(Debug)]
pub struct TscTimer {
    _low: u32,
    _high: u32,
}
