use anyhow::Context;
use std::ops::Index;
use windows::core::PCSTR;
use windows::Win32::System::LibraryLoader::{GetModuleHandleA, GetProcAddress};

#[derive(Debug)]
pub struct Offsets {
    pub base: usize,
    ct3: [usize; CT3::Count as usize],
    user32: [usize; User32::Count as usize],
}

#[repr(usize)]
#[derive(Debug, Copy, Clone)]
pub enum CT3 {
    GameTick = 0,
    InitPostResolutionSwitch,
    CD3DInitWindow,
    BuildPresentParams,
    FrameLimiterUpdate,
    BootLogoSequenceUpdate,
    SetCameraPerspective,
    Count,
}

#[repr(usize)]
#[derive(Debug, Copy, Clone)]
pub enum User32 {
    CreateWindowExA = 0,
    Count,
}

impl Offsets {
    pub fn get_default(base: usize) -> anyhow::Result<Offsets> {
        let ct3 = Self::get_ct3(base);
        let user32 = Self::get_user32()?;

        Ok(Offsets { base, ct3, user32 })
    }

    fn get_ct3(base: usize) -> [usize; CT3::Count as usize] {
        let mut ct3 = [0; CT3::Count as usize];

        ct3[CT3::GameTick as usize] = 0x0007a5c0;
        ct3[CT3::InitPostResolutionSwitch as usize] = 0x00007a97;

        ct3[CT3::CD3DInitWindow as usize] = 0x00028da0;
        ct3[CT3::BuildPresentParams as usize] = 0x000283d0;
        ct3[CT3::FrameLimiterUpdate as usize] = 0x00007d00;

        ct3[CT3::BootLogoSequenceUpdate as usize] = 0x0002e160;

        ct3[CT3::SetCameraPerspective as usize] = 0x0001ee10;

        for i in 0..(CT3::Count as usize) {
            ct3[i] += base;
        }

        ct3
    }

    fn get_user32() -> anyhow::Result<[usize; User32::Count as usize]> {
        let user32_handle = unsafe { GetModuleHandleA(PCSTR(b"user32.dll\0".as_ptr()))? };

        let mut user32 = [0; User32::Count as usize];
        unsafe {
            user32[User32::CreateWindowExA as usize] =
                GetProcAddress(user32_handle, PCSTR(b"CreateWindowExA\0".as_ptr()))
                    .context("Failed to find USER32.DLL:CreateWindowExA")? as usize;
        }

        Ok(user32)
    }

    pub fn get(&self, symbol: CT3) -> usize {
        self.ct3[symbol as usize]
    }
}

impl Index<CT3> for Offsets {
    type Output = usize;

    fn index(&self, symbol: CT3) -> &Self::Output {
        &self.ct3[symbol as usize]
    }
}

impl Index<User32> for Offsets {
    type Output = usize;

    fn index(&self, symbol: User32) -> &Self::Output {
        &self.user32[symbol as usize]
    }
}
