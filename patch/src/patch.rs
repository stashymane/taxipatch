use crate::data::PatchContext;

pub struct Patch {
    pub name: &'static str,
    pub priority: i32,
    pub register: fn(&PatchContext) -> anyhow::Result<()>,
}

inventory::collect!(Patch);
