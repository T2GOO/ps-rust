// tools/brush.rs — Outil pinceau
use eframe::egui;
use super::{Tool, ToolContext};

pub struct BrushTool {
    pub color: [u8; 4],
    pub size: u32,
    pub hardness: f32,   // 0.0 (soft) .. 1.0 (hard)
    last_pos: Option<egui::Vec2>,
}

impl BrushTool {
    pub fn new() -> Self {
        Self {
            color: [0, 0, 0, 255],
            size: 10,
            hardness: 0.8,
            last_pos: None,
        }
    }

    /// Dessine un cercle plein sur la layer à la position donnée
    fn stamp(&self, ctx: &mut ToolContext) {
        let r = self.size as i32 / 2;
        let cx = ctx.doc_pos.x as i32;
        let cy = ctx.doc_pos.y as i32;

        for dy in -r..=r {
            for dx in -r..=r {
                let dist = ((dx * dx + dy * dy) as f32).sqrt();
                if dist > r as f32 { continue; }

                // Soft brush : atténuation selon la distance
                let alpha_factor = if self.hardness >= 1.0 {
                    1.0
                } else {
                    let edge = r as f32 * self.hardness;
                    if dist <= edge {
                        1.0
                    } else {
                        1.0 - (dist - edge) / (r as f32 - edge)
                    }
                };

                let alpha = (self.color[3] as f32 * alpha_factor * ctx.pressure) as u8;
                let px = (cx + dx) as u32;
                let py = (cy + dy) as u32;

                ctx.layer.put_pixel(px, py, [
                    self.color[0],
                    self.color[1],
                    self.color[2],
                    alpha,
                ]);
            }
        }
    }

    /// Interpolation linéaire entre deux positions (évite les "trous" à vitesse rapide)
    fn stroke_line(&self, from: egui::Vec2, to: egui::Vec2, ctx: &mut ToolContext) {
        let dist = (to - from).length();
        let steps = (dist / (self.size as f32 * 0.3)).max(1.0) as u32;

        for i in 0..=steps {
            let t = i as f32 / steps as f32;
            ctx.doc_pos = from + (to - from) * t;
            self.stamp(ctx);
        }
    }
}

impl Tool for BrushTool {
    fn name(&self) -> &str { "Brush" }
    fn icon(&self) -> &str { "🖌" }

    fn on_press(&mut self, ctx: &mut ToolContext) {
        self.last_pos = Some(ctx.doc_pos);
        self.stamp(ctx);
    }

    fn on_drag(&mut self, ctx: &mut ToolContext) {
        if let Some(from) = self.last_pos {
            let to = ctx.doc_pos;
            self.stroke_line(from, to, ctx);
            self.last_pos = Some(to);
        }
    }

    fn on_release(&mut self, ctx: &mut ToolContext) {
        self.last_pos = None;
    }
}
