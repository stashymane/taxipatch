use crate::data::game::{BufferingMode, MultisamplingMode, WindowMode};
use crate::data::PatchContext;
use crate::data::Settings;
use crate::patch::Patch;
use crate::windows::display::get_display_info;
use game::libs::user32::User32;
use game::{CD3DApplication, Global};
use std::ptr::null_mut;
use windows::Win32::Graphics::Direct3D9::D3DSWAPEFFECT_DISCARD;
use windows::Win32::UI::WindowsAndMessaging::{HCURSOR, WM_SETCURSOR, WS_POPUP, WS_VISIBLE};

inventory::submit! {
    Patch::new("resolution", initialize)
}

#[derive(Debug, Copy, Clone)]
struct ResolutionPatchState {
    resolution_x: u32,
    resolution_y: u32,
    refresh_rate: u32,
    back_buffer_count: u32,
    window_mode: WindowMode,
    multisampling: MultisamplingMode,
}

impl ResolutionPatchState {
    fn from(settings: &Settings) -> anyhow::Result<Self> {
        let default = get_display_info();

        let (resolution_x, resolution_y) = settings
            .game
            .resolution_tuple(|| (default.width, default.height))?;

        let buffer_mode = settings.game.buffering_mode.unwrap_or_default();
        let back_buffer_count = match buffer_mode {
            BufferingMode::Double => 1,
            BufferingMode::Triple => 2,
        };

        Ok(Self {
            resolution_x,
            resolution_y,
            refresh_rate: settings.game.refresh_rate.unwrap_or(default.refresh_rate),
            window_mode: settings.game.mode.unwrap_or_default(),
            back_buffer_count,
            multisampling: settings.game.multisampling.unwrap_or_default(),
        })
    }
}

pub fn initialize(ctx: &PatchContext) -> anyhow::Result<()> {
    let state = ResolutionPatchState::from(&ctx.settings)?;

    unsafe {
        CD3DApplication::init_window.hook({
            let state = state.to_owned();

            move |fun, app_ptr, hinstance| {
                let app: &mut CD3DApplication = &mut (*app_ptr);

                app.initial_window_width = state.resolution_x;
                app.initial_window_height = state.resolution_y;

                match state.window_mode {
                    WindowMode::Fullscreen => {
                        app.use_fullscreen_mode = true;
                    }
                    WindowMode::Borderless | WindowMode::Windowed => {
                        app.use_fullscreen_mode = false;
                        app.is_windowed = true;
                        app.use_fallback_d3d_mode = true;
                    }
                };

                fun.call((app_ptr, hinstance))
            }
        });

        CD3DApplication::wnd_proc_dispatcher.hook({
            let is_borderless = state.window_mode == WindowMode::Borderless;

            move |fun, this, hwnd, msg, w_param, l_param| {
                let result = fun.call((this, hwnd, msg, w_param, l_param));

                if is_borderless && msg == WM_SETCURSOR {
                    User32::set_cursor(HCURSOR(null_mut()));
                }

                result
            }
        });

        User32::create_window_ex_a.hook({
            let state = state.to_owned();

            move |fun,
                  dw_ex_style,
                  lp_class_name,
                  lp_window_name,
                  dw_style,
                  x,
                  y,
                  n_width,
                  n_height,
                  h_wnd_parent,
                  h_menu,
                  h_instance,
                  lp_param| {
                let dw_style = match state.window_mode {
                    WindowMode::Fullscreen => dw_style,
                    WindowMode::Borderless => WS_VISIBLE | WS_POPUP,
                    WindowMode::Windowed => dw_style,
                };

                let (x, y) = match state.window_mode {
                    WindowMode::Fullscreen | WindowMode::Windowed => (x, y),
                    WindowMode::Borderless => (0, 0),
                };

                let (n_width, n_height) = match state.window_mode {
                    WindowMode::Fullscreen | WindowMode::Windowed => (n_width, n_height),
                    WindowMode::Borderless => {
                        (state.resolution_x as i32, state.resolution_y as i32)
                    }
                };

                fun.call((
                    dw_ex_style,
                    lp_class_name,
                    lp_window_name,
                    dw_style,
                    x,
                    y,
                    n_width,
                    n_height,
                    h_wnd_parent,
                    h_menu,
                    h_instance,
                    lp_param,
                ))
            }
        });

        CD3DApplication::build_present_params.hook({
            let state = state.to_owned();

            move |_, app_ptr| {
                let app: &mut CD3DApplication = &mut (*app_ptr);

                Global::DW_CREATION_WIDTH.write(state.resolution_x);
                Global::DW_CREATION_HEIGHT.write(state.resolution_y);

                apply_present_params(app, state);
            }
        });
    }

    Ok(())
}

fn apply_present_params(app: &mut CD3DApplication, state: ResolutionPatchState) {
    let params = &mut app.present_parameters;

    let current_settings = if app.is_windowed {
        &app.windowed_settings
    } else {
        &app.fullscreen_settings
    };

    let back_buffer_format = if state.window_mode == WindowMode::Fullscreen {
        current_settings.display_mode.Format
    } else {
        unsafe { (*current_settings.device_settings_combo).back_buffer_format }
    };

    params.hDeviceWindow = app.window_handle;
    params.Windowed = app.is_windowed.into();
    params.BackBufferCount = state.back_buffer_count;
    params.BackBufferWidth = state.resolution_x;
    params.BackBufferHeight = state.resolution_y;
    params.BackBufferFormat = back_buffer_format;
    params.SwapEffect = D3DSWAPEFFECT_DISCARD;

    params.MultiSampleType = state.multisampling.into();
    params.MultiSampleQuality = 0;

    params.EnableAutoDepthStencil = true.into();
    params.Flags = 0;
    params.AutoDepthStencilFormat = current_settings.depth_stencil_format;

    match state.window_mode {
        WindowMode::Fullscreen => {
            params.FullScreen_RefreshRateInHz = state.refresh_rate;
        }
        WindowMode::Borderless | WindowMode::Windowed => {
            params.FullScreen_RefreshRateInHz = 0;
        }
    }

    params.PresentationInterval = current_settings.present_interval;
}
