/*
 * smartcalc-app v1.0.8
 * Copyright (c) Erhan BARIS (Ruslan Ognyanov Asenov)
 * Licensed under the GNU General Public License v2.0.
 */

use std::collections::HashMap;

use eframe::egui::{Ui, RichText};
use eframe::egui::{self, CollapsingHeader};
use lazy_static::*;

use crate::query::PluginManager;
use crate::toggle_switch::toggle;
use crate::app::State;
use crate::calculation::Calculation;

use chrono::TimeZone;
use chrono::Local;
use chrono_tz::{Tz, OffsetName};

use crate::config::{TIMEZONE_LIST, update_current_timezone};
use crate::config::Timezone;

lazy_static! {
    pub static ref DATE_PARSE_TYPES: Vec<DateFormat> = {
        let m = vec![DateFormat {
            name: "day month year".to_string(),
            datas: vec![
                "{NUMBER:day}.{NUMBER:month}.{NUMBER:year}".to_string(),
                "{NUMBER:day} {MONTH:month} {NUMBER:year}".to_string(),
                "{NUMBER:day}/{NUMBER:month}/{NUMBER:year}".to_string(),
                "{MONTH:month} {NUMBER:day}".to_string(),
                "{NUMBER:day} {MONTH:month}".to_string()
            ]
        }, DateFormat {
            name: "month day year".to_string(),
            datas: vec![
                "{NUMBER:month}.{NUMBER:day}.{NUMBER:year}".to_string(),
                "{MONTH:month} {NUMBER:day}, {NUMBER:year}".to_string(),
                "{MONTH:month} {NUMBER:day} {NUMBER:year}".to_string(),
                "{NUMBER:month}/{NUMBER:day}/{NUMBER:year}".to_string(),
                "{MONTH:month} {NUMBER:day}".to_string(),
                "{NUMBER:day} {MONTH:month}".to_string()
            ]
        }, DateFormat {
            name: "year month day".to_string(),
            datas: vec![
                "{NUMBER:year}.{NUMBER:month}.{NUMBER:day}".to_string(),
                "{NUMBER:year} {MONTH:month} {NUMBER:day}".to_string(),
                "{NUMBER:year}/{NUMBER:month}/{NUMBER:day}".to_string(),
                "{MONTH:month} {NUMBER:day}".to_string(),
                "{NUMBER:day} {MONTH:month}".to_string()
            ]
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

#[derive(Default, PartialEq)]
#[derive(Debug, Clone)]
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct NumberFormat {
    pub decimal_digits: u8,
    pub remove_fract_if_zero: bool,
    pub use_fract_rounding: bool
}

impl NumberFormat {
    pub fn new(decimal_digits: u8, remove_fract_if_zero: bool, use_fract_rounding: bool) -> Self {
        Self {
            decimal_digits,
            remove_fract_if_zero,
            use_fract_rounding
        }
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Settings {
    pub decimal_seperator: String,
    pub thousand_separator: String,
    pub timezone: Timezone,
    pub date_format: DateFormat,
    pub enabled_plugins: HashMap<String, bool>,
    pub money_format: NumberFormat,
    pub number_format: NumberFormat,
    pub percent_format: NumberFormat
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
            date_format: DATE_PARSE_TYPES[0].clone(),
            money_format: NumberFormat::new(0, false, true),
            number_format: NumberFormat::new(2, true, true),
            percent_format: NumberFormat::new(2, true, true),
        }
    }
}

#[derive(Default)]
pub struct SettingsWindow;

impl SettingsWindow {

    fn numbering_panel(&mut self, name: &str, ui: &mut Ui, config: &mut NumberFormat, update_smartcalc_config: &mut bool) {
        ui.separator();
        CollapsingHeader::new(format!("{} Configuration", name))
            .default_open(false)
            .show(ui, |ui| {
                ui.columns(2, |columns| {
                    columns[0].label("Decimal digits");
                    if columns[1].add(egui::DragValue::new(&mut config.decimal_digits).speed(1.0)).changed() {
                        tracing::warn!("{} decimal_digits changed", name);
                        *update_smartcalc_config = true;
                    }

                    columns[0].label("Remove fract if zero");
                    if columns[1].add(toggle(&mut config.remove_fract_if_zero)).changed() {
                        tracing::warn!("{} remove_fract_if_zero changed", name);
                        *update_smartcalc_config = true;
                    }
                    
                    columns[0].label("Round fract");
                    if columns[1].add(toggle(&mut config.use_fract_rounding)).changed() {
                        tracing::warn!("{} use_fract_rounding changed", name);
                        *update_smartcalc_config = true;
                    }
                });
            });
    }

    fn money_panel(&mut self, ui: &mut Ui, config: &mut NumberFormat, update_smartcalc_config: &mut bool) {
        ui.separator();
        CollapsingHeader::new("Money Configuration")
            .default_open(false)
            .show(ui, |ui| {
                ui.columns(2, |columns| {
                    columns[0].label("Decimal digits");
                    columns[1].label(RichText::new("value comes from currency type").italics());
                    
                    columns[0].label("Remove fract if zero");
                    if columns[1].add(toggle(&mut config.remove_fract_if_zero)).changed() {
                        tracing::warn!("Money remove_fract_if_zero changed");
                        *update_smartcalc_config = true;
                    }
                    
                    columns[0].label("Round fract");
                    if columns[1].add(toggle(&mut config.use_fract_rounding)).changed() {
                        tracing::warn!("Money use_fract_rounding changed");
                        *update_smartcalc_config = true;
                    }
                });
            });
    }

    pub fn ui(&mut self, ctx: &egui::Context, state: &mut State, settings: &mut Settings, plugins: &mut PluginManager, calculation: &mut Calculation) {
        let State { show_settings, update_smartcalc_config, ..} = state;

        egui::Window::new("⚙ Settings").collapsible(false).open(show_settings).resizable(false).show(ctx, |ui| {
            ui.columns(2, |columns| {
                columns[0].label("Decimal Seperator");
                if columns[1].text_edit_singleline(&mut settings.decimal_seperator).changed() {
                    tracing::warn!("decimal_seperator changed");
                    *update_smartcalc_config = true;
                }
            });
            
            ui.columns(2, |columns| {
                columns[0].label("Thousand Seperator");
                if columns[1].text_edit_singleline(&mut settings.thousand_separator).changed() {
                    tracing::warn!("thousand_separator changed");
                    *update_smartcalc_config = true;
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
                                update_current_timezone(&settings.timezone);
                                tracing::warn!("timezone changed");
                                *update_smartcalc_config = true;
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
                                *update_smartcalc_config = true;
                            }
                        }
                    });
                });

            self.numbering_panel("Number", ui, &mut settings.number_format, update_smartcalc_config);
            self.money_panel(ui, &mut settings.money_format, update_smartcalc_config);
            self.numbering_panel("Percentage", ui, &mut settings.percent_format, update_smartcalc_config);

            ui.separator();
            CollapsingHeader::new("Plugins")
                .default_open(false)
                .show(ui, |ui| {
                    ui.columns(2, |columns| {
                    for plugin in plugins.plugins.iter() {
                        columns[0].label(plugin.name());

                        match settings.enabled_plugins.get_mut(&plugin.name()) {
                            Some(status) => {
                                if columns[1].add(toggle(status)).changed() {
                                    tracing::warn!("plugin({}) changed, {}", plugin.name(), status);
                                    *update_smartcalc_config = true;

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
