use crate::data::game::{GameSettings, WindowMode};
use crate::data::{PackagedPtr, CT3};
use crate::data::{PatchContext, User32};
use crate::game::user32::CreateWindowExAHook;
use crate::game::{BuildPresentParamsHook, CD3DApplication, CD3DApplication_InitWindowHook};
use std::mem::transmute;
use windows::core::BOOL;
use windows::Win32::UI::WindowsAndMessaging::{WS_POPUP, WS_VISIBLE};

pub fn initialize(ctx: &PatchContext) -> Result<(), retour::Error> {
    unsafe {
        CD3DApplication_InitWindowHook.initialize(
            transmute(ctx.offsets[CT3::CD3DInitWindow]),
            {
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

                    return CD3DApplication_InitWindowHook.call(app_ptr, hinstance);
                }
            },
        )?;

        CreateWindowExAHook.initialize(transmute(ctx.offsets[User32::CreateWindowExA]), {
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

        BuildPresentParamsHook.initialize(transmute(ctx.offsets[CT3::BuildPresentParams]), {
            let window_settings = ctx.settings.game.clone();
            let width = ctx.pointers.dw_creation_width;
            let height = ctx.pointers.dw_creation_height;

            move |app_ptr| {
                patch_resolution_globals(width, height, &window_settings);

                let app: &mut CD3DApplication = &mut (*app_ptr);

                BuildPresentParamsHook.call(app_ptr);
                post_present_params(app, &window_settings);
            }
        })?;

        CD3DApplication_InitWindowHook.enable()?;
        CreateWindowExAHook.enable()?;
        BuildPresentParamsHook.enable()?;
    }

    Ok(())
}

fn patch_resolution_globals(
    width: PackagedPtr<u32>,
    height: PackagedPtr<u32>,
    settings: &GameSettings,
) {
    unsafe {
        let (res_width, res_height) = settings.resolution_u32().unwrap();

        width.write(res_width);
        height.write(res_height);
    }
}

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
