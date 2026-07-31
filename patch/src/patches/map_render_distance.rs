use crate::data::PatchContext;
use crate::patch::Patch;
use game::{Camera, Map};

const MAX_MESH_COUNT_CAP: i32 = 200;
const DEFAULT_CLIP_FAR: f32 = 10000.0;

inventory::submit! {
    Patch::new("map render distance", initialize)
}

pub fn initialize(ctx: &PatchContext) -> anyhow::Result<()> {
    let scale = ctx.settings.distance.map.unwrap_or_else(|| 2.0);

    if !scale.is_finite() || scale <= 0.0 {
        anyhow::bail!("game.map_draw_distance must be > 0 (got {scale})");
    }

    if (scale - 1.0).abs() < f32::EPSILON {
        return Ok(());
    }

    Map::update_draw_distance.hook({
        move |original| {
            original.call(());

            unsafe {
                let radius = Map::DRAW_RADIUS.read() * scale;
                Map::DRAW_RADIUS.write(radius);

                let count = ((Map::MAX_MESH_COUNT.read() as f32) * scale).ceil() as i32;
                Map::MAX_MESH_COUNT.write(count.clamp(1, MAX_MESH_COUNT_CAP));

                let lod = Map::LOD_DISTANCE.read();
                if lod > 0.0 {
                    Map::LOD_DISTANCE.write(lod * scale);
                }

                let far_clip = Camera::G_CLIP_FAR.read().max(DEFAULT_CLIP_FAR);
                Camera::G_CLIP_FAR.write(far_clip.max(radius * 1.1));
            }
        }
    });

    Ok(())
}
