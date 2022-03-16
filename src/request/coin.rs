/*
 * smartcalc-app v1.0.6
 * Copyright (c) Erhan BARIS (Ruslan Ognyanov Asenov)
 * Licensed under the GNU General Public License v2.0.
 */

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::ops::Deref;
use std::rc::Rc;

use serde_derive::Deserialize;
use serde_derive::Serialize;
use smartcalc::SmartCalcAstType;
use smartcalc::SmartCalc;
use smartcalc::SmartCalcConfig;
use smartcalc::TokenType;
use smartcalc::RuleTrait;
use async_trait::async_trait;
use wasm_bindgen_futures::spawn_local;
use crate::request::get;

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
pub struct Coin {
    coins: Rc<RefCell<Vec<CoinItem>>>
}

#[async_trait]
pub trait ComponentTrait: RuleTrait {
    fn update(&self);
    fn configure(&self, smartcalc: &mut SmartCalc);
}

impl Coin {
    pub fn new(smartcalc: &mut SmartCalc) -> Rc<Self> {
        let data = Rc::new(Self::default());
        smartcalc.add_rule("en".to_string(), vec!["coin {TEXT:coin}".to_string(), "coin {TEXT:coin} {GROUP:conversion:conversion_group} {TEXT:target}".to_string()], data.clone());
        data
    }
}

impl ComponentTrait for Coin {
    fn update(&self) {

    }

    fn configure(&self, _: &mut SmartCalc) {
        let coins = self.coins.clone();
        spawn_local(async move {
            *coins.borrow_mut() = match get::<CoinData>("https://api.coincap.io/v2/assets".to_string()).await {
                Ok(coins) => coins.data,
                Err(error) => {
                    web_sys::console::log_1(&error);
                    Vec::new()
                }
            };
        });
    }
}

pub fn get_number(field_name: &str, fields: &BTreeMap<String, TokenType>) -> Option<f64> {
    return match fields.get(field_name) {
        Some(data) => match data {
            TokenType::Number(number, _) => Some(*number),
            TokenType::Variable(variable) => {
                match variable.data.borrow().deref().deref() {
                    SmartCalcAstType::Item(item) => Some(item.get_underlying_number()),
                    _ => None
                }
            },
            _ => None
        },
        _ => None
    };
}

pub fn get_text(field_name: &str, fields: &BTreeMap<String, TokenType>) -> Option<String> {
    return match fields.get(field_name) {
        Some(TokenType::Text(text)) =>  Some(text.to_string()),
        _ => None
    }
}

impl RuleTrait for Coin {
    fn name(&self) -> String {
        "Coin".to_string()
    }
    fn call(&self, smartcalc: &SmartCalcConfig, fields: &BTreeMap<String, TokenType>) -> Option<TokenType> {
        if fields.contains_key("coin") {
            let coin = get_text("coin", fields).unwrap();
            return match self.coins.borrow().iter().find(|item| item.symbol == coin || item.name == coin) {
                Some(coin) => Some(TokenType::Money(coin.price_usd.parse::<f64>().unwrap(), smartcalc.get_currency("usd".to_string()).unwrap())),
                None => None
            };
        } else {
            return None;
        }
     }
}

pub fn configure(smartcalc: &mut SmartCalc) -> Rc<dyn ComponentTrait> {
    Coin::new(smartcalc)
}

/*pub async fn query() -> Result<Coinda, JsValue> {
    get("https://api.coinpaprika.com/v1/coins".to_string()).await
}
*/