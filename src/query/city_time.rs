use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

use chrono::NaiveDateTime;
use eframe::egui::Context;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::from_str;
use smartcalc::NumberType;
use smartcalc::SmartCalc;
use smartcalc::SmartCalcConfig;
use smartcalc::TokenType;
use smartcalc::RuleTrait;


use crate::http::Request;
use super::PluginTrait;

use super::PluginError;
use super::RequestManager;
use super::get_number;
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
        vec!["time {GROUP:conversion:conversion_group} {TEXT:city}".to_string(), "{TEXT:city} {GROUP:conversion:conversion_group} time".to_string()]
    }
    fn upcast(self: Rc<Self>) -> Rc<dyn RuleTrait> { self }

    fn init(_: &mut SmartCalc, ctx: &Context, requests: Rc<RequestManager>) -> Result<Rc<Self>, PluginError> {
        let mut city_time = Self::default();
        city_time.requests = requests;
        city_time.requests.add(&city_time.name(), Request::get("https://github.com/erhanbaris/jsons/raw/main/cities.json", ctx));
        Ok(Rc::new(city_time))
    }
}

impl RuleTrait for CityTimePlugin {
    fn name(&self) -> String {
        "CityTime".to_string()
    }

    fn call(&self, _: &SmartCalcConfig, fields: &BTreeMap<String, TokenType>) -> Option<TokenType> {
        if fields.contains_key("city") {
            let city_name = get_text("city", fields).unwrap().to_lowercase();
            return match self.cities.borrow().iter().find(|item| item.names.contains(&city_name)) {
                Some(city) => {
                    tracing::warn!("City found : {:?}", city);
                    Some(TokenType::Number(1024.0, NumberType::Decimal))
                },
                None => None
            };
        } else {
            return None;
        }
    }
}