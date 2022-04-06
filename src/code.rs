/*
 * smartcalc-app v1.0.8
 * Copyright (c) Erhan BARIS (Ruslan Ognyanov Asenov)
 * Licensed under the GNU General Public License v2.0.
 */

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
    fn calculate_and_format(&mut self, code: &str, outputs: &mut Vec<Result<String, String>>, smartcalc: &SmartCalc, recalculate: &mut bool) -> egui::text::LayoutJob {
        let Self { highlighter, .. } = self;
        let mut ui_tokens = Vec::new();

        if highlighter.is_dirty(code) || *recalculate {
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
        
        let output = highlighter.highlight(code, &ui_tokens, *recalculate);
        *recalculate = false;
        output
    }

    pub fn ui(&mut self, ui: &mut egui::Ui, calculation: &mut Calculation, state: &mut State) {
        let Calculation {code, outputs, smartcalc} = calculation;
        let State {scroll, cursor, recalculate, ..} = state;

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
                let mut layout_job = self.calculate_and_format(string, outputs, smartcalc, recalculate);
                layout_job.wrap_width = wrap_width;
                ui.fonts().layout_job(layout_job)
            };

            let output = ScrollArea::vertical()
                .id_source("source")
                .show(ui, |ui| {
                    *cursor = egui::TextEdit::multiline(code)
                        .frame(false)
                        .desired_width(f32::INFINITY)
                        .font(FontId::new(30.0, FontFamily::Name("Quicksand".into())))
                        .layouter(&mut layouter)
                        .show(ui).cursor_range;
            });
            
            if &output.state.offset != scroll {
                *scroll = output.state.offset;
            }
        });
    }
}