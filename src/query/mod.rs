use std::cell::RefCell;
use std::collections::BTreeMap;
use std::fmt::Debug;
use std::ops::Deref;
use std::rc::Rc;

use eframe::egui::Context;
use smartcalc::{TokenType, SmartCalcAstType};
use smartcalc::{RuleTrait, SmartCalc};

use crate::http::Request;


pub mod coin;
pub mod weather;

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

pub struct Plugin {
    pub plugin: Rc<dyn PluginTrait>
}

impl Plugin {
    pub fn new(plugin: Rc<dyn PluginTrait>) -> Self {
        Self { plugin }
    }

    pub fn name(&self) -> String {
        self.plugin.name().to_owned()
    }

    pub fn enable(&self, smartcalc: &mut SmartCalc) {
        smartcalc.add_rule("en".to_string(), self.plugin.get_rules(), self.plugin.clone().upcast());
    }

    pub fn disable(&self, smartcalc: &mut SmartCalc) {
        //smartcalc.delete_rule("en".to_string(), self.name());
    }
}

#[derive(Debug)]
pub enum PluginError {
    Error(String)
}

pub trait PluginTrait: RuleTrait {
    fn get_rules(&self) -> Vec<String>;
    fn http_result(&self, content: &str, request: Option<String>);
    
    fn init(smartcalc: &mut SmartCalc, ctx: &Context, requests: Rc<RequestManager>) -> Result<Rc<Self>, PluginError> where Self: Sized;
    fn upcast(self: Rc<Self>) -> Rc<dyn RuleTrait>;
}

#[derive(Default)]
pub struct RequestManager {
    pub requests: RefCell<Vec<(String, Request)>>
}

impl RequestManager {
    pub fn add(&self, plugin_name: &String, request: Request) {
        self.requests.borrow_mut().push((plugin_name.to_string(), request));
    }
}

#[derive(Default)]
pub struct PluginManager {
    plugins: Vec<Rc<dyn PluginTrait>>,
    plugins2: Vec<Plugin>,
    requests: Rc<RequestManager>
}

impl PluginManager {
    fn add_plugin<T: 'static + PluginTrait>(&mut self, smartcalc: &mut SmartCalc, ctx: &Context) {
        match T::init(smartcalc, ctx, self.requests.clone()) {
            Ok(plugin) => {
                tracing::info!("Plugin added: {}", plugin.name());
                self.plugins2.push(Plugin::new(plugin.clone()));
                self.plugins.push(plugin);
            },
            Err(error) => {
                tracing::warn!("Plugin configure error: {:?}", error);
            }
        };
    }
    
    pub fn build(&mut self, smartcalc: &mut SmartCalc, ctx: &Context) {
        self.add_plugin::<coin::CoinPlugin>(smartcalc, ctx);
        self.add_plugin::<weather::WeatherPlugin>(smartcalc, ctx);


        for plugin in self.plugins2.iter() {
            plugin.enable(smartcalc);
        }
    }
    
    pub fn process(&mut self, ctx: &Context, smartcalc: &mut SmartCalc) {
        let mut finished_requests = Vec::new();
        let mut requests = self.requests.requests.borrow_mut();
        //self.requests.requests.borrow_mut().retain(|_, request| request.get_data().is_some());
        for (index, (plugin_name, request)) in requests.iter().enumerate() {
            match request.get_data() {
                Some(response) => {
                    match self.plugins2.iter().find(|plugin| plugin.name() == &plugin_name[..]) {
                        Some(plugin) => {
                            finished_requests.push(index);
                            plugin.plugin.http_result(response, request.extra.clone());
                        },
                        _ => ()
                    }
                },
                None => ()
            }
        }

        for index in finished_requests.iter() {
            requests.remove(*index);
        }
    }
}