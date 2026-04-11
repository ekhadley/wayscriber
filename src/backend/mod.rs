use anyhow::Result;

pub mod wayland;

// Removed: Backend trait - no longer needed with single backend
// Removed: BackendChoice enum - Wayland is the only backend

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExitAfterCaptureMode {
    Auto,
    Always,
    Never,
}

/// Run Wayland backend with full event loop
pub fn run_wayland(
    initial_mode: Option<String>,
    freeze_on_start: bool,
    windowed: bool,
    exit_after_capture_mode: ExitAfterCaptureMode,
) -> Result<()> {
    let presentation_mode = if windowed {
        wayland::PresentationMode::Windowed
    } else {
        wayland::PresentationMode::Overlay
    };
    let mut backend = wayland::WaylandBackend::new(
        initial_mode,
        freeze_on_start,
        presentation_mode,
        exit_after_capture_mode,
    )?;
    backend.init()?;
    backend.show()?;
    backend.hide()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    #[ignore]
    fn wayland_backend_smoke_test() {
        if std::env::var("WAYLAND_DISPLAY").is_err() {
            eprintln!("WAYLAND_DISPLAY not set; skipping Wayland smoke test");
            return;
        }
        super::run_wayland(None, false, false, super::ExitAfterCaptureMode::Never)
            .expect("Wayland backend should start");
    }
}
