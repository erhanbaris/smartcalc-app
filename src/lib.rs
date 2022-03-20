use eframe::{egui::{self, FontDefinitions, FontData, Button, Widget, RichText}, epi, epaint::{FontFamily, FontId, Color32}};
use chrono_tz::Tz;
use chrono_tz::OffsetName;
use chrono::{TimeZone, Local};

#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct SmartcalcApp {
    #[cfg_attr(feature = "persistence", serde(skip))]
    content: String,
    
    #[cfg_attr(feature = "persistence", serde(skip))]
    outputs: Vec<String>,
    
    smartcalc: SmartCalc
}

impl Default for SmartcalcApp {
    fn default() -> Self {
        let timezone = match localzone::get_local_zone() {
            Some(tz) => match tz.parse::<Tz>() {
                Ok(tz) => {
                    let date_time = Local::today().naive_local();
                    tz.offset_from_utc_date(&date_time).abbreviation().to_string()
                },
                Err(_) => "UTC".to_string()
            },
            None => "UTC".to_string()
        };
    
        let mut app = SmartCalc::default();
        
        app.set_decimal_seperator(",".to_string());
        app.set_thousand_separator(".".to_string());
        app.set_timezone(timezone).unwrap();

        Self {
            content: "".to_owned(),
            outputs: Vec::new(),
            smartcalc: app
        }
    }
}

impl epi::App for SmartcalcApp {
    fn name(&self) -> &str {
        "eframe template"
    }

    fn setup(&mut self, ctx: &egui::Context, _frame: &epi::Frame, _storage: Option<&dyn epi::Storage>) {
        #[cfg(feature = "persistence")]
        if let Some(storage) = _storage {
            *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
        }

        let mut font = FontDefinitions::default();
        font.font_data.insert("TitilliumWeb".to_owned(),FontData::from_static(include_bytes!("../fonts/TitilliumWeb-Regular.ttf")));
        ctx.set_fonts(font)
    }

    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }
    
    fn update(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
        let Self { content, outputs, smartcalc } = self;

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("SmartCalc", |ui| {
                    if ui.button("About").clicked() {
                        
                    }

                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });

                Button::new(RichText::new("🔄 Update Currencies").color(Color32::WHITE))
                    .fill(Color32::from_rgb(33, 133, 208))
                    .ui(ui);
            });
        });

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                egui::warn_if_debug_build(ui);

                let progress_bar_len = 10;
                let progress = 0.5;
                ui.label(format!(
                    "Download Status: {}",
                    (0..progress_bar_len)
                        .map(|i| {
                            let percent = i as f32 / progress_bar_len as f32;
                            if percent < progress {
                                '◼'
                            } else {
                                '◻'
                            }
                        })
                        .collect::<String>()
                ));

                ui.spacing_mut().item_spacing.x = 0.0;
                ui.label("powered by ");
                ui.hyperlink_to("SmartCalc Engine", "https://github.com/erhanbaris/smartcalc");
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {

            ui.columns(2, |columns| {
                /* Left panel */
                let my_frame = egui::containers::Frame {
                    margin: egui::style::Margin { left: 10., right: 5., top: 5., bottom: 5. },
                    rounding: egui::Rounding { nw: 1.0, ne: 1.0, sw: 1.0, se: 1.0 },
                    shadow: eframe::epaint::Shadow { extrusion: 1.0, color: egui::Color32::from_rgb(53, 62, 80) },
                    fill: Color32::from_rgb(31, 36, 48),
                    stroke: egui::Stroke::new(0.0, Color32::from_rgb(53, 62, 80)),
                };
                
                egui::CentralPanel::default().frame(my_frame).show_inside(&mut columns[0], |ui| {
                    ui.heading("Calculation");

                    let text = ui.add(egui::TextEdit::multiline(content)
                        .frame(false)
                        .desired_width(f32::INFINITY)
                        .desired_rows(10)
                        .font(FontId::new(25.0, FontFamily::Proportional)));
                    
                    if text.changed() {
                        let results = smartcalc.execute("en", &content[..]);
                        outputs.clear();
            
                        for result in results.lines.iter() {
                            match result {
                                Some(result) => match &result.result {
                                    Ok(line) => { outputs.push(line.output.to_string()); },
                                    Err(_) => { outputs.push("".to_string()); }
                                },
                                None => { outputs.push("".to_string()); }
                            }
                        }
                    }
                });

                /* Right panel */
                let my_frame = egui::containers::Frame {
                    margin: egui::style::Margin { left: 10., right: 5., top: 5., bottom: 5. },
                    rounding: egui::Rounding { nw: 1.0, ne: 1.0, sw: 1.0, se: 1.0 },
                    shadow: eframe::epaint::Shadow { extrusion: 1.0, color: egui::Color32::from_rgb(53, 62, 80) },
                    fill: Color32::from_rgb(53, 62, 80),
                    stroke: egui::Stroke::new(0.0, Color32::from_rgb(53, 62, 80)),
                };
                
                egui::CentralPanel::default().frame(my_frame).show_inside(&mut columns[1], |ui| {
                    ui.heading("Results");
                    ui.add(egui::TextEdit::multiline(&mut outputs.join("\r\n"))
                        .frame(false)
                        .desired_width(f32::INFINITY)
                        .interactive(false)
                        .desired_rows(10)
                        .text_color(egui::Color32::from_rgb(205, 255, 0))
                        .font(FontId::new(25.0, FontFamily::Proportional)));
                });
            });
        });
    }
}

#[cfg(target_arch = "wasm32")]
use eframe::wasm_bindgen::{self, prelude::*};
use smartcalc::SmartCalc;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn start(canvas_id: &str) -> Result<(), eframe::wasm_bindgen::JsValue> {
    console_error_panic_hook::set_once();

    tracing_wasm::set_as_global_default();

    let app = SmartcalcApp::default();
    eframe::start_web(canvas_id, Box::new(app))
}
