use eframe::egui::{self, CollapsingHeader};

use crate::query::PluginManager;
use crate::toggle_switch::toggle;
use crate::app::State;
use crate::calculation::{Settings, Calculation};

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
                columns[0].label("Date Format");
                    egui::ComboBox::from_id_source("date-format")
                    .width(180.)
                    .selected_text(format!("{}", settings.date_format))
                    .show_ui(&mut columns[1], |ui| {
                        ui.selectable_value(&mut settings.date_format, "dd/MM/yyyy".to_string(), "dd/MM/yyyy");
                        ui.selectable_value(&mut settings.date_format, "MM/dd/yyyy".to_string(), "MM/dd/yyyy");
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
