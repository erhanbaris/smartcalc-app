use std::collections::HashMap;

use eframe::egui::{self, CollapsingHeader};
use lazy_static::*;

use crate::query::PluginManager;
use crate::toggle_switch::toggle;
use crate::app::State;
use crate::calculation::Calculation;

use chrono::TimeZone;
use chrono::Local;
use chrono_tz::{Tz, OffsetName};

use crate::config::TIMEZONE_LIST;
use crate::config::Timezone;

lazy_static! {
    pub static ref DATE_PARSE_TYPES: Vec<DateFormat> = {
        let m = vec![DateFormat {
            name: "dd/mm/yyyy".to_string(),
            datas: vec!["{NUMBER:day}/{NUMBER:month}/{NUMBER:year}".to_string(),
                "{MONTH:month} {NUMBER:day}".to_string(),
                "{NUMBER:day} {MONTH:month}".to_string()]
        }, DateFormat {
            name: "mm/dd/yyyy".to_string(),
            datas: vec!["{NUMBER:month}/{NUMBER:day}/{NUMBER:year}".to_string(),
                "{MONTH:month} {NUMBER:day}".to_string(),
                "{NUMBER:day} {MONTH:month}".to_string()]
        }, DateFormat {
            name: "dd.mm.yyyy".to_string(),
            datas: vec!["{NUMBER:day}.{NUMBER:month}.{NUMBER:year}".to_string(),
                "{MONTH:month} {NUMBER:day}".to_string(),
                "{NUMBER:day} {MONTH:month}".to_string()]
        }, DateFormat {
            name: "mm.dd.yyyy".to_string(),
            datas: vec!["{NUMBER:month}.{NUMBER:day}.{NUMBER:year}".to_string(),
                "{MONTH:month} {NUMBER:day}".to_string(),
                "{NUMBER:day} {MONTH:month}".to_string()]
        }];
        m
    };
}

#[derive(Default, PartialEq)]
#[derive(Debug, Clone)]
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct DateFormat {
    pub datas: Vec<String>,
    pub name: String
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Settings {
    pub decimal_seperator: String,
    pub thousand_separator: String,
    pub timezone: Timezone,
    pub date_format: DateFormat,
    pub enabled_plugins: HashMap<String, bool>
}

impl Default for Settings {
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

        let timezone = match TIMEZONE_LIST.iter().find(|tz| tz.name == timezone || tz.name == timezone) {
            Some(tz) => tz.clone(),
            None => TIMEZONE_LIST[TIMEZONE_LIST.len() - 1].clone()
        };

        Self {
            timezone,
            decimal_seperator: ",".to_string(),
            thousand_separator: ".".to_string(),
            enabled_plugins: HashMap::new(),
            date_format: DATE_PARSE_TYPES[0].clone()
        }
    }
}

#[derive(Default)]
pub struct SettingsWindow;

impl SettingsWindow {
    pub fn ui(&mut self, ctx: &egui::Context, state: &mut State, settings: &mut Settings, plugins: &mut PluginManager, calculation: &mut Calculation) {
        egui::Window::new("⚙ Settings").collapsible(false).open(&mut state.show_settings).resizable(false).show(ctx, |ui| {
            ui.columns(2, |columns| {
                columns[0].label("Decimal Seperator");
                if columns[1].text_edit_singleline(&mut settings.decimal_seperator).changed() {
                    tracing::warn!("decimal_seperator changed");
                    state.update_smartcalc_config = true;
                }
            });
            
            ui.columns(2, |columns| {
                columns[0].label("Thousand Seperator");
                if columns[1].text_edit_singleline(&mut settings.thousand_separator).changed() {
                    tracing::warn!("thousand_separator changed");
                    state.update_smartcalc_config = true;
                }
            });

            ui.columns(2, |columns| {
                columns[0].label("Timezone");
                egui::ComboBox::from_id_source("timezone")
                    .width(180.)
                    .selected_text(format!("{}", settings.timezone.name))
                    .show_ui(&mut columns[1], |ui| {
                        for timezone in crate::config::TIMEZONE_LIST.iter() {
                            if ui.selectable_value(&mut settings.timezone, timezone.clone(), timezone.to_string()).changed() {
                                tracing::warn!("timezone changed");
                                state.update_smartcalc_config = true;
                            }
                        }
                    });
            });

            ui.columns(2, |columns| {
                columns[0].label("Date Format(parse)");
                    egui::ComboBox::from_id_source("date-format-parse")
                    .width(180.)
                    .selected_text(format!("{}", settings.date_format.name))
                    .show_ui(&mut columns[1], |ui| {
                        for date_format in DATE_PARSE_TYPES.iter() {
                            if ui.selectable_value(&mut settings.date_format, date_format.clone(), date_format.name.to_string()).changed() {
                                tracing::warn!("date-format-parse changed");
                                state.update_smartcalc_config = true;
                            }
                        }
                    });
            });

            CollapsingHeader::new("Plugins")
                .default_open(true)
                .show(ui, |ui| {
                    ui.columns(2, |columns| {
                    for plugin in plugins.plugins.iter() {
                        columns[0].label(plugin.name());

                        match settings.enabled_plugins.get_mut(&plugin.name()) {
                            Some(status) => {
                                if columns[1].add(toggle(status)).changed() {
                                    tracing::warn!("plugin({}) changed, {}", plugin.name(), status);
                                    state.update_smartcalc_config = true;

                                    match status {
                                        true => plugin.enable(&mut calculation.smartcalc),
                                        false => plugin.disable(&mut calculation.smartcalc)
                                    }
                                }
                            },
                            None => { columns[1].label("-"); }
                        };
                    }
                });
            });
        });
    }
}
