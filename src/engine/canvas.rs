// engine/canvas.rs — Viewport : gestion pan/zoom

use glam::Vec2;

pub struct Canvas {
    /// Offset en pixels (espace écran)
    pub pan: Vec2,
    /// Facteur de zoom (1.0 = 100%)
    pub zoom: f32,
}

impl Default for Canvas {
    fn default() -> Self {
        Self {
            pan: Vec2::ZERO,
            zoom: 1.0,
        }
    }
}

impl Canvas {
    /// Convertit une position écran (egui) en position document
    pub fn screen_to_document(&self, screen_pos: Vec2) -> Vec2 {
        (screen_pos - self.pan) / self.zoom
    }

    /// Convertit une position document en position écran
    pub fn document_to_screen(&self, doc_pos: Vec2) -> Vec2 {
        doc_pos * self.zoom + self.pan
    }

    /// Zoom centré sur un point écran (ex: curseur souris)
    pub fn zoom_at(&mut self, screen_pivot: Vec2, delta: f32) {
        let factor = if delta > 0.0 { 1.1f32 } else { 1.0 / 1.1 };
        let new_zoom = (self.zoom * factor).clamp(0.05, 32.0);

        // Ajuste le pan pour que le point sous le curseur reste fixe
        self.pan = screen_pivot - (screen_pivot - self.pan) * (new_zoom / self.zoom);
        self.zoom = new_zoom;
    }

    pub fn zoom_percent(&self) -> u32 {
        (self.zoom * 100.0).round() as u32
    }
}

// engine/pixel_ops.rs — Opérations pixel basiques (filtres CPU)
