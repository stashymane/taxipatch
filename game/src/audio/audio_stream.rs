use retour_utils::hook_impl;
use windows::Win32::Media::Audio::DirectSound::IDirectSound8;

#[repr(C)]
pub struct CAudioStream {}

#[hook_impl]
impl CAudioStream {
    #[hook(pub unsafe extern "thiscall" CAudioStream_Open, offset = 0x00012b10, chain)]
    pub fn open_stream(
        this: *mut CAudioStream,
        sample_rate: i32,
        channels: u32,
        buffer_size_bytes: u32,
        dsound: *mut IDirectSound8,
        filename: *mut u8,
        unknown1: u32,
        unknown2: u32,
    ) -> bool {
        unsafe {
            CAudioStream_Open.call(
                this,
                sample_rate,
                channels,
                buffer_size_bytes,
                dsound,
                filename,
                unknown1,
                unknown2,
            )
        }
    }

    #[hook(pub unsafe extern "thiscall" CreateStreamWithParams, offset = 0x000136c0, chain)]
    pub fn create_with_params(
        this: *mut (),
        sample_rate: i32,
        channels: u32,
        flags: u32,
        param_4: u32,
        dsound: *mut (),
        param_6: *mut u8,
        param_7: u32,
        param_8: u8,
        param_9: i32,
    ) -> i32 {
        unsafe {
            CreateStreamWithParams.call(
                this,
                sample_rate,
                channels,
                flags,
                param_4,
                dsound,
                param_6,
                param_7,
                param_8,
                param_9,
            )
        }
    }

    #[hook(pub unsafe extern "thiscall" CreateAndPlayStream, offset = 0x00013800, chain)]
    pub fn create_and_play(
        this: *mut (),
        sample_rate: i32,
        channels: u32,
        flags: u32,
        param_4: u32,
        dsound: *mut (),
        param_6: *mut char,
        param_7: u32,
    ) -> i32 {
        unsafe {
            CreateAndPlayStream.call(
                this,
                sample_rate,
                channels,
                flags,
                param_4,
                dsound,
                param_6,
                param_7,
            )
        }
    }
}
