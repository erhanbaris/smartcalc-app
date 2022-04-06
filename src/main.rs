/*
 * smartcalc-app v1.0.8
 * Copyright (c) Erhan BARIS (Ruslan Ognyanov Asenov)
 * Licensed under the GNU General Public License v2.0.
 */

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
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

#[cfg(not(target_arch = "wasm32"))]
pub fn main() {
    use crate::app::SmartcalcApp;
    
    let icon = image::open("./assets/smartcalc.png").expect("Failed to open icon path").to_rgba8();
    let (icon_width, icon_height) = icon.dimensions();
    
    let options = eframe::NativeOptions {
        icon_data: Some(eframe::epi::IconData {
            rgba: icon.into_raw(),
            width: icon_width,
            height: icon_height,
        }),
        ..Default::default()
    };

    tracing_subscriber::fmt::init();
    let app = SmartcalcApp::default();

    tracing::debug!("Started");
    eframe::run_native(Box::new(app), options);
}
