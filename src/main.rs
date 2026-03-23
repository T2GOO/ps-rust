use eframe::egui;
use eframe::wasm_bindgen::JsCast;

struct MyApp {
    count: i32,
    user_entry: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            count: 0,
            user_entry: String::from("world"),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Basic interface for LIME");

            ui.horizontal(|ui| {
                ui.label("Entry :");
                ui.text_edit_singleline(&mut self.user_entry);
            });

            ui.label(format!("Hello, {} !", self.user_entry));

            ui.separator();

            ui.horizontal(|ui| {
                if ui.button("+1").clicked() {
                    self.count += 1;
                }
                if ui.button("-1").clicked() {
                    self.count -= 1;
                }
            });


            ui.label(format!("Counter : {}", self.count));
        });
    }
}

fn main() {
    let canvas = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("egui_canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async move {
        eframe::WebRunner::new()
            .start(canvas, web_options, Box::new(|_cc| Ok(Box::new(MyApp::default()))))
            .await
            .expect("Failed to start eframe");
    });
}