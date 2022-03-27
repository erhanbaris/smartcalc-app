use std::any::Any;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

use eframe::egui::Context;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::from_str;
use smartcalc::SmartCalc;
use smartcalc::SmartCalcConfig;
use smartcalc::TokenType;
use smartcalc::RuleTrait;


use crate::http::Request;
use super::PluginStatus;
use super::PluginTrait;

use super::PluginError;
use super::get_number;
use super::get_text;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoinData {
    pub data: Vec<CoinItem>,
    pub timestamp: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoinItem {
    pub id: String,
    pub rank: String,
    pub symbol: String,
    pub name: String,
    pub supply: String,
    pub max_supply: Option<String>,
    pub market_cap_usd: String,
    #[serde(rename = "volumeUsd24Hr")]
    pub volume_usd24hr: String,
    pub price_usd: String,
    #[serde(rename = "changePercent24Hr")]
    pub change_percent24hr: String,
    #[serde(rename = "vwap24Hr")]
    pub vwap24hr: Option<String>,
    pub explorer: Option<String>,
}

#[derive(Default)]
pub struct CoinPlugin {
    coins: Vec<CoinItem>,
    request: Option<Request>,
    status: PluginStatus
}

impl CoinPlugin {
    pub fn loading(&mut self) {
        self.status = PluginStatus::Loading;
    }

    pub fn ready_to_process(&mut self) {
        self.status = PluginStatus::ReadyToProcess;
    }
}

impl PluginTrait for CoinPlugin {
    fn http_result(&self, _: &str) -> Rc<dyn Any> { Rc::new("Coin".to_string()) }
    fn get_rules(&self) -> Vec<String> { Vec::new() }
    fn upcast(self: Rc<Self>) -> Rc<dyn RuleTrait> { self }

    fn init(_: &mut SmartCalc, ctx: &Context) -> Result<Rc<Self>, PluginError> {
        let mut coin = Self::default();
        coin.update(ctx)?;
        Ok(Rc::new(coin))
    }

    fn update(&mut self, ctx: &Context) -> Result<(), PluginError> {
        self.loading();
        self.request = Some(Request::get("https://api.coincap.io/v2/assets", ctx));
        Ok(())
    }

    fn process(&mut self) {
        match &self.request {
            Some(promise) => {
                println!("get_data");
                match promise.get_data() {
                    Some(response) => {
                        self.coins = match from_str::<CoinData>(&response[..]) {
                            Ok(result) => result.data,
                            Err(error) => {
                                tracing::warn!("Json parse: {}", error);
                                Vec::new()
                            }
                        };

                        self.ready_to_process();
                    },
                    None => ()
                }
            },
            None => ()
        };
    }

    fn status(&self) -> PluginStatus {
        self.status
    }
}

impl RuleTrait for CoinPlugin {
    fn name(&self) -> String {
        "Coin".to_string()
    }

    fn call(&self, smartcalc: &SmartCalcConfig, fields: &BTreeMap<String, TokenType>) -> Option<TokenType> {
        if fields.contains_key("coin") {
            let coin_name = get_text("coin", fields).unwrap().to_lowercase();
            let coin = match self.coins.iter().find(|item| item.symbol.to_lowercase() == coin_name || item.name.to_lowercase() == coin_name) {
                Some(coin) => coin.price_usd.parse::<f64>().unwrap(),
                None => return None
            };

            let price = match get_number("count", fields) {
                Some(count) => count,
                None => 1.0
            } * coin;

            return Some(TokenType::Money(price, smartcalc.get_currency("usd".to_string()).unwrap()));
        } else {
            return None;
        }
    }
}
