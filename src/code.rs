use eframe::egui::{self, ScrollArea};
use eframe::{epaint::{Color32, FontId, FontFamily}, egui::RichText};
use smartcalc::SmartCalc;

use crate::app::State;
use crate::calculation::Calculation;
use crate::highlighter::MemoizedHighlighter;

#[derive(Default)]
pub struct CodePanel {
    highlighter: MemoizedHighlighter
}

impl CodePanel {
    fn calculate_and_format(&mut self, code: &str, outputs: &mut Vec<Result<String, String>>, smartcalc: &SmartCalc, update_smartcalc_config: &mut bool) -> egui::text::LayoutJob {
        let Self { highlighter, .. } = self;
        let mut ui_tokens = Vec::new();

        if highlighter.is_dirty(code) || *update_smartcalc_config {
            *update_smartcalc_config = false;

            let results = smartcalc.execute("en", code);
            outputs.clear();

            for result in results.lines.iter() {
                match result {
                    Some(result) => {
                        ui_tokens.push(result.ui_tokens.to_vec());
                        match &result.result {
                            Ok(line) => { 
                                outputs.push(Ok(line.output.to_string()));},
                            Err(error) => { outputs.push(Err(error.to_string())); }
                        }
                    },
                    None => { outputs.push(Ok("".to_string())); }
                }
            }
        }
        
        highlighter.highlight(code, &ui_tokens)
    }

    pub fn ui(&mut self, ui: &mut egui::Ui, calculation: &mut Calculation, state: &mut State) {
        let Calculation {code, outputs, smartcalc} = calculation;
        let State {scroll, cursor, update_smartcalc_config, ..} = state;

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
                let mut layout_job = self.calculate_and_format(string, outputs, smartcalc, update_smartcalc_config);
                layout_job.wrap_width = wrap_width;
                ui.fonts().layout_job(layout_job)
            };

            let output = ScrollArea::vertical()
                .id_source("source")
                .show(ui, |ui| {
                    *cursor = egui::TextEdit::multiline(code)
                        .frame(false)
                        .desired_width(f32::INFINITY)
                        .font(FontId::new(35.0, FontFamily::Name("Quicksand".into())))
                        .layouter(&mut layouter)
                        .show(ui).cursor_range;
            });
            
            if &output.state.offset != scroll {
                *scroll = output.state.offset;
            }
        });
    }
}