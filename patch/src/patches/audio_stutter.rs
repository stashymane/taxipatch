use crate::data::PatchContext;
use crate::patch::Patch;
use game::audio::CStreamHandler;
use std::thread::sleep;
use std::time::{Duration, Instant};

inventory::submit! {
    Patch {
        name: "audio cleanup thread",
        priority: 0,
        register: initialize
    }
}

const POLL_INTERVAL: Duration = Duration::from_millis(2);

pub fn initialize(_ctx: &PatchContext) -> anyhow::Result<()> {
    CStreamHandler::worker_thread.hook({
        |fun, handler_ptr| unsafe {
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
