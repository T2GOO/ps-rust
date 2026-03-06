// ui/mod.rs — Application principale egui
use eframe::egui;
use egui::{CentralPanel, SidePanel, TopBottomPanel, Vec2};

use crate::engine::{Document, canvas::Canvas};
use crate::renderer::{CanvasRenderer, compute_display_rect};
use crate::tools::{ActiveTool, BrushTool, Tool, ToolContext};

pub struct PhotoshopApp {
    document: Document,
    canvas: Canvas,
    renderer: CanvasRenderer,

    active_tool: ActiveTool,
    brush: BrushTool,

    /// true si le document a été modifié depuis le dernier upload GPU
    dirty: bool,

    // État UI
    foreground_color: [u8; 4],
    show_layers_panel: bool,
}

impl PhotoshopApp {
    pub fn new(_cc: &eframe::CreationContext) -> Self {
        Self {
            document: Document::new(800, 600),
            canvas: Canvas::default(),
            renderer: CanvasRenderer::new(),
            active_tool: ActiveTool::Brush,
            brush: BrushTool::new(),
            dirty: true,
            foreground_color: [0, 0, 0, 255],
            show_layers_panel: true,
        }
    }
}

impl eframe::App for PhotoshopApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // ── 1. Menus ──────────────────────────────────────────────────────────
        TopBottomPanel::top("menubar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("New…").clicked() { /* TODO */ }
                    if ui.button("Open…").clicked() { /* TODO */ }
                    if ui.button("Save").clicked() { /* TODO */ }
                });
                ui.menu_button("Edit", |ui| {
                    if ui.button("Undo").clicked() { /* TODO */ }
                    if ui.button("Redo").clicked() { /* TODO */ }
                });
                ui.menu_button("Filter", |ui| {
                    if ui.button("Blur").clicked() {
                        if let Some(layer) = self.document.active_layer_mut() {
                            crate::engine::pixel_ops::box_blur(
                                &mut layer.pixels,
                                layer.width,
                                layer.height,
                                3,
                            );
                            layer.dirty = true;
                            self.dirty = true;
                        }
                    }
                    if ui.button("Invert").clicked() {
                        if let Some(layer) = self.document.active_layer_mut() {
                            crate::engine::pixel_ops::invert(&mut layer.pixels);
                            layer.dirty = true;
                            self.dirty = true;
                        }
                    }
                });
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(format!("{}%", self.canvas.zoom_percent()));
                });
            });
        });

        // ── 2. Toolbar gauche ─────────────────────────────────────────────────
        SidePanel::left("toolbar").exact_width(48.0).show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(8.0);
                for tool in [
                    ActiveTool::Brush,
                    ActiveTool::Eraser,
                    ActiveTool::EyeDropper,
                    ActiveTool::Move,
                    ActiveTool::Selection,
                ] {
                    let selected = self.active_tool == tool;
                    if ui.selectable_label(selected, tool.label().split_whitespace().next().unwrap_or("?"))
                        .clicked()
                    {
                        self.active_tool = tool;
                    }
                    ui.add_space(4.0);
                }

                ui.separator();

                // Couleur foreground
                let mut rgb = [
                    self.foreground_color[0] as f32 / 255.0,
                    self.foreground_color[1] as f32 / 255.0,
                    self.foreground_color[2] as f32 / 255.0,
                ];
                if ui.color_edit_button_rgb(&mut rgb).changed() {
                    self.foreground_color = [
                        (rgb[0] * 255.0) as u8,
                        (rgb[1] * 255.0) as u8,
                        (rgb[2] * 255.0) as u8,
                        255,
                    ];
                    self.brush.color = self.foreground_color;
                }
            });
        });

        // ── 3. Panneau layers droite ──────────────────────────────────────────
        if self.show_layers_panel {
            SidePanel::right("layers").min_width(200.0).show(ctx, |ui| {
                ui.heading("Layers");
                ui.separator();

                for layer in self.document.layers.iter_mut().rev() {
                    ui.horizontal(|ui| {
                        ui.checkbox(&mut layer.visible, "");
                        ui.label(&layer.name);
                    });
                }

                ui.separator();
                if ui.button("+ New Layer").clicked() {
                    let size = self.document.size;
                    self.document.layers.push(
                        crate::engine::layer::Layer::new_transparent(
                            size.width, size.height, "Layer",
                        )
                    );
                }

                // Options du brush
                ui.separator();
                ui.label("Brush");
                ui.add(egui::Slider::new(&mut self.brush.size, 1..=200).text("Size"));
                ui.add(egui::Slider::new(&mut self.brush.hardness, 0.0..=1.0).text("Hardness"));
            });
        }

        // ── 4. Zone canvas principale ─────────────────────────────────────────
        CentralPanel::default().show(ctx, |ui| {
            let available = ui.available_rect_before_wrap();

            // Upload GPU si le document a changé
            if self.dirty {
                let composite = self.document.composite();
                let size = self.document.size;
                self.renderer.upload_pixels(ctx, &composite, size.width, size.height);
                self.dirty = false;
            }

            // Affichage de la texture
            if let Some(tex_id) = self.renderer.texture_id() {
                let doc_size = self.renderer.size();
                let pan = Vec2::new(self.canvas.pan.x, self.canvas.pan.y);
                let rect = compute_display_rect(available, doc_size, pan, self.canvas.zoom);

                let painter = ui.painter_at(available);

                // Fond damier (transparence)
                painter.rect_filled(rect, 0.0, egui::Color32::WHITE);

                // Image du document
                painter.image(
                    tex_id,
                    rect,
                    egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                    egui::Color32::WHITE,
                );
            }

            // ── Events souris ──────────────────────────────────────────────────
            let response = ui.allocate_rect(available, egui::Sense::click_and_drag());

            // Zoom à la molette
            let scroll = ctx.input(|i| i.smooth_scroll_delta);
            if scroll.y != 0.0 {
                if let Some(cursor) = ctx.input(|i| i.pointer.hover_pos()) {
                    let cursor_vec = glam::Vec2::new(cursor.x, cursor.y);
                    self.canvas.zoom_at(cursor_vec, scroll.y);
                }
            }

            // Pan (clic milieu ou espace+drag)
            if response.dragged_by(egui::PointerButton::Middle) {
                let delta = response.drag_delta();
                self.canvas.pan += glam::Vec2::new(delta.x, delta.y);
            }

            // Outil actif
            if self.active_tool == ActiveTool::Brush {
                let is_pressing = response.is_pointer_button_down_on();
                let pointer_pos = ctx.input(|i| i.pointer.interact_pos());

                if (is_pressing || response.dragged()) {
                    if let Some(pos) = pointer_pos {
                        let screen = glam::Vec2::new(pos.x, pos.y);
                        let doc = self.canvas.screen_to_document(screen);

                        if let Some(layer) = self.document.active_layer_mut() {
                            let mut tool_ctx = ToolContext {
                                layer,
                                doc_pos: egui::Vec2::new(doc.x, doc.y),
                                pressure: 1.0,
                            };

                            if response.drag_started() {
                                self.brush.on_press(&mut tool_ctx);
                            } else {
                                self.brush.on_drag(&mut tool_ctx);
                            }

                            self.dirty = true;
                        }
                    }
                }

                if response.drag_stopped() {
                    if let Some(layer) = self.document.active_layer_mut() {
                        let mut tool_ctx = ToolContext {
                            layer,
                            doc_pos: egui::Vec2::ZERO,
                            pressure: 0.0,
                        };
                        self.brush.on_release(&mut tool_ctx);
                    }
                }
            }
        });

        // Demande un repaint continu (nécessaire pour les animations et la fluidité du brush)
        ctx.request_repaint();
    }
}
