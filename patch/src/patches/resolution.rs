use crate::data::PatchContext;
use crate::data::Settings;
use crate::data::game::{MultisamplingMode, WindowMode};
use crate::patch::Patch;
use crate::windows::display::get_display_info;
use game::libs::user32::User32;
use game::{CD3DApplication, Global};
use std::ptr::null_mut;
use windows::Win32::UI::WindowsAndMessaging::{HCURSOR, WM_SETCURSOR, WS_POPUP, WS_VISIBLE};

inventory::submit! {
    Patch::new("resolution", initialize)
}

#[derive(Debug, Copy, Clone)]
struct ResolutionPatchState {
    resolution_x: u32,
    resolution_y: u32,
    refresh_rate: Option<u32>,
    window_mode: WindowMode,
    multisampling: MultisamplingMode,
}

impl ResolutionPatchState {
    fn from(settings: &Settings) -> anyhow::Result<Self> {
        let default = get_display_info();

        let (resolution_x, resolution_y) = settings
            .game
            .resolution_tuple(|| (default.width, default.height))?;

        Ok(Self {
            resolution_x,
            resolution_y,
            refresh_rate: settings.game.refresh_rate,
            window_mode: settings.game.mode.unwrap_or_default(),
            multisampling: settings.game.multisampling.unwrap_or_default(),
        })
    }

    fn apply_resolution(&self, app: &mut CD3DApplication) {
        app.initial_window_width = self.resolution_x;
        app.initial_window_height = self.resolution_y;
        unsafe {
            Global::DW_CREATION_WIDTH.write(self.resolution_x);
            Global::DW_CREATION_HEIGHT.write(self.resolution_y);
        }
    }

    fn apply_window_mode(&self, app: &mut CD3DApplication) {
        match self.window_mode {
            WindowMode::Fullscreen => {
                app.use_fullscreen_mode = true;
            }
            WindowMode::Borderless | WindowMode::Windowed => {
                app.use_fullscreen_mode = false;
                app.is_windowed = true;
            }
        }
    }

    fn apply_present_overrides(&self, app: &mut CD3DApplication) {
        let params = &mut app.present_parameters;

        params.MultiSampleType = self.multisampling.into();
        params.MultiSampleQuality = 0;

        if !app.is_windowed {
            if let Some(refresh_rate) = self.refresh_rate {
                params.FullScreen_RefreshRateInHz = refresh_rate;
            }
        }
    }
}

pub fn initialize(ctx: &PatchContext) -> anyhow::Result<()> {
    let state = ResolutionPatchState::from(&ctx.settings)?;

    unsafe {
        CD3DApplication::init_window.hook({
            let state = state;

            move |fun, app_ptr, hinstance| {
                let app = &mut *app_ptr;
                state.apply_resolution(app);
                state.apply_window_mode(app);
                fun.call((app_ptr, hinstance))
            }
        });

        if state.window_mode == WindowMode::Borderless {
            CD3DApplication::wnd_proc_dispatcher.hook(move |fun, this, hwnd, msg, w_param, l_param| {
                let result = fun.call((this, hwnd, msg, w_param, l_param));

                if msg == WM_SETCURSOR {
                    User32::set_cursor(HCURSOR(null_mut()));
                }

                result
            });

            User32::create_window_ex_a.hook({
                let state = state;

                move |fun,
                      dw_ex_style,
                      lp_class_name,
                      lp_window_name,
                      _dw_style,
                      _x,
                      _y,
                      _n_width,
                      _n_height,
                      h_wnd_parent,
                      h_menu,
                      h_instance,
                      lp_param| {
                    fun.call((
                        dw_ex_style,
                        lp_class_name,
                        lp_window_name,
                        WS_VISIBLE | WS_POPUP,
                        0,
                        0,
                        state.resolution_x as i32,
                        state.resolution_y as i32,
                        h_wnd_parent,
                        h_menu,
                        h_instance,
                        lp_param,
                    ))
                }
            });
        }

        CD3DApplication::build_present_params.hook({
            let state = state;

            move |fun, app_ptr| {
                fun.call((app_ptr,));
                state.apply_present_overrides(&mut *app_ptr);
            }
        });
    }

    Ok(())
}
