// renderer/mod.rs — Renderer GPU : gère la texture egui du canvas
use eframe::egui;
use egui::TextureHandle;

/// Le renderer tient une texture egui mise à jour depuis le CPU
pub struct CanvasRenderer {
    /// Handle egui de la texture du document composité
    texture: Option<TextureHandle>,
    /// Dernière taille uploadée (pour détecter un resize)
    last_size: (u32, u32),
}

impl CanvasRenderer {
    pub fn new() -> Self {
        Self {
            texture: None,
            last_size: (0, 0),
        }
    }

    /// Upload un buffer RGBA vers la texture egui
    /// Optimisation : appeler seulement si dirty == true
    pub fn upload_pixels(
        &mut self,
        ctx: &egui::Context,
        pixels: &[u8],
        width: u32,
        height: u32,
    ) {
        let size = [width as usize, height as usize];
        let color_image = egui::ColorImage::from_rgba_unmultiplied(size, pixels);

        if self.texture.is_none() || self.last_size != (width, height) {
            // Première création ou resize : alloue une nouvelle texture
            self.texture = Some(ctx.load_texture(
                "canvas_texture",
                color_image,
                egui::TextureOptions::NEAREST, // NEAREST pour le pixel art, LINEAR pour les photos
            ));
            self.last_size = (width, height);
        } else if let Some(tex) = &mut self.texture {
            // Update incremental (même taille) — evite une réallocation GPU
            tex.set(color_image, egui::TextureOptions::NEAREST);
        }
    }

    /// Retourne le TextureId pour l'affichage dans egui
    pub fn texture_id(&self) -> Option<egui::TextureId> {
        self.texture.as_ref().map(|t| t.id())
    }

    /// Retourne la taille de la texture (utile pour calculer le rect d'affichage)
    pub fn size(&self) -> (u32, u32) {
        self.last_size
    }
}

/// Calcule le rect d'affichage du document dans la zone canvas d'egui,
/// en tenant compte du pan/zoom du Canvas viewport
pub fn compute_display_rect(
    available_rect: egui::Rect,
    doc_size: (u32, u32),
    pan: egui::Vec2,
    zoom: f32,
) -> egui::Rect {
    let w = doc_size.0 as f32 * zoom;
    let h = doc_size.1 as f32 * zoom;
    let origin = available_rect.min + pan;
    egui::Rect::from_min_size(origin, egui::vec2(w, h))
}
