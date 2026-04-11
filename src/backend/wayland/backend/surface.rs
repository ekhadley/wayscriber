use anyhow::Result;
use log::info;
use smithay_client_toolkit::shell::{WaylandSurface, wlr_layer::Anchor};

use super::super::state::WaylandState;

pub(super) fn create_overlay_surface(
    state: &mut WaylandState,
    qh: &wayland_client::QueueHandle<WaylandState>,
) -> Result<()> {
    let layer_shell = state
        .layer_shell
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("wlr-layer-shell protocol unavailable"))?;
    let wl_surface = state.compositor_state.create_surface(qh);
    let layer = state.main_surface_layer();
    info!("Creating layer shell surface in {:?} layer", layer);
    let layer_surface = layer_shell.create_layer_surface(
        qh,
        wl_surface,
        layer,
        Some("wayscriber"),
        None, // Default output
    );

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
