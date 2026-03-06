// lib.rs — Point d'entrée unique (WASM), eframe 0.33+
mod engine;
mod renderer;
mod ui;
mod tools;

use wasm_bindgen::prelude::*;

#[derive(Clone)]
#[wasm_bindgen]
pub struct WebHandle {
    runner: eframe::WebRunner,
}

#[wasm_bindgen]
impl WebHandle {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        eframe::WebLogger::init(log::LevelFilter::Debug).ok();
        Self {
            runner: eframe::WebRunner::new(),
        }
    }

    pub async fn start(
        &self,
        canvas: web_sys::HtmlCanvasElement,
    ) -> Result<(), JsValue> {
        self.runner
            .start(
                canvas,
                eframe::WebOptions::default(),
                Box::new(|cc| Ok(Box::new(ui::PhotoshopApp::new(cc)))),
            )
            .await
    }

    pub fn destroy(&self) {
        self.runner.destroy();
    }
}