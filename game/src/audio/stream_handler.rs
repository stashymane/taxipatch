use crate::audio::StreamNode;
use retour_utils::hook_impl;
use static_assertions::assert_eq_size;
use windows::Win32::Foundation::HANDLE;

assert_eq_size!(CStreamHandler, [u8; 0x30]);

#[repr(C)]
pub struct CStreamHandler {
    vtable: *mut (),
    _0x4: u32,
    pub streams: *mut StreamNode,
    pub stream_count: u32,
    _0x10: u32,
    _0x14: u32,
    _0x18: u32,
    _0x1c: u8,
    _0x1d: u8,
    pub locked: bool,
    _unknown_0x1f: u8,
    _unknown_0x20: u32,
    pub lock_requested: bool,
    pub awaiting_unlock: bool,
    _unknown_0x26: [u8; 2],
    pub thread_handle: HANDLE,
    pub lp_thread_id: u32,
}

#[hook_impl]
impl CStreamHandler {
    #[hook(pub unsafe extern "fastcall" CStreamHandler_WorkerThread, offset = 0x000134d0, chain)]
    pub fn worker_thread(handler: *mut CStreamHandler) {
        unsafe { CStreamHandler_WorkerThread.call(handler) }
    }

    #[hook(pub unsafe extern "fastcall" CStreamHandler_CleanStreams, offset = 0x000132b0)]
    pub fn clean_streams(handler: *mut CStreamHandler) {
        unsafe { CStreamHandler_CleanStreams.call(handler) }
    }
}
