use eframe::egui;
use eframe::epaint::{FontId, FontFamily};

#[derive(Default)]
pub struct MemoizedHighlighter {
    style: egui::Style,
    code: String,
    output: egui::text::LayoutJob,
    cursor_row: usize
}

impl MemoizedHighlighter {
    pub fn highlight(&mut self, egui_style: &egui::Style, code: &str, cursor_row: usize) -> egui::text::LayoutJob {
        if (&self.style, self.code.as_str()) != (egui_style, code) {
            self.style = egui_style.clone();
            self.code = code.to_owned();
            self.output = highlight(egui_style, code, cursor_row);
            self.cursor_row = cursor_row;
        }
        self.output.clone()
    }
}

pub fn highlight(egui_style: &egui::Style, text: &str, cursor_row: usize) -> egui::text::LayoutJob {
    let mut job = egui::text::LayoutJob::default();
    for (index, row) in text.split_inclusive('\n').enumerate() {
        job.append(row, 0.0, format_from_style(egui_style, index, cursor_row));
    }
    job
}

fn format_from_style(egui_style: &egui::Style, row: usize, cursor_row: usize) -> egui::text::TextFormat {
    use egui::{Align, Color32, Stroke};

    let color = egui_style.visuals.strong_text_color();
    let font_id = FontId::new(25.0, FontFamily::Proportional);
    let background = Color32::RED;
    let underline = match row == cursor_row {
        true => Stroke::new(1.0, color),
        false => Stroke::none()
    };
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