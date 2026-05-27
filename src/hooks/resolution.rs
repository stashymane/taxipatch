use crate::data::PatchContext;
use crate::log;
use retour::static_detour;
use windows::Win32::Foundation::HINSTANCE;
use windows::Win32::System::LibraryLoader::GetModuleHandleA;

type WinMainContinuationFn = unsafe extern "stdcall" fn(HINSTANCE, HINSTANCE, *mut u8, i32) -> i32;

static_detour! {
    pub static ResolutionHook: unsafe extern "stdcall" fn(HINSTANCE, HINSTANCE, *mut u8, i32) -> i32;
}

pub fn initialize(ctx: &PatchContext) -> Result<(), retour::Error> {
    if !ctx.settings.patches.resolution {
        log!("Resolution patch disabled, skipping");
        return Ok(());
    }

    let (width, height) = ctx.settings.window.resolution_u32().unwrap();

    unsafe {
        let base = GetModuleHandleA(None).unwrap().0 as usize;

        let continuation: WinMainContinuationFn =
            std::mem::transmute(base + ctx.offsets.resolution_continuation);

        let width_offset = ctx.offsets.resolution_width;
        let height_offset = ctx.offsets.resolution_height;

        ResolutionHook.initialize(
            continuation,
            move |hinstance, hprev_instance, lp_cmd_line, nshow_cmd| {
                patch_resolution(width_offset, height_offset, width, height);
                ResolutionHook.call(hinstance, hprev_instance, lp_cmd_line, nshow_cmd)
            },
        )?;

        ResolutionHook.enable()?;

        Ok(())
    }
}

fn patch_resolution(width_offset: usize, height_offset: usize, width: u32, height: u32) {
    unsafe {
        let base = GetModuleHandleA(None).unwrap().0 as usize;

        let width_ptr = (base + width_offset) as *mut u32;
        let height_ptr = (base + height_offset) as *mut u32;

        *width_ptr = width;
        *height_ptr = height;
    }
}
