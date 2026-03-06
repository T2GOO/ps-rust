// engine/layer.rs
use std::sync::atomic::{AtomicU32, Ordering};

static LAYER_ID_COUNTER: AtomicU32 = AtomicU32::new(0);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LayerId(u32);

impl LayerId {
    fn next() -> Self {
        Self(LAYER_ID_COUNTER.fetch_add(1, Ordering::Relaxed))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LayerBlendMode {
    Normal,
    Multiply,
    Screen,
    Overlay,
}

/// Une layer = un buffer RGBA + ses métadonnées
pub struct Layer {
    pub id: LayerId,
    pub name: String,
    pub visible: bool,
    pub opacity: f32,        // 0.0..=1.0
    pub blend_mode: LayerBlendMode,
    pub pixels: Vec<u8>,     // RGBA, row-major
    pub width: u32,
    pub height: u32,
    /// Dirty flag : true si les pixels ont changé depuis le dernier upload GPU
    pub dirty: bool,
}

impl Layer {
    pub fn new_fill(width: u32, height: u32, color: [u8; 4], name: &str) -> Self {
        let pixel_count = (width * height) as usize;
        let mut pixels = Vec::with_capacity(pixel_count * 4);
        for _ in 0..pixel_count {
            pixels.extend_from_slice(&color);
        }
        Self {
            id: LayerId::next(),
            name: name.to_string(),
            visible: true,
            opacity: 1.0,
            blend_mode: LayerBlendMode::Normal,
            pixels,
            width,
            height,
            dirty: true,
        }
    }

    pub fn new_transparent(width: u32, height: u32, name: &str) -> Self {
        Self::new_fill(width, height, [0, 0, 0, 0], name)
    }

    /// Blend this layer onto a destination buffer (alpha compositing)
    pub fn blend_onto(&self, dst: &mut [u8], dst_w: u32, dst_h: u32) {
        let w = self.width.min(dst_w) as usize;
        let h = self.height.min(dst_h) as usize;

        for y in 0..h {
            for x in 0..w {
                let src_idx = (y * self.width as usize + x) * 4;
                let dst_idx = (y * dst_w as usize + x) * 4;

                let sa = (self.pixels[src_idx + 3] as f32 / 255.0) * self.opacity;
                let da = dst[dst_idx + 3] as f32 / 255.0;

                // Alpha compositing "over"
                let out_a = sa + da * (1.0 - sa);

                if out_a < f32::EPSILON {
                    continue;
                }

                for c in 0..3 {
                    let src_c = self.pixels[src_idx + c] as f32 / 255.0;
                    let dst_c = dst[dst_idx + c] as f32 / 255.0;

                    let blended = match self.blend_mode {
                        LayerBlendMode::Normal => src_c,
                        LayerBlendMode::Multiply => src_c * dst_c,
                        LayerBlendMode::Screen => 1.0 - (1.0 - src_c) * (1.0 - dst_c),
                        LayerBlendMode::Overlay => {
                            if dst_c < 0.5 {
                                2.0 * src_c * dst_c
                            } else {
                                1.0 - 2.0 * (1.0 - src_c) * (1.0 - dst_c)
                            }
                        }
                    };

                    let out_c = (blended * sa + dst_c * da * (1.0 - sa)) / out_a;
                    dst[dst_idx + c] = (out_c * 255.0).clamp(0.0, 255.0) as u8;
                }
                dst[dst_idx + 3] = (out_a * 255.0).clamp(0.0, 255.0) as u8;
            }
        }
    }

    /// Dessine un pixel sur la layer (bounds check inclus)
    pub fn put_pixel(&mut self, x: u32, y: u32, color: [u8; 4]) {
        if x >= self.width || y >= self.height {
            return;
        }
        let idx = ((y * self.width + x) * 4) as usize;
        self.pixels[idx..idx + 4].copy_from_slice(&color);
        self.dirty = true;
    }
}
