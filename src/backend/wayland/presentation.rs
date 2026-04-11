/// Whether the wayscriber surface is mounted as a fullscreen layer-shell overlay
/// or as a regular xdg-toplevel window. Determines which compositor features the
/// surface can use (passthrough, capture, output-switching, freeze) and which
/// shell-protocol path the backend takes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PresentationMode {
    #[default]
    Overlay,
    Windowed,
}

impl PresentationMode {
    pub fn is_overlay(self) -> bool {
        matches!(self, Self::Overlay)
    }

    pub fn is_windowed(self) -> bool {
        matches!(self, Self::Windowed)
    }

    pub fn allows_passthrough(self) -> bool {
        self.is_overlay()
    }

    pub fn allows_capture(self) -> bool {
        self.is_overlay()
    }

    pub fn allows_freeze(self) -> bool {
        self.is_overlay()
    }

    pub fn allows_output_switching(self) -> bool {
        self.is_overlay()
    }
}
