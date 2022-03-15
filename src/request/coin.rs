use std::collections::BTreeMap;
use std::rc::Rc;

use serde_derive::Deserialize;
use serde_derive::Serialize;
use smartcalc::NumberType;
use smartcalc::SmartCalc;
use smartcalc::TokenType;
use wasm_bindgen::JsValue;
use smartcalc::RuleTrait;
use async_trait::async_trait;
use crate::request::get;

pub type CoinList = Vec<CoinItem>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoinItem {
    pub id: String,
    pub name: String,
    pub symbol: String,
    pub rank: i64,
    #[serde(rename = "is_new")]
    pub is_new: bool,
    #[serde(rename = "is_active")]
    pub is_active: bool,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default)]
pub struct Coin {
    coins: CoinList
}

#[async_trait]
pub trait ComponentTrait {
    async fn update(&mut self);
}

impl Coin {
    pub fn new(smartcalc: &mut SmartCalc) -> Rc<Self> {
        let data = Rc::new(Self::default());
        smartcalc.add_rule("en".to_string(), vec!["{TEXT:text}".to_string()], data.clone());
        data
    }

    pub async fn configure(&mut self, smartcalc: &mut SmartCalc) {
        let coins = get::<CoinList>("https://api.coinpaprika.com/v1/coins".to_string()).await;
    }
}

impl RuleTrait for Coin {
    fn name(&self) -> String {
        "Coin".to_string()
    }
    fn call(&self, fields: &BTreeMap<String, TokenType>) -> Option<TokenType> { 
        Some(TokenType::Number(123.0, NumberType::Decimal))
     }
}

pub async fn configure(smartcalc: &mut SmartCalc) -> Rc<dyn RuleTrait> {
    Coin::new(smartcalc)
}

pub async fn query() -> Result<CoinList, JsValue> {
    get("https://api.coinpaprika.com/v1/coins".to_string()).await
}
