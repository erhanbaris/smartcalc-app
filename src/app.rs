use std::{collections::BTreeMap};

use eframe::{egui::{self, FontDefinitions, FontData, Button, Widget, RichText, Visuals, text_edit::CursorRange}, epi, epaint::{Color32, FontFamily, Vec2}};
use serde_derive::{Deserialize, Serialize};
use serde_json::from_str;
use smartcalc::SmartCalc;

use crate::{result::ResultPanel, http::Request, calculation::Calculation, query::PluginManager, settings::{SettingsWindow, Settings}};
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
    pub show_settings: bool,
    pub update_smartcalc_config: bool
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct SmartcalcApp {
    calculation: Calculation,

    #[serde(skip)]
    code_panel: CodePanel,

    #[serde(skip)]
    result_panel: ResultPanel,

    #[serde(skip)]
    settings_window: SettingsWindow,

    #[serde(skip)]
    fetch_currencies: Option<Request>,

    #[serde(skip)]
    state: State,

    #[serde(skip)]
    plugins: PluginManager,

    settings: Settings
}

impl Default for SmartcalcApp {
    fn default() -> Self {
        let mut smartcalc = SmartCalc::default();
        
        smartcalc.set_decimal_seperator(",".to_string());
        smartcalc.set_thousand_separator(".".to_string());
        smartcalc.set_timezone("UTC".to_string()).unwrap();

        Self {
            result_panel: ResultPanel::default(),
            code_panel: CodePanel::default(),
            fetch_currencies: None,
            calculation: Calculation::new(),
            state: State::default(),
            plugins: PluginManager::default(),
            settings: Settings::default(),
            settings_window: SettingsWindow::default()
        }
    }
}

impl epi::App for SmartcalcApp {
    fn name(&self) -> &str {
        "SmartCalc App"
    }

    fn setup(&mut self, ctx: &egui::Context, _frame: &epi::Frame, storage: Option<&dyn epi::Storage>) {
        if let Some(storage) = storage {
            *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
        }

        let Self { calculation, fetch_currencies, plugins, settings, ..} = self;

        let mut font = FontDefinitions::default();
        font.font_data.insert("Quicksand".to_owned(),FontData::from_static(include_bytes!("../fonts/Quicksand-Regular.ttf")));
        font.families.insert(FontFamily::Name("Quicksand".into()), vec!["Quicksand".to_owned(), "Ubuntu-Light".to_owned(), "NotoEmoji-Regular".to_owned(), "emoji-icon-font".to_owned()]);
        ctx.set_fonts(font);
        ctx.set_visuals(Visuals::dark());
        calculation.set(settings);

        *fetch_currencies = Some(Request::get("https://www.floatrates.com/daily/usd.json", ctx));
        plugins.build(&mut calculation.smartcalc, &mut settings.enabled_plugins, ctx);
    }

    fn save(&mut self, storage: &mut dyn epi::Storage) {
        tracing::warn!("saved");
        epi::set_value(storage, epi::APP_KEY, self);
    }
    
    fn update(&mut self, ctx: &egui::Context, _: &epi::Frame) {
        let Self { result_panel, code_panel, calculation, fetch_currencies, state, plugins, settings, settings_window} = self;
        plugins.process(ctx, &mut calculation.smartcalc);
        settings_window.ui(ctx, state, settings, plugins, calculation);

        if state.update_smartcalc_config {
            calculation.set(settings);
        }
        
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
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("Version: ");
                    ui.label(RichText::new(env!("CARGO_PKG_VERSION")).color(Color32::WHITE));
                    ui.add_space(6.0);
                    ui.separator();
                    ui.add_space(6.0);
                    ui.label("Status: ");
                    if plugins.ongoing_request() {
                        ui.label(RichText::new("Loading").strong().color(Color32::LIGHT_GREEN));
                    } else {
                        ui.label(RichText::new("Ready").color(Color32::WHITE));
                    }
                    ui.add_space(6.0);
                    ui.separator();
                });

                ui.with_layout(egui::Layout::right_to_left(), |ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.hyperlink_to("SmartCalc Engine", "https://github.com/erhanbaris/smartcalc");
                    ui.add_space(6.0);
                    ui.label("powered by ");
                });
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
