#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod http;
mod app;
mod result;
mod code;
mod calculation;
mod highlighter;
mod scroll;
mod config;

#[cfg(not(target_arch = "wasm32"))]
pub fn main() {
    use crate::app::SmartcalcApp;
    tracing_subscriber::fmt::init();
    let app = SmartcalcApp::default();
    let native_options = eframe::NativeOptions::default();

    tracing::debug!("Started");
    eframe::run_native(Box::new(app), native_options);
}
