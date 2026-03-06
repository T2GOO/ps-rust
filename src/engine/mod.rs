// engine/mod.rs — Moteur image CPU
pub mod canvas;
pub mod layer;
pub mod pixel_ops;

pub use canvas::Canvas;
pub use layer::{Layer, LayerBlendMode, LayerId};

/// Résolution de travail d'un document
#[derive(Debug, Clone, Copy)]
pub struct DocumentSize {
    pub width: u32,
    pub height: u32,
}

/// État complet du document (ce qui est sauvegardé)
pub struct Document {
    pub size: DocumentSize,
    pub layers: Vec<Layer>,
    pub active_layer: LayerId,
}

impl Document {
    pub fn new(width: u32, height: u32) -> Self {
        let bg = Layer::new_fill(width, height, [255, 255, 255, 255], "Background");
        let active = bg.id;
        Self {
            size: DocumentSize { width, height },
            layers: vec![bg],
            active_layer: active,
        }
    }

    /// Flatten toutes les layers en un seul buffer RGBA
    /// Appelé par le renderer pour préparer la texture GPU
    pub fn composite(&self) -> Vec<u8> {
        let pixel_count = (self.size.width * self.size.height) as usize;
        let mut output = vec![0u8; pixel_count * 4];

        for layer in &self.layers {
            if !layer.visible {
                continue;
            }
            layer.blend_onto(&mut output, self.size.width, self.size.height);
        }

        output
    }

    pub fn active_layer_mut(&mut self) -> Option<&mut Layer> {
        let id = self.active_layer;
        self.layers.iter_mut().find(|l| l.id == id)
    }
}
