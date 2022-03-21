use eframe::egui;
use eframe::{epaint::{Color32, FontId, FontFamily}, egui::RichText};

#[derive(Default)]
pub struct ResultPanel;

impl ResultPanel {
    pub fn ui(&mut self, ui: &mut egui::Ui, outputs: &mut Vec<String>) {
        let frame = egui::containers::Frame {
            margin: egui::style::Margin { left: 10., right: 5., top: 5., bottom: 5. },
            rounding: egui::Rounding { nw: 1.0, ne: 1.0, sw: 1.0, se: 1.0 },
            shadow: eframe::epaint::Shadow { extrusion: 1.0, color: egui::Color32::from_rgb(53, 62, 80) },
            fill: Color32::from_rgb(53, 62, 80),
            stroke: egui::Stroke::new(0.0, Color32::from_rgb(53, 62, 80)),
        };
        
        egui::CentralPanel::default().frame(frame).show_inside(ui, |ui| {
            ui.heading(RichText::new("Results").color(Color32::WHITE));
            ui.add(egui::TextEdit::multiline(&mut outputs.join("\r\n"))
                .frame(false)
                .desired_width(f32::INFINITY)
                .interactive(false)
                .desired_rows(10)
                .text_color(egui::Color32::from_rgb(205, 255, 0))
                .font(FontId::new(25.0, FontFamily::Proportional)));
        });
    }
}
