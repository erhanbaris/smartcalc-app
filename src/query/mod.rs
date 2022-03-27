use std::any::Any;
use std::cell::{RefCell, Cell};
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
    data: Cell<Rc<dyn Any>>,
    request: Option<Request>,
    plugin: Rc<dyn PluginTrait>
}

impl Plugin {
    pub fn new(plugin: Rc<dyn PluginTrait>) -> Self {
        Self { request: None, plugin, data: Cell::new(Rc::new(false)) }
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

    pub fn process(&mut self) {
        //let a : Rc<RefCell<String>> = Rc::new(RefCell::new(String::new()));
        match &self.request {
            Some(promise) => {
                println!("get_data");
                match promise.get_data() {
                    Some(response) => {
                        self.data.set(self.plugin.http_result(response));
                    },
                    None => ()
                }
            },
            None => ()
        };
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PluginStatus {
    Idle,
    Loading,
    ReadyToProcess,
    Done
}

impl Default for PluginStatus {
    fn default() -> Self { PluginStatus::Idle }
}

#[derive(Debug)]
pub enum PluginError {
    Error(String)
}

pub trait PluginTrait: RuleTrait {
    fn get_rules(&self) -> Vec<String>;
    fn http_result(&self, content: &str) -> Rc<dyn Any>;
    
    fn init(smartcalc: &mut SmartCalc, ctx: &Context) -> Result<Rc<Self>, PluginError> where Self: Sized;
    fn update(&mut self, ctx: &Context) -> Result<(), PluginError>;
    fn process(&mut self);
    fn status(&self) -> PluginStatus;
    fn upcast(self: Rc<Self>) -> Rc<dyn RuleTrait>;
}

#[derive(Default)]
pub struct PluginManager {
    plugins: Vec<Rc<dyn PluginTrait>>,
    plugins2: Vec<Plugin>
}

impl PluginManager {
    fn add_plugin<T: 'static + PluginTrait>(&mut self, smartcalc: &mut SmartCalc, ctx: &Context) {
        match T::init(smartcalc, ctx) {
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
    }
    
    pub fn process(&mut self, calculator: &mut SmartCalc) {
        for plugin in self.plugins2.iter() {
            plugin.enable(calculator);
        }

        for plugin in self.plugins.iter_mut() {
            if plugin.status() == PluginStatus::Loading {
                match Rc::get_mut(plugin) {
                    Some(object) => {
                        println!("process >>");
                        object.process();
                        if object.status() == PluginStatus::ReadyToProcess {
                            //plugin.enable(PluginTrait::upcast(plugin.clone()), calculator);
                        }
                    },
                    None => {
                        println!("Item is none")
                    }
                }
            }
        }
    }
}