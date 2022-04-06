use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

use chrono::Utc;
use eframe::egui::Context;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::from_str;
use smartcalc::TimeOffset;
use smartcalc::SmartCalc;
use smartcalc::SmartCalcConfig;
use smartcalc::TokenType;
use smartcalc::RuleTrait;

use crate::config::TIMEZONE_LIST;
use crate::config::get_current_timezone;
use crate::http::Request;
use super::PluginTrait;

use super::PluginError;
use super::RequestManager;
use super::get_text;

pub type CityArray = Vec<CityItem>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CityItem {
    pub names: Vec<String>,
    pub lat: String,
    pub lon: String,
    pub timezone: String,
}

#[derive(Default)]
pub struct CityTimePlugin {
    cities: RefCell<CityArray>,
    requests: Rc<RequestManager>
}

impl PluginTrait for CityTimePlugin {
    fn http_result(&self, result: &str, _: Option<String>) {
        *self.cities.borrow_mut() = match from_str(&result) {
            Ok(result) => result,
            Err(error) => {
                tracing::warn!("Json parse: {}", error);
                Vec::new()
            }
        };
    }
    fn get_rules(&self) -> Vec<String> {
        vec![
            "time {TEXT:type:at} {TEXT:city}".to_string(),
            "{TEXT:city} {TEXT:type:at} time".to_string(),
            "time {GROUP:conversion:conversion_group} {TEXT:city}".to_string(),
            "{TEXT:city} {GROUP:conversion:conversion_group} time".to_string()        ]
    }
    fn upcast(self: Rc<Self>) -> Rc<dyn RuleTrait> { self }

    fn init(_: &mut SmartCalc, ctx: &Context, requests: Rc<RequestManager>) -> Result<Rc<Self>, PluginError> {
        let mut city_time = Self::default();
        city_time.requests = requests;
        city_time.requests.add(&city_time.name(), Request::get("https://erhanbaris.github.io/jsons/cities.json", ctx));
        Ok(Rc::new(city_time))
    }
}

impl RuleTrait for CityTimePlugin {
    fn name(&self) -> String {
        "City Time".to_string()
    }

    fn call(&self, _: &SmartCalcConfig, fields: &BTreeMap<String, TokenType>) -> Option<TokenType> {
        if fields.contains_key("city") {
            let city_name = get_text("city", fields).unwrap().to_lowercase();
            return match self.cities.borrow().iter().find(|item| item.names.contains(&city_name)) {
                Some(city) => {
                    match TIMEZONE_LIST.iter().find(|timezone| timezone.name == city.timezone) {
                        Some(timezone) => Some(TokenType::Time(Utc::now().naive_utc(), TimeOffset {
                            name: get_current_timezone().name,
                            offset: (timezone.offset * 60.0) as i32
                        })),
                        None => None
                    }
                },
                None => None
            };
        } else {
            return None;
        }
    }
}
