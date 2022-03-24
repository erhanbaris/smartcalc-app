use eframe::egui::{self, Layout};
use eframe::{epaint::{Color32, FontId, FontFamily}, egui::RichText};

use crate::app::State;
use crate::calculation::Calculation;
use crate::scroll::ScrollArea;

#[derive(Default)]
pub struct ResultPanel;

impl ResultPanel {
    pub fn ui(&mut self, ui: &mut egui::Ui, calculation: &mut Calculation, state: &mut State) {
        let frame = egui::containers::Frame {
            margin: egui::style::Margin { left: 10., right: 5., top: 5., bottom: 5. },
            rounding: egui::Rounding { nw: 1.0, ne: 1.0, sw: 1.0, se: 1.0 },
            shadow: eframe::epaint::Shadow { extrusion: 1.0, color: egui::Color32::from_rgb(53, 62, 80) },
            fill: Color32::from_rgb(53, 62, 80),
            stroke: egui::Stroke::new(0.0, Color32::from_rgb(53, 62, 80)),
        };
        
        egui::CentralPanel::default().frame(frame).show_inside(ui, |ui| {
            ui.heading(RichText::new("Results").color(Color32::WHITE));
            ui.separator();

            ScrollArea::vertical()
                .id_source("target")
                .vertical_scroll_offset(state.scroll[1])
                .show(ui, |ui| {
                    let mut layout = Layout::right_to_left();
                    layout = layout.with_cross_align(egui::Align::Min);
        
                    for output in calculation.outputs.iter() {
                        ui.with_layout(layout, |ui| { 
                            ui.label(RichText::new(output)
                                .color(egui::Color32::from_rgb(205, 255, 0))
                                .font(FontId::new(32., FontFamily::Name("Quicksand".into()))));
                        });
                    }
                });
        });
    }
}
