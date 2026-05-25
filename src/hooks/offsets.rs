pub struct Offsets {
    pub resolution_width: usize,
    pub resolution_height: usize,
    pub resolution_continuation: usize,
}

impl Offsets {
    pub fn get_fairlight() -> Offsets {
        Offsets {
            resolution_width: 0x001EC5F8,
            resolution_height: 0x001EC5FC,
            resolution_continuation: 0x00007A97,
        }
    }
}
