/*
 * smartcalc-app v1.0.5
 * Copyright (c) Erhan BARIS (Ruslan Ognyanov Asenov)
 * Licensed under the GNU General Public License v2.0.
 */

extern crate console_error_panic_hook;

use smartcalc::*;
use smartcalc::UiToken;
use core::ops::Deref;
use js_sys::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub struct SmartCalcWeb {
    smartcalc: SmartCalc
}

fn convert_uitoken(token: &UiToken) -> Object {
    let start_ref       = JsValue::from("start");
    let end_ref         = JsValue::from("end");
    let type_ref        = JsValue::from("type");

    let token_object = js_sys::Object::new();
    let token_type = match token.ui_type {
        UiTokenType::Number => 1,
        UiTokenType::Symbol2 => 2,
        UiTokenType::DateTime => 3,
        UiTokenType::Operator => 4,
        UiTokenType::Text => 5,
        UiTokenType::Comment => 9,
        UiTokenType::Symbol1 => 10,
        UiTokenType::VariableUse => 11,
        UiTokenType::VariableDefination => 12,
        UiTokenType::Month => 13
    };

    Reflect::set(token_object.as_ref(), start_ref.as_ref(),  JsValue::from(token.start as u16).as_ref()).unwrap();
    Reflect::set(token_object.as_ref(), end_ref.as_ref(),    JsValue::from(token.end as u16).as_ref()).unwrap();
    Reflect::set(token_object.as_ref(), type_ref.as_ref(),   JsValue::from(token_type).as_ref()).unwrap();
    token_object
}

#[wasm_bindgen]
impl SmartCalcWeb {
    #[wasm_bindgen]
    pub fn default(decimal_seperator: &str, thousand_separator: &str, timezone: &str) -> Self {
        let mut smartcalc = SmartCalc::default();
        smartcalc.set_decimal_seperator(decimal_seperator.to_string());
        smartcalc.set_thousand_separator(thousand_separator.to_string());
        smartcalc.set_timezone(timezone.to_string());

        SmartCalcWeb { smartcalc }
    }
    
    #[wasm_bindgen]
    pub fn load_from_json(json_data: &str, decimal_seperator: &str, thousand_separator: &str, timezone: &str) -> Self {
        let mut smartcalc = SmartCalc::load_from_json(json_data);
        smartcalc.set_decimal_seperator(decimal_seperator.to_string());
        smartcalc.set_thousand_separator(thousand_separator.to_string());
        smartcalc.set_timezone(timezone.to_string());

        SmartCalcWeb { smartcalc }
    }

    #[wasm_bindgen]
    pub fn execute(&self, language: &str, data: &str) -> JsValue {
        self.smartcalc.execute(language, data);
        let status_ref      = JsValue::from("status");
        let result_type_ref = JsValue::from("type");
        let text_ref        = JsValue::from("output");
        let tokens_ref      = JsValue::from("tokens");

        let line_items = js_sys::Array::new();
        let execute_result = self.smartcalc.execute(language, data);
        for result in execute_result.lines {
            let line_object = js_sys::Object::new();
            match result {
                Some(result) => {
                    match &result.result {
                        Ok(line_result) => {
                            let (status, result_type, output) = match line_result.ast.deref() {
                                SmartCalcAstType::Item(item) => {
                                    match item.type_name() {
                                        "NUMBER" =>       (true, 1, line_result.output.to_string()),
                                        "TIME" =>         (true, 2, line_result.output.to_string()),
                                        "PERCENT" =>      (true, 3, line_result.output.to_string()),
                                        "MONEY" =>        (true, 4, line_result.output.to_string()),
                                        "DURATION" =>     (true, 5, line_result.output.to_string()),
                                        "DATE" =>         (true, 6, line_result.output.to_string()),
                                        "DATE_TIME" =>    (true, 6, line_result.output.to_string()),
                                        "MEMORY" =>       (true, 7, line_result.output.to_string()),
                                        "DYNAMIC_TYPE" => (true, 7, line_result.output.to_string()),
                                        _ =>              (false, 0, "".to_string())
                                    }
                                },
                                _ => (false, 0, "".to_string())
                            };

                            Reflect::set(line_object.as_ref(), status_ref.as_ref(),      JsValue::from(status).as_ref()).unwrap();
                            Reflect::set(line_object.as_ref(), result_type_ref.as_ref(), JsValue::from(result_type).as_ref()).unwrap();
                            Reflect::set(line_object.as_ref(), text_ref.as_ref(),        JsValue::from(&output[..]).as_ref()).unwrap();
                        },
                        Err(error) => {
                            Reflect::set(line_object.as_ref(), status_ref.as_ref(),      JsValue::from(false).as_ref()).unwrap();
                            Reflect::set(line_object.as_ref(), result_type_ref.as_ref(), JsValue::from(0).as_ref()).unwrap();
                            Reflect::set(line_object.as_ref(), text_ref.as_ref(),        JsValue::from(&error[..]).as_ref()).unwrap();
                        }
                    };

                    /* Token generation */
                    let token_objects = js_sys::Array::new();
                    for token in result.ui_tokens.iter() {
                        token_objects.push(&convert_uitoken(token));
                    }
                    Reflect::set(line_object.as_ref(), tokens_ref.as_ref(),      token_objects.as_ref()).unwrap();
                },

                None => {
                    Reflect::set(line_object.as_ref(), status_ref.as_ref(),      JsValue::from(false).as_ref()).unwrap();
                    Reflect::set(line_object.as_ref(), result_type_ref.as_ref(), JsValue::from(0).as_ref()).unwrap();
                    Reflect::set(line_object.as_ref(), text_ref.as_ref(),        JsValue::from("").as_ref()).unwrap();
                }
            }
            line_items.push(&line_object.into());
        }

        line_items.into()
    }

    #[wasm_bindgen]
    pub fn update_currency(&mut self, currency: &str, rate: f64, callback: &js_sys::Function) {
        self.smartcalc.update_currency(currency, rate);
    
        let arguments = js_sys::Array::new();
        arguments.push(&JsValue::from(format!("Currency({}) rate updated", currency)));
        callback.apply(&JsValue::null(), &arguments).unwrap();
    }
}

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}


#[cfg(test)]
mod tests {
    use wasm_bindgen_test::*;
    use super::SmartCalcWeb;

    #[wasm_bindgen_test]
    fn state_from_dom_simple() {
        let calculator = SmartCalcWeb::default(",", ".");
        calculator.execute("en", r"
        tomorrow + 3 weeks
        3/3/2021 to 3/3/2000
        12/02/2020 - 11680 days
        jan 28, 2019 - 14 months 33 days
        3:35 am + 7 hours 15 minutes
        
        date information = 11:30
        date information add 1 hour 1 minute 30 second
        
        8 / (45 - 20%)
        
        10% of 200 try
        180 is 10% of what
        
        10% off 200
        
        10 * 20 + 40
        
        $1k earninng / 5 people
        tomorrow + 3 weeks
        3/3/2021 to 3/3/2000
        12/02/2020 - 11680 days
        jan 28, 2019 - 14 months 33 days
        3:35 am + 7 hours 15 minutes
        
        date information = 11:30
        date information add 1 hour 1 minute 30 second
        
        8 / (45 - 20%)
        
        10% of 200 try
        180 is 10% of what
        
        10% off 200
        1024mb + (1024kb * 24)
        
        10 * 20 + 40
        
        $1k earninng / 5 people");
        assert_eq!(1, 1);
    }
}
