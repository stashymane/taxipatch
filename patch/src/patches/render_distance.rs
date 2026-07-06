use crate::data::PatchContext;
use crate::patch::Patch;
use game::hooks::init_map_objects;
use game::Global;

inventory::submit! {
    Patch::new("render distance", initialize)
}

pub fn initialize(_ctx: &PatchContext) -> anyhow::Result<()> {
    init_map_objects.hook({
        |this| {
            let result = this.call(());
            unsafe {
                // set object render distance based on used for CrazyX course 0x11
                Global::SET_OBJECT_RENDER_DIST_SQUARED.write(50000.0_f32.powi(2));
                Global::DETAIL_OBJECT_RENDER_DIST_SQUARED.write(50000.0);
            }
            result
        }
    });

    Ok(())
}
