use std::collections::BTreeMap;
use std::rc::Rc;

use eframe::egui::Context;
use smartcalc::SmartCalc;
use smartcalc::SmartCalcConfig;
use smartcalc::TokenType;
use smartcalc::RuleTrait;

use super::PluginTrait;

use super::PluginError;
use super::RequestManager;

#[derive(Default)]
pub struct WeatherPlugin {
    requests: Rc<RequestManager>,
    context: Context
}

impl PluginTrait for WeatherPlugin {
    fn http_result(&self, _: &str, _: Option<String>) { }
    fn get_rules(&self) -> Vec<String> { Vec::new() }
    fn upcast(self: Rc<Self>) -> Rc<dyn RuleTrait> { self }

    fn init(_: &mut SmartCalc, ctx: &Context, requests: Rc<RequestManager>) -> Result<Rc<Self>, PluginError> {
        let mut weather = Self::default();
        weather.requests = requests.clone();
        weather.context = ctx.clone();
        Ok(Rc::new(weather))
    }
}

impl RuleTrait for WeatherPlugin {
    fn name(&self) -> String {
        "Weather".to_string()
    }

    fn call(&self, _: &SmartCalcConfig, _: &BTreeMap<String, TokenType>) -> Option<TokenType> {
        None
     }
}
// {DATETIME_DATE_TIME:data} {GROUP:conversion:conversion_group} {TEXT:type:unix}