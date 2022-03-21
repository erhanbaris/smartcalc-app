#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod http;
mod app;
mod result;
mod code;

#[cfg(not(target_arch = "wasm32"))]
pub fn main() {
    use crate::app::SmartcalcApp;
    let app = SmartcalcApp::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}
