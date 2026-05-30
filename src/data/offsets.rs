use anyhow::Context;
use windows::core::PCSTR;
use windows::Win32::System::LibraryLoader::{GetModuleHandleA, GetProcAddress};

#[derive(Debug)]
pub struct Offsets {
    pub base: usize,

    pub user32_dll: User32DllOffsets,

    pub game_tick: usize,
    pub init_post_resolution_switch: usize,

    pub cd3d_init_window: usize,
    pub build_present_params: usize,

    pub frame_limiter_update: usize,

    pub boot_logo_sequence_update: usize,
    pub boot_logo_frame_counter: usize,

    pub set_camera_perspective: usize,
}

impl Offsets {
    pub fn get_default() -> anyhow::Result<Offsets> {
        let base = unsafe {
            GetModuleHandleA(None)
                .context("Failed to retrieve module handle")?
                .0 as usize
        };

        Ok(Offsets {
            base,

            user32_dll: User32DllOffsets::get()
                .context("Failed to fetch offsets for USER32.DLL")?,

            game_tick: base + 0x0007a5c0,

            init_post_resolution_switch: base + 0x00007a97,

            cd3d_init_window: base + 0x00028da0,
            build_present_params: base + 0x000283d0,

            frame_limiter_update: base + 0x00007d00,

            boot_logo_sequence_update: base + 0x0002e160,
            boot_logo_frame_counter: base + 0x00317884,

            set_camera_perspective: base + 0x0001ee10,
        })
    }
}

#[derive(Debug)]
pub struct User32DllOffsets {
    pub create_window_ex_a: usize,
}

impl User32DllOffsets {
    pub fn get() -> anyhow::Result<Self> {
        unsafe {
            let user32 = GetModuleHandleA(PCSTR(b"user32.dll\0".as_ptr()))?;

            Ok(Self {
                create_window_ex_a: GetProcAddress(user32, PCSTR(b"CreateWindowExA\0".as_ptr()))
                    .context("Failed to find USER32.DLL:CreateWindowExA")?
                    as usize,
            })
        }
    }
}
