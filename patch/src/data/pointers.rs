use crate::data::packaged_ptr::PackagedPtr;
use game::CD3DApplication;

#[derive(Debug)]
pub struct Pointers {
    pub cd3d_app: PackagedPtr<CD3DApplication>,

    pub dw_creation_width: PackagedPtr<u32>,
    pub dw_creation_height: PackagedPtr<u32>,

    pub game_stage: PackagedPtr<u32>,
    pub game_substage: PackagedPtr<u32>,

    pub boot_logo_frame_counter: PackagedPtr<i32>,
}

impl Pointers {
    pub fn from(base: usize) -> Self {
        Self {
            cd3d_app: PackagedPtr::new(base + 0x00314f70),

            dw_creation_width: PackagedPtr::new(base + 0x001EC5F8),
            dw_creation_height: PackagedPtr::new(base + 0x001EC5FC),

            game_stage: PackagedPtr::new(base + 0x003bc330),
            game_substage: PackagedPtr::new(base + 0x003bc334),

            boot_logo_frame_counter: PackagedPtr::new(base + 0x00317884),
        }
    }
}
