use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap};
use std::fmt::Debug;
use std::ops::Deref;
use std::rc::Rc;

use eframe::egui::Context;
use smartcalc::{TokenType, SmartCalcAstType};
use smartcalc::{RuleTrait, SmartCalc};

use crate::app::State;
use crate::http::Request;


mod coin;
mod city_time;

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
        smartcalc.delete_rule("en".to_string(), self.name());
    }
}

#[derive(Debug)]
pub enum PluginError {
   #[allow(dead_code)]
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
    pub plugins: Vec<Plugin>,
    requests: Rc<RequestManager>,
    ongoing_request: bool
}

impl PluginManager {
    fn add_plugin<T: 'static + PluginTrait>(&mut self, smartcalc: &mut SmartCalc, enabled_plugins: &mut HashMap<String, bool>, ctx: &Context) {
        match T::init(smartcalc, ctx, self.requests.clone()) {
            Ok(plugin) => {                
                tracing::info!("Plugin added: {}", plugin.name());
                self.plugins.push(Plugin::new(plugin.clone()));
                if !enabled_plugins.contains_key(&plugin.name()) {
                    enabled_plugins.insert(plugin.name(), true);
                }
            },
            Err(error) => {
                tracing::warn!("Plugin configure error: {:?}", error);
            }
        };
    }
    
    pub fn build(&mut self, smartcalc: &mut SmartCalc, enabled_plugins: &mut HashMap<String, bool>, ctx: &Context) {
        self.add_plugin::<coin::CoinPlugin>(smartcalc, enabled_plugins, ctx);
        self.add_plugin::<city_time::CityTimePlugin>(smartcalc, enabled_plugins, ctx);


        for plugin in self.plugins.iter() {
            plugin.enable(smartcalc);
        }
    }

    pub fn ongoing_request(&self) -> bool {
        self.ongoing_request
    }
    
    pub fn process(&mut self, _: &Context, state: &mut State) {
        let mut finished_requests = Vec::new();
        let mut requests = self.requests.requests.borrow_mut();
        //self.requests.requests.borrow_mut().retain(|_, request| request.get_data().is_some());
        for (index, (plugin_name, request)) in requests.iter().enumerate() {
            match request.get_data() {
                Some(response) => {
                    match self.plugins.iter().find(|plugin| plugin.name() == &plugin_name[..]) {
                        Some(plugin) => {
                            state.recalculate = true;
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

        self.ongoing_request = requests.len() > 0;
    }
}