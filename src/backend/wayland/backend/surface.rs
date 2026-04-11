use anyhow::{Result, anyhow};
use log::info;
use smithay_client_toolkit::shell::{
    WaylandSurface, wlr_layer::Anchor, xdg::window::WindowDecorations,
};

use crate::app_id::runtime_app_id;

use super::super::PresentationMode;
use super::super::state::WaylandState;

const WINDOWED_MIN_WIDTH: u32 = 480;
const WINDOWED_MIN_HEIGHT: u32 = 320;

pub(super) fn create_overlay_surface(
    state: &mut WaylandState,
    qh: &wayland_client::QueueHandle<WaylandState>,
) -> Result<()> {
    match state.presentation_mode() {
        PresentationMode::Overlay => create_layer_overlay(state, qh),
        PresentationMode::Windowed => create_xdg_window(state, qh),
    }
}

fn create_layer_overlay(
    state: &mut WaylandState,
    qh: &wayland_client::QueueHandle<WaylandState>,
) -> Result<()> {
    let layer_shell = state
        .layer_shell
        .as_ref()
        .ok_or_else(|| anyhow!("wlr-layer-shell protocol unavailable; pass --windowed instead"))?;
    let wl_surface = state.compositor_state.create_surface(qh);
    let layer = state.main_surface_layer();
    info!("Creating layer shell surface in {:?} layer", layer);
    let layer_surface =
        layer_shell.create_layer_surface(qh, wl_surface, layer, Some("wayscriber"), None);

    layer_surface.set_anchor(Anchor::all());
    let desired_keyboard_mode = state.desired_keyboard_interactivity();
    layer_surface.set_keyboard_interactivity(desired_keyboard_mode);
    layer_surface.set_size(0, 0);
    layer_surface.set_exclusive_zone(-1);
    layer_surface.commit();

    state.surface.set_layer_surface(layer_surface);
    state.set_current_keyboard_interactivity(Some(desired_keyboard_mode));
    info!("Layer shell surface created");

    Ok(())
}

fn create_xdg_window(
    state: &mut WaylandState,
    qh: &wayland_client::QueueHandle<WaylandState>,
) -> Result<()> {
    let xdg_shell = state
        .xdg_shell
        .as_ref()
        .ok_or_else(|| anyhow!("xdg-shell protocol unavailable"))?;
    let wl_surface = state.compositor_state.create_surface(qh);
    info!("Creating xdg-toplevel window for windowed mode");
    let window = xdg_shell.create_window(wl_surface, WindowDecorations::RequestServer, qh);
    window.set_title("Wayscriber");
    let app_id = runtime_app_id();
    window.set_app_id(&app_id);
    window.set_min_size(Some((WINDOWED_MIN_WIDTH, WINDOWED_MIN_HEIGHT)));
    window.commit();

    state.surface.set_xdg_window(window);
    if !state.activate_xdg_window_with_startup_token_if_present() {
        state.request_xdg_activation(qh);
    }
    info!("xdg-toplevel window created");

    Ok(())
}
