use crate::data::game::{GameSettings, WindowMode};
use crate::data::PatchContext;
use crate::game::libs::user32::CreateWindowExAHook;
use crate::game::{CD3DApplication, CD3DApplication_InitWindow};
use retour::static_detour;
use std::mem::transmute;
use windows::core::BOOL;
use windows::Win32::UI::WindowsAndMessaging::{WS_POPUP, WS_VISIBLE};

static_detour! {
    pub static BuildPresentParamsHook: unsafe extern "thiscall" fn(*mut CD3DApplication);
}

pub fn initialize(ctx: &PatchContext) -> Result<(), retour::Error> {
    unsafe {
        CD3DApplication_InitWindow.initialize(transmute(ctx.offsets.cd3d_init_window), {
            let settings = ctx.settings.game.clone();
            let (width, height) = settings.resolution_u32().unwrap();

            move |app_ptr, hinstance| {
                let app: &mut CD3DApplication = &mut (*app_ptr);

                app.initial_window_width = width;
                app.initial_window_height = height;

                match settings.mode {
                    WindowMode::Fullscreen => {
                        app.use_fullscreen_mode = true;
                    }
                    WindowMode::Borderless | WindowMode::Windowed => {
                        app.use_fullscreen_mode = false;
                        app.is_windowed = true;
                        app.use_fallback_d3d_mode = true;
                    }
                };

                return CD3DApplication_InitWindow.call(app_ptr, hinstance);
            }
        })?;

        CreateWindowExAHook.initialize(transmute(ctx.offsets.user32_dll.create_window_ex_a), {
            let settings = ctx.settings.game.clone();

            move |dw_ex_style,
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
                let (width, height) = settings.resolution_u32().unwrap();

                let dw_style = match settings.mode {
                    WindowMode::Fullscreen => dw_style,
                    WindowMode::Borderless => WS_VISIBLE | WS_POPUP,
                    WindowMode::Windowed => dw_style,
                };

                let (x, y) = match settings.mode {
                    WindowMode::Fullscreen | WindowMode::Windowed => (x, y),
                    WindowMode::Borderless => (0, 0),
                };

                let (n_width, n_height) = match settings.mode {
                    WindowMode::Fullscreen | WindowMode::Windowed => (n_width, n_height),
                    WindowMode::Borderless => (width as i32, height as i32),
                };

                CreateWindowExAHook.call(
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
                )
            }
        })?;

        BuildPresentParamsHook.initialize(transmute(ctx.offsets.build_present_params), {
            let window_settings = ctx.settings.game.clone();
            let width_offset = ctx.offsets.globals.dw_creation_width;
            let height_offset = ctx.offsets.globals.dw_creation_height;

            move |app_ptr| {
                patch_resolution_globals(width_offset, height_offset, &window_settings);

                let app: &mut CD3DApplication = &mut (*app_ptr);

                pre_present_params(app, &window_settings);
                BuildPresentParamsHook.call(app_ptr);
                post_present_params(app, &window_settings);
            }
        })?;

        CD3DApplication_InitWindow.enable()?;
        CreateWindowExAHook.enable()?;
        BuildPresentParamsHook.enable()?;
    }

    Ok(())
}

fn patch_resolution_globals(width_offset: usize, height_offset: usize, settings: &GameSettings) {
    unsafe {
        let width_ptr = width_offset as *mut u32;
        let height_ptr = height_offset as *mut u32;

        let (width, height) = settings.resolution_u32().unwrap();

        *width_ptr = width;
        *height_ptr = height;
    }
}

fn pre_present_params(app: &mut CD3DApplication, settings: &GameSettings) {}

fn post_present_params(app: &mut CD3DApplication, settings: &GameSettings) {
    let (width, height) = settings.resolution_u32().unwrap();

    app.present_parameters.BackBufferWidth = width;
    app.present_parameters.BackBufferHeight = height;

    match settings.mode {
        WindowMode::Fullscreen => {}
        WindowMode::Borderless | WindowMode::Windowed => {
            app.present_parameters.Windowed = BOOL(1);
            app.present_parameters.FullScreen_RefreshRateInHz = 0;
        }
    }
}
