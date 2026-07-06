use crate::data::PatchContext;

type PatchInit = fn(&PatchContext) -> anyhow::Result<()>;

#[derive(Debug)]
pub struct Patch {
    pub name: &'static str,
    pub register: PatchInit,
    pub priority: i32,
    pub enabled: bool,
}

impl Patch {
    pub const fn new(name: &'static str, register: PatchInit) -> Self {
        Self {
            name,
            priority: 0,
            register,
            enabled: true,
        }
    }

    pub const fn enabled(mut self, value: bool) -> Self {
        self.enabled = false;
        self
    }

    pub const fn priority(mut self, value: i32) -> Self {
        self.priority = value;
        self
    }
}

inventory::collect!(Patch);
