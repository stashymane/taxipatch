use crate::audio::CAudioStream;
use static_assertions::assert_eq_size;

assert_eq_size!(StreamNode, [u8; 0xc]);

#[repr(C)]
pub struct StreamNode {
    next: *mut StreamNode,
    prev: *mut StreamNode,
    stream: *mut CAudioStream,
}
