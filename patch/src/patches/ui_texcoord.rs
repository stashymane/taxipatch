use crate::data::PatchContext;
use crate::patch::Patch;
use game::Render;
use windows::Win32::Graphics::Direct3D9::{D3DTSS_TEXCOORDINDEX, IDirect3DDevice9};
use windows::core::Interface;

inventory::submit! {
    Patch::new("ui_texcoord", initialize)
}

/// CT3 never resets `D3DTSS_TEXCOORDINDEX` when flushing sprites. Env map materials leave
/// `D3DTSS_TCI_CAMERASPACEREFLECTIONVECTOR` on the device; DXVK (and wined3d) then
/// generates UVs from that for UI draws, causing distortion
/// ([DXVK #2819](https://github.com/doitsujin/dxvk/issues/2819)).
pub fn initialize(_ctx: &PatchContext) -> anyhow::Result<()> {
    Render::flush_sprites.hook(|fun, this| {
        unsafe {
            let raw = Render::D3D_DEVICE.read();
            if raw.is_null() {
                return;
            }

            let Some(device) = IDirect3DDevice9::from_raw_borrowed(&raw) else {
                return;
            };

            for stage in 0..4 {
                let _ = device.SetTextureStageState(stage, D3DTSS_TEXCOORDINDEX, 0);
            }
        }

        fun.call((this,))
    });

    Ok(())
}
