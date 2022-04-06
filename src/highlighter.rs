use eframe::egui;
use eframe::epaint::{FontId, FontFamily};
use smartcalc::{UiToken, UiTokenType};

#[derive(Default)]
pub struct MemoizedHighlighter {
    code: String,
    output: egui::text::LayoutJob
}

impl MemoizedHighlighter {

    fn add_layout_item(&self, text: &str, start_position: usize, end_position: usize, job: &mut egui::text::LayoutJob, token: Option<&UiToken>) {        
        job.append(&text[start_position..end_position], 0.0, self.format_from_style(token));
    }

    fn consume_whitespaces(&self, text: &str, last_position: usize, job: &mut egui::text::LayoutJob) -> usize {
        let start_position = last_position;
        let mut last_position = last_position;

        while matches!(text.chars().nth(last_position), Some(ch) if ch.is_whitespace()) {
            last_position += 1
        }

        if start_position != last_position {
            self.add_layout_item(text, start_position, last_position, job, None);
        }

        last_position
    }

    fn colorize_tokens(&self, text: &str, ui_tokens: &[Vec<UiToken>]) -> egui::text::LayoutJob {
        let mut job = egui::text::LayoutJob::default();
        let mut last_position = self.consume_whitespaces(text, 0, &mut job);

        for token_line in ui_tokens.iter() {
            let last_line_position = last_position;
            
            for token in token_line.iter() {
                let start_position = last_line_position + token.start;
                let end_position = last_line_position + token.end;

                self.add_layout_item(text, start_position, end_position, &mut job, Some(token));

                last_position = self.consume_whitespaces(text, end_position, &mut job);
            }
        }

        self.consume_whitespaces(text, last_position, &mut job);
        job
    }

    fn format_from_style(&self, token: Option<&UiToken>) -> egui::text::TextFormat {
        use egui::{Align, Color32, Stroke};

        let (color, background, underline) = match token {
            Some(token) => {
                match token.ui_type {
                    UiTokenType::Text => (Color32::from_rgb(139, 148, 158), Color32::TRANSPARENT, Stroke::none()),
                    UiTokenType::Number => (Color32::from_rgb(121, 192, 255), Color32::TRANSPARENT, Stroke::none()),
                    UiTokenType::Symbol1 => (Color32::from_rgb(255, 123, 114), Color32::TRANSPARENT, Stroke::none()),
                    UiTokenType::Symbol2 => (Color32::from_rgb(255, 123, 114), Color32::TRANSPARENT, Stroke::none()),
                    UiTokenType::DateTime => (Color32::from_rgb(139, 148, 158), Color32::TRANSPARENT, Stroke::none()),
                    UiTokenType::Operator => (Color32::from_rgb(208, 168, 105), Color32::TRANSPARENT, Stroke::none()),
                    UiTokenType::Comment => (Color32::from_rgb(111, 66, 193), Color32::TRANSPARENT, Stroke::none()),
                    UiTokenType::VariableDefination => (Color32::WHITE, Color32::TRANSPARENT, Stroke::new(1.0, Color32::WHITE)),
                    UiTokenType::VariableUse => (Color32::WHITE, Color32::TRANSPARENT, Stroke::none()),
                    UiTokenType::Month => (Color32::from_rgb(139, 148, 158), Color32::TRANSPARENT, Stroke::none()),
                }
            },
            None => (Color32::from_rgb(139, 148, 158), Color32::TRANSPARENT, Stroke::none())
        };

        let font_id = FontId::new(30.0, FontFamily::Name("Quicksand".into()));
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

    pub fn highlight(&mut self, code: &str, ui_tokens: &[Vec<UiToken>], recalculate: bool) -> egui::text::LayoutJob {
        if self.is_dirty(code) || recalculate {
            self.code = code.to_owned();
            self.output = self.colorize_tokens(code, ui_tokens);
        }
        self.output.clone()
    }

    pub fn is_dirty(&self, code: &str) -> bool {
        self.code.as_str() != code
    }
}
