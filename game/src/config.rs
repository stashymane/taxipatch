use static_assertions::assert_eq_size;

assert_eq_size!(GameSettings, [u8; 0x20]);

/// Fields CT3 actually uses after load:
/// - [`resolution`](Self::resolution) — WinMain jump table → backbuffer size
/// - [`bgm_off`](Self::bgm_off) — disables background audio
/// - [`language`](Self::language) — `mapConfigLanguage`
///
/// [`color_depth`](Self::color_depth) is loaded then overwritten to `0` in WinMain.
/// The remaining fields are likely leftovers from earlier Crazy Taxi games, not used in CT3.
#[repr(C)]
pub struct GameSettings {
    /// `1`=640×480, `2`=800×600, `3`=1024×768, `4`=1152×864, `5`=1280×960; else 640×480.
    pub resolution: i32,
    /// Default `0.5`. Never read by CT3.
    pub draw_distance: f32,
    /// Default `1`. Never read by CT3.
    pub shadows: i32,
    /// Default `1`; WinMain forces this to `0` after load.
    pub color_depth: i32,
    /// Default `1`. Never read by CT3.
    pub texture_filtering: i32,
    /// Default `1`. Never read by CT3.
    pub vertex_offsets_2d: i32,
    /// `1` blocks streaming BGM (`g_audioBlocked_`).
    pub bgm_off: i32,
    /// CFG byte → internal: `0`/other→EN, `1`→FR, `2`→DE, `3`→ES, `4`→IT, `5`→JP.
    pub language: u8,
    pub _pad: [u8; 3],
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            resolution: 1,
            draw_distance: 0.5,
            shadows: 1,
            color_depth: 1,
            texture_filtering: 1,
            vertex_offsets_2d: 1,
            bgm_off: 0,
            language: 0,
            _pad: [0, 0, 0],
        }
    }
}
