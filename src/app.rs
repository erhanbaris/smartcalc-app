use std::{collections::BTreeMap};

use eframe::{egui::{self, FontDefinitions, FontData, Button, Widget, RichText, Visuals, text_edit::CursorRange, CollapsingHeader}, epi, epaint::{Color32, FontFamily, Vec2}};
use serde_derive::{Deserialize, Serialize};
use serde_json::from_str;
use smartcalc::SmartCalc;
use chrono::TimeZone;
use chrono::Local;
use chrono_tz::{Tz, OffsetName};

use crate::{result::ResultPanel, http::Request, calculation::Calculation, toggle_switch::toggle, query::PluginManager};
use crate::code::CodePanel;


#[derive(Default, Debug, Clone, PartialEq)]
#[derive(Deserialize, Serialize)]
pub struct Currency {
    pub rate: f64
}

#[derive(Default)]
pub struct State {
    pub scroll: Vec2,
    pub cursor: Option<CursorRange>,
    pub show_settings: bool
}

pub struct SmartcalcApp {
    calculation: Calculation,
    code_panel: CodePanel,
    result_panel: ResultPanel,
    fetch_currencies: Option<Request>,
    state: State,
    plugins: PluginManager
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
            state: State::default(),
            plugins: PluginManager::default()
        }
    }
}

impl epi::App for SmartcalcApp {
    fn name(&self) -> &str {
        "SmartCalc App"
    }

    fn setup(&mut self, ctx: &egui::Context, _frame: &epi::Frame, _storage: Option<&dyn epi::Storage>) {
        let Self { result_panel, code_panel, calculation, fetch_currencies, state, plugins} = self;

        #[cfg(feature = "persistence")]
        if let Some(storage) = _storage {
            *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
        }

        let mut font = FontDefinitions::default();
        font.font_data.insert("Quicksand".to_owned(),FontData::from_static(include_bytes!("../fonts/Quicksand-Regular.ttf")));
        font.families.insert(FontFamily::Name("Quicksand".into()), vec!["Quicksand".to_owned(), "Ubuntu-Light".to_owned(), "NotoEmoji-Regular".to_owned(), "emoji-icon-font".to_owned()]);
        ctx.set_fonts(font);
        ctx.set_visuals(Visuals::dark());

        *fetch_currencies = Some(Request::get("https://www.floatrates.com/daily/usd.json", ctx));
        plugins.build(&mut calculation.smartcalc, ctx);
    }

    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }
    
    fn update(&mut self, ctx: &egui::Context, _frame: &epi::Frame) {
        let Self { result_panel, code_panel, calculation, fetch_currencies, state, plugins} = self;
        plugins.process(ctx, &mut calculation.smartcalc);

        egui::Window::new("⚙ Settings").collapsible(false).open(&mut state.show_settings).resizable(false).show(ctx, |ui| {
            let mut my_bool = true;

            CollapsingHeader::new("Widgets")
                .default_open(true)
                .show(ui, |ui| {
                    ui.label("Hello World!");
                    ui.add(toggle(&mut my_bool));
            });
        });
        
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                let settings = Button::new(RichText::new("⚙ Settings")).ui(ui);

                if settings.clicked() {
                    state.show_settings = true;
                }
                    
                let update_currencies = Button::new(RichText::new("🔄 Update Currencies").color(Color32::WHITE))
                    .fill(Color32::from_rgb(33, 133, 208))
                    .ui(ui);

                if update_currencies.clicked() && fetch_currencies.is_none() {
                    *fetch_currencies = Some(Request::get("https://www.floatrates.com/daily/usd.json", ctx));
                }

                let fetch_done = match fetch_currencies {
                    Some(promise) => {
                        match promise.get_data() {
                            Some(response) => {
                                match from_str::<BTreeMap<String, Currency>>(response) {
                                    Ok(data) => {
                                        for (name, currency) in data.iter() {
                                            calculation.smartcalc.update_currency(name, currency.rate);
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
                if plugins.ongoing_request() {
                    ui.add(egui::Spinner::new());
                    ui.label("Loading... ");
                    tracing::warn!(">>>loaded");
                }
                egui::warn_if_debug_build(ui);
                ui.spacing_mut().item_spacing.x = 0.0;
                ui.label("powered by ");
                ui.hyperlink_to("SmartCalc Engine", "https://github.com/erhanbaris/smartcalc");
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {

            ui.columns(2, |columns| {
                /* Left panel */
                result_panel.ui(&mut columns[0], calculation, state);

                /* Right panel */
                code_panel.ui(&mut columns[1], calculation, state);
            });
        });
    }
}
