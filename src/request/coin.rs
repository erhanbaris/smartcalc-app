/*
 * smartcalc-app v1.0.7
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

#[derive(Debug)]
pub enum ComponentError {
    Error(String)
}

pub trait ComponentTrait: RuleTrait {
    fn update(&self) -> Result<(), ComponentError>;
}

impl ComponentTrait for Coin {
    fn update(&self) -> Result<(), ComponentError> {
        Ok(())
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
            let coin_name = get_text("coin", fields).unwrap().to_lowercase();
            let coin = match self.coins.borrow().iter().find(|item| item.symbol.to_lowercase() == coin_name || item.name.to_lowercase() == coin_name) {
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

impl Coin {
    async fn configure(&self, _: &mut SmartCalc) -> Result<(), ComponentError> {
        *self.coins.borrow_mut() = match get::<CoinData>("https://api.coincap.io/v2/assets".to_string()).await {
            Ok(data) => data.data,
            Err(error) => {
                return Err(ComponentError::Error(format!("{:?}", error)))
            }
        };

        Ok(())
    }
}

pub async fn configure(smartcalc: &mut SmartCalc) -> Result<Rc<dyn ComponentTrait>, ComponentError> {
    let coin = Rc::new(Coin::default());
    coin.configure(smartcalc).await?;
    smartcalc.add_rule("en".to_string(), vec!["{NUMBER:count} {TEXT:coin}".to_string(), "{TEXT:coin}".to_string()], coin.clone());
    Ok(coin)
}
