use crate::data::PatchContext;
use crate::patch::Patch;
use game::audio::CStreamHandler;
use std::thread::sleep;
use std::time::{Duration, Instant};

inventory::submit! {
    Patch::new("audio cleanup thread", initialize)
}

const POLL_INTERVAL: Duration = Duration::from_millis(2);

pub fn initialize(_ctx: &PatchContext) -> anyhow::Result<()> {
    CStreamHandler::worker_thread.hook({
        |_, handler_ptr| {
            loop {
                let before = Instant::now();
                CStreamHandler::clean_streams(handler_ptr);

                let elapsed = before.elapsed();
                if elapsed < POLL_INTERVAL {
                    sleep(POLL_INTERVAL - elapsed);
                }
            }
        }
    });

    Ok(())
}
