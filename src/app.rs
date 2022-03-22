use std::{collections::BTreeMap};

use eframe::{egui::{self, FontDefinitions, FontData, Button, Widget, RichText, Visuals}, epi, epaint::{Color32, FontFamily}};
use serde_derive::{Deserialize, Serialize};
use serde_json::from_str;
use smartcalc::SmartCalc;
use chrono::TimeZone;
use chrono::Local;
use chrono_tz::{Tz, OffsetName};

use crate::{result::ResultPanel, http::Request, calculation::Calculation};
use crate::code::CodePanel;


#[derive(Default, Debug, Clone, PartialEq)]
#[derive(Deserialize, Serialize)]
pub struct Currency {
    pub rate: f64
}

pub struct SmartcalcApp {
    calculation: Calculation,
    code_panel: CodePanel,
    result_panel: ResultPanel,
    fetch_currencies: Option<Request>,
    cursor_row: usize
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
    
        let mut smartcalc = SmartCalc::default();
        
        smartcalc.set_decimal_seperator(",".to_string());
        smartcalc.set_thousand_separator(".".to_string());
        smartcalc.set_timezone(timezone).unwrap();

        Self {
            result_panel: ResultPanel::default(),
            code_panel: CodePanel::default(),
            fetch_currencies: None,
            calculation: Calculation::new(),
            cursor_row: 0
        }
    }
}

impl epi::App for SmartcalcApp {
    fn name(&self) -> &str {
        "SmartCalc App"
    }

    fn setup(&mut self, ctx: &egui::Context, _frame: &epi::Frame, _storage: Option<&dyn epi::Storage>) {
        #[cfg(feature = "persistence")]
        if let Some(storage) = _storage {
            *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
        }

        let mut font = FontDefinitions::default();
        font.font_data.insert("TitilliumWeb".to_owned(),FontData::from_static(include_bytes!("../fonts/TitilliumWeb-Regular.ttf")));
        font.font_data.insert("TitilliumWebBold".to_owned(),FontData::from_static(include_bytes!("../fonts/TitilliumWeb-Bold.ttf")));
        font.families.insert(FontFamily::Name("TitilliumWeb".into()), vec!["TitilliumWeb".to_owned()]);
        font.families.insert(FontFamily::Name("TitilliumWebBold".into()), vec!["TitilliumWebBold".to_owned()]);
        ctx.set_fonts(font);
        ctx.set_visuals(Visuals::dark());
    }

    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }
    
    fn update(&mut self, ctx: &egui::Context, _frame: &epi::Frame) {
        let Self { result_panel, code_panel, calculation, fetch_currencies, cursor_row } = self;

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                let update_currencies = Button::new(RichText::new("🔄 Update Currencies").color(Color32::WHITE))
                    .fill(Color32::from_rgb(33, 133, 208))
                    .ui(ui);

                if update_currencies.clicked() && fetch_currencies.is_none() {
                    *fetch_currencies = Some(Request::get("http://www.floatrates.com/daily/usd.json", &ctx));
                }

                let fetch_done = match fetch_currencies {
                    Some(promise) => {
                        match promise.get_data() {
                            Some(response) => {
                                match from_str::<BTreeMap<String, Currency>>(response) {
                                    Ok(data) => {
                                        for (name, currency) in data.iter() {
                                            calculation.smartcalc.update_currency(&name, currency.rate);
                                        }
                                        calculation.calculate();
                                    },
                                    Err(error) => println!("JSON parse error: {:?}", error)
                                }
                                true
                            },
                            None => {
                                ui.add(egui::Spinner::new());
                                false
                            }
                        }
                    },
                    None => false
                };

                if fetch_done {
                    *fetch_currencies = None;
                }
            });
        });

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                egui::warn_if_debug_build(ui);
                ui.spacing_mut().item_spacing.x = 0.0;
                ui.label("powered by ");
                ui.hyperlink_to("SmartCalc Engine", "https://github.com/erhanbaris/smartcalc");
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {

            ui.columns(2, |columns| {
                /* Left panel */
                code_panel.ui(&mut columns[0], calculation, cursor_row);

                /* Right panel */
                result_panel.ui(&mut columns[1], calculation, cursor_row);
            });
        });
    }
}
