use crate::data::game::WindowMode;
use crate::data::{PackagedPtr, Settings};
use crate::data::{PatchContext, User32};
use crate::patch::Patch;
use crate::windows::display::get_display_info;
use anyhow::Context;
use game::user32::CreateWindowExAHook;
use game::{BuildPresentParamsHook, CD3DApplication, CD3DApplication_InitWindowHook};
use std::mem::transmute;
use windows::core::BOOL;
use windows::Win32::UI::WindowsAndMessaging::{WS_POPUP, WS_VISIBLE};

inventory::submit! {
    Patch {
        name: "resolution",
        priority: 0,
        register: initialize
    }
}

#[derive(Debug, Copy, Clone)]
struct ResolutionPatchState {
    resolution_x: u32,
    resolution_y: u32,
    window_mode: WindowMode,
}

impl ResolutionPatchState {
    fn from(settings: &Settings) -> anyhow::Result<Self> {
        let default = get_display_info();
        let (resolution_x, resolution_y) = match &settings.game.resolution {
            Some(resolution) => resolution
                .split('x')
                .map(|dim| dim.parse::<u32>().context("Failed to parse resolution"))
                .collect::<anyhow::Result<Vec<_>>>()
                .map(|result| (result[0], result[1])),
            None => Ok((default.width, default.height)),
        }?;

        let window_mode = settings
            .game
            .mode
            .clone()
            .unwrap_or_else(|| WindowMode::Borderless);

        Ok(Self {
            resolution_x,
            resolution_y,
            window_mode,
        })
    }
}

pub fn initialize(ctx: &PatchContext) -> anyhow::Result<()> {
    let state = ResolutionPatchState::from(&ctx.settings)?;

    unsafe {
        CD3DApplication_InitWindowHook.wrap({
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

                fun.call(app_ptr, hinstance)
            }
        })?;

        CreateWindowExAHook
            .initialize(transmute(ctx.offsets[User32::CreateWindowExA]), {
                let state = state.to_owned();

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
            })?
            .enable()?;

        BuildPresentParamsHook.wrap({
            let state = state.to_owned();

            let width_ptr = ctx.pointers.dw_creation_width;
            let height_ptr = ctx.pointers.dw_creation_height;

            move |fun, app_ptr| {
                patch_resolution_globals(width_ptr, height_ptr, &state);

                fun.call(app_ptr);

                let app: &mut CD3DApplication = &mut (*app_ptr);
                post_present_params(app, &state);
            }
        })?;
    }

    Ok(())
}

fn patch_resolution_globals(
    width: PackagedPtr<u32>,
    height: PackagedPtr<u32>,
    state: &ResolutionPatchState,
) {
    unsafe {
        width.write(state.resolution_x);
        height.write(state.resolution_y);
    }
}

fn post_present_params(app: &mut CD3DApplication, state: &ResolutionPatchState) {
    app.present_parameters.BackBufferWidth = state.resolution_x;
    app.present_parameters.BackBufferHeight = state.resolution_y;

    match state.window_mode {
        WindowMode::Fullscreen => {}
        WindowMode::Borderless | WindowMode::Windowed => {
            app.present_parameters.Windowed = BOOL(1);
            app.present_parameters.FullScreen_RefreshRateInHz = 0;
        }
    }
}
