// tools/mod.rs — Outils (brush, eraser, selection, etc.)
pub mod brush;

pub use brush::BrushTool;
use eframe::egui;
use egui::Vec2;
use crate::engine::layer::Layer;

/// Contexte passé à chaque outil lors d'un événement souris
pub struct ToolContext<'a> {
    pub layer: &'a mut Layer,
    /// Position en coordonnées document (pas écran)
    pub doc_pos: Vec2,
    pub pressure: f32,       // 1.0 si pas de stylus
}

/// Interface commune à tous les outils
pub trait Tool {
    fn name(&self) -> &str;
    fn on_press(&mut self, ctx: &mut ToolContext);
    fn on_drag(&mut self, ctx: &mut ToolContext);
    fn on_release(&mut self, ctx: &mut ToolContext);
    /// Icône (optionnelle, pour la toolbar)
    fn icon(&self) -> &str { "?" }
}

/// Outil actuellement sélectionné
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ActiveTool {
    Brush,
    Eraser,
    EyeDropper,
    Move,
    Selection,
}

impl ActiveTool {
    pub fn label(&self) -> &str {
        match self {
            Self::Brush     => "🖌 Brush",
            Self::Eraser    => "🧹 Eraser",
            Self::EyeDropper=> "💧 Eyedropper",
            Self::Move      => "✥ Move",
            Self::Selection => "⬚ Select",
        }
    }
}
