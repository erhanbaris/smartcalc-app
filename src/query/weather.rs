use std::any::Any;
use std::borrow::BorrowMut;
use std::collections::BTreeMap;
use std::rc::Rc;

use eframe::egui::Context;
use smartcalc::SmartCalc;
use smartcalc::SmartCalcConfig;
use smartcalc::TokenType;
use smartcalc::RuleTrait;

use crate::http::Request;
use super::PluginStatus;
use super::PluginTrait;

use super::PluginError;
use super::RequestManager;
use super::get_text;

#[derive(Default)]
pub struct WeatherPlugin {
    request: Option<Request>,
    status: PluginStatus,
    requests: Rc<RequestManager>,
    context: Context
}

impl WeatherPlugin {
    pub fn loading(&mut self) {
        self.status = PluginStatus::Loading;
    }

    pub fn done(&mut self) {
        self.status = PluginStatus::Done;
    }

    pub fn ready_to_process(&mut self) {
        self.status = PluginStatus::ReadyToProcess;
    }
}

impl PluginTrait for WeatherPlugin {
    fn http_result(&self, _: &str) -> Rc<dyn Any> { Rc::new("Weather".to_string()) }
    fn get_rules(&self) -> Vec<String> { Vec::new() }
    fn upcast(self: Rc<Self>) -> Rc<dyn RuleTrait> { self }

    fn init(_: &mut SmartCalc, ctx: &Context, requests: Rc<RequestManager>) -> Result<Rc<Self>, PluginError> {
        let mut weather = Self::default();
        weather.requests = requests.clone();
        weather.context = ctx.clone();
        weather.update(ctx)?;
        Ok(Rc::new(weather))
    }

    fn update(&mut self, ctx: &Context) -> Result<(), PluginError> {
        self.loading();
        self.request = Some(Request::get("https://api.coincap.io/v2/assets", ctx));
        Ok(())
    }

    fn process(&mut self) {
        println!("process");
        match &self.request {
            Some(promise) => {
                println!("get_data");
                match promise.get_data() {
                    Some(response) => {
                        println!("coin received");

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

impl RuleTrait for WeatherPlugin {
    fn name(&self) -> String {
        "Weather".to_string()
    }

    fn call(&self, smartcalc: &SmartCalcConfig, fields: &BTreeMap<String, TokenType>) -> Option<TokenType> {
        if fields.contains_key("coin") {
            let coin_name = get_text("coin", fields).unwrap().to_lowercase();
            self.requests.add(&self.name(), Request::get("https://api.coincap.io/v2/assets", &self.context));
            return None;
        } else {
            return None;
        }
     }
}
