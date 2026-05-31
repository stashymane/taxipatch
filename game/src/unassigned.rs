use retour_util::wrapped_detour;

wrapped_detour! {
    pub static GameTickHook: unsafe extern "stdcall" fn();
}
