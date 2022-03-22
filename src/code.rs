use eframe::egui;
use eframe::{epaint::{Color32, FontId, FontFamily}, egui::RichText};
use smartcalc::{UiToken, SmartCalc};

use crate::calculation::Calculation;
use crate::highlighter::MemoizedHighlighter;

#[derive(Default)]
pub struct CodePanel {
    highlighter: MemoizedHighlighter
}

impl CodePanel {
    fn calculate_and_format(&mut self, egui_style: &egui::Style, code: &str, outputs: &mut Vec<String>, smartcalc: &SmartCalc) -> egui::text::LayoutJob {
        let Self { highlighter } = self;
        let mut ui_tokens = Vec::new();

        if highlighter.is_dirty(egui_style, code) {

            tracing::warn!("Calculate: {}", &code);
            let results = smartcalc.execute("en", &code[..]);
            outputs.clear();

            for result in results.lines.iter() {
                match result {
                    Some(result) => {
                        ui_tokens.append(&mut result.ui_tokens.to_vec());
                        match &result.result {
                            Ok(line) => { 
                                outputs.push(line.output.to_string());},
                            Err(_) => { outputs.push("".to_string()); }
                        }
                    },
                    None => { outputs.push("".to_string()); }
                }
            }
        }
        
        highlighter.highlight(egui_style, code, &ui_tokens)
    }

    pub fn ui(&mut self, ui: &mut egui::Ui, calculation: &mut Calculation) {
        let Calculation {code, outputs, smartcalc} = calculation;

        let frame = egui::containers::Frame {
            margin: egui::style::Margin { left: 10., right: 5., top: 5., bottom: 5. },
            rounding: egui::Rounding { nw: 1.0, ne: 1.0, sw: 1.0, se: 1.0 },
            shadow: eframe::epaint::Shadow { extrusion: 1.0, color: egui::Color32::from_rgb(53, 62, 80) },
            fill: Color32::from_rgb(31, 36, 48),
            stroke: egui::Stroke::new(0.0, Color32::from_rgb(53, 62, 80)),
        };
        
        egui::CentralPanel::default().frame(frame).show_inside(ui, |ui| {
            ui.heading(RichText::new("Calculation").color(Color32::WHITE));
            let mut layouter = |ui: &egui::Ui, string: &str, wrap_width: f32| {
                let mut layout_job = self.calculate_and_format(ui.style(), string, outputs, smartcalc);
                layout_job.wrap_width = wrap_width;
                ui.fonts().layout_job(layout_job)
            };
            
            egui::TextEdit::multiline(code)
                .frame(false)
                .desired_width(f32::INFINITY)
                .desired_rows(10)
                .font(FontId::new(20.0, FontFamily::Proportional))
                .layouter(&mut layouter)
                .show(ui);
        });
    }
}