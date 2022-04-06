use eframe::egui::{self, Layout, Label, WidgetInfo, WidgetType};
use eframe::epaint::{Vec2, Rounding, Stroke, TextShape};
use eframe::{epaint::{Color32, FontId, FontFamily}, egui::RichText};

use crate::app::State;
use crate::calculation::Calculation;
use crate::scroll::ScrollArea;

#[derive(Default)]
pub struct ResultPanel;

impl ResultPanel {

    fn draw_label(&mut self, ui: &mut egui::Ui, label: Label, state: &mut State, index: usize) {
        let (pos, text_galley, response) = label.layout_in_ui(ui).clone();
        match state.cursor {
            Some(cursor) => {
                if cursor.primary.rcursor.row == index {
                    let mut rect = ui.max_rect();
                    *rect.top_mut() = response.rect.top();
                    *rect.bottom_mut() = response.rect.bottom();
                    ui.painter().rect(rect, Rounding::from(4.0), Color32::from_rgb(44, 50, 63), Stroke::none());
                }
            },
            None => ()
        };

        response.widget_info(|| WidgetInfo::labeled(WidgetType::Label, text_galley.text()));

        if ui.is_rect_visible(response.rect) {
            let response_color = ui.style().interact(&response).text_color();

            let underline = if response.has_focus() {
                Stroke::new(1.0, response_color)
            } else {
                Stroke::none()
            };

            let override_text_color = if text_galley.galley_has_color {
                None
            } else {
                Some(response_color)
            };

            ui.painter().add(TextShape {
                pos,
                galley: text_galley.galley,
                override_text_color,
                underline,
                angle: 0.0,
            });
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui, calculation: &mut Calculation, state: &mut State) {
        let frame = egui::containers::Frame {
            margin: egui::style::Margin { left: 10., right: 5., top: 5., bottom: 5. },
            rounding: egui::Rounding { nw: 1.0, ne: 1.0, sw: 1.0, se: 1.0 },
            shadow: eframe::epaint::Shadow { extrusion: 1.0, color: egui::Color32::from_rgb(53, 62, 80) },
            fill: Color32::from_rgb(53, 62, 80),
            stroke: egui::Stroke::new(0.0, Color32::from_rgb(53, 62, 80)),
        };
        
        egui::CentralPanel::default().frame(frame).show_inside(ui, |ui| {
            let mut layout = Layout::right_to_left();
            layout = layout.with_cross_align(egui::Align::Min);

            ui.with_layout(layout, |ui| { ui.heading(RichText::new("Result").color(Color32::WHITE)) });
            ui.spacing_mut().item_spacing.y = 0.0;
            ScrollArea::vertical()
                .id_source("target")
                .vertical_scroll_offset(state.scroll[1])
                .show(ui, |ui| {
                    let button_padding = ui.spacing().button_padding;
                    ui.spacing_mut().button_padding = Vec2::ZERO;
        
                    for (index, output) in calculation.outputs.iter().enumerate() {
                        ui.with_layout(layout, |ui| {
                            ui.spacing_mut().item_spacing.y = 0.0;
                            let label = match output {
                                Ok(output) => output,
                                Err(_) => "⚠"
                            };

                            let label = Label::new(RichText::new(label).color(egui::Color32::from_rgb(205, 255, 0)).font(FontId::new(30., FontFamily::Name("Quicksand".into()))));
                            self.draw_label(ui, label, state, index);
                        });
                    }

                    ui.spacing_mut().button_padding = button_padding;
                });
        });
    }
}
