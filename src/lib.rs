/*
 * smartcalc-app v1.0.8
 * Copyright (c) Erhan BARIS (Ruslan Ognyanov Asenov)
 * Licensed under the GNU General Public License v2.0.
 */

mod http;
mod app;
mod result;
mod code;
mod calculation;
mod highlighter;
mod scroll;
mod config;
mod toggle_switch;
mod query;
mod settings;

#[cfg(target_arch = "wasm32")]
use eframe::wasm_bindgen::{self, prelude::*};

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn start(canvas_id: &str) -> Result<(), eframe::wasm_bindgen::JsValue> {
    use crate::app::SmartcalcApp;

    console_error_panic_hook::set_once();

    tracing_wasm::set_as_global_default();
    tracing::debug!("Started");

    let app = SmartcalcApp::default();
    eframe::start_web(canvas_id, Box::new(app))
}
