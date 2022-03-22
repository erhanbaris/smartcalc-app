use eframe::egui;
use eframe::epaint::{FontId, FontFamily};
use smartcalc::UiToken;

#[derive(Default)]
pub struct MemoizedHighlighter {
    style: egui::Style,
    code: String,
    output: egui::text::LayoutJob
}

impl MemoizedHighlighter {
    pub fn highlight(&mut self, egui_style: &egui::Style, code: &str, ui_tokens: &Vec<UiToken>) -> egui::text::LayoutJob {
        if (&self.style, self.code.as_str()) != (egui_style, code) {
            self.style = egui_style.clone();
            self.code = code.to_owned();
            self.output = highlight(egui_style, code, ui_tokens);
        }
        self.output.clone()
    }

    pub fn is_dirty(&self, egui_style: &egui::Style, code: &str) -> bool {
        (&self.style, self.code.as_str()) != (egui_style, code)
    }
}

pub fn highlight(egui_style: &egui::Style, text: &str, ui_tokens: &Vec<UiToken>) -> egui::text::LayoutJob {
    let mut job = egui::text::LayoutJob::default();
    for token in ui_tokens.iter() {
        tracing::info!(" > {} {}", token.start, token.end);
        job.append(&text[token.start..token.end], 0.0, format_from_style(egui_style, token));
    }
    job
}

fn format_from_style(egui_style: &egui::Style, token: &UiToken) -> egui::text::TextFormat {
    use egui::{Align, Color32, Stroke};

    let color = egui_style.visuals.strong_text_color();
    let font_id = FontId::new(25.0, FontFamily::Proportional);
    let background = Color32::RED;
    let underline = Stroke::none();
    let strikethrough = Stroke::none();
    let valign = Align::TOP;

    egui::text::TextFormat {
        font_id,
        color,
        background,
        italics: false,
        underline,
        strikethrough,
        valign,
    }
}