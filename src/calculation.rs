use std::collections::HashMap;

use smartcalc::SmartCalc;
use chrono::TimeZone;
use chrono::Local;
use chrono_tz::{Tz, OffsetName};

use crate::config::TIMEZONE_LIST;
use crate::config::Timezone;

#[derive(Default, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Calculation {
    pub code: String,

    #[serde(skip)]
    pub outputs: Vec<Result<String, String>>,

    #[serde(skip)]
    pub smartcalc: SmartCalc
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Settings {
    pub decimal_seperator: String,
    pub thousand_separator: String,
    pub timezone: Timezone,
    pub date_format: String,
    pub enabled_plugins: HashMap<String, bool>
}

impl Default for Settings {
    fn default() -> Self {
        let timezone = match localzone::get_local_zone() {
            Some(tz) => match tz.parse::<Tz>() {
                Ok(tz) => {
                    let date_time = Local::today().naive_local();
                    tz.offset_from_utc_date(&date_time).abbreviation().to_string()
                },
                Err(_) => "UTC".to_string()
            },
            None => "UTC".to_string()
        };

        let timezone = match TIMEZONE_LIST.iter().find(|tz| tz.name == timezone || tz.data == timezone) {
            Some(tz) => tz.clone(),
            None => TIMEZONE_LIST[TIMEZONE_LIST.len() - 1].clone()
        };

        Self {
            timezone,
            decimal_seperator: ",".to_string(),
            thousand_separator: ".".to_string(),
            enabled_plugins: HashMap::new(),
            date_format: "dd/MM/yyyy".to_string()
        }
    }
}

impl Calculation {
    pub fn new() -> Self {
        Self {
            code: "data = 1
data * 2
1024 dkk".to_string(),
            outputs: Vec::new(),
            smartcalc: SmartCalc::default()
        }
    }

    pub fn set(&mut self, settings: &Settings) {
        self.smartcalc.set_decimal_seperator(settings.decimal_seperator.to_string());
        self.smartcalc.set_thousand_separator(settings.thousand_separator.to_string());
        if let Err(error) = self.smartcalc.set_timezone(settings.timezone.data.to_string()) {
            tracing::warn!("Timezone not valid. Error: {}", error);
            self.smartcalc.set_timezone("UTC".to_string()).unwrap();
        }
    }

    pub fn calculate(&mut self) {
        let results = self.smartcalc.execute("en", &self.code[..]);
        self.outputs.clear();

        for result in results.lines.iter() {
            match result {
                Some(result) => match &result.result {
                    Ok(line) => { self.outputs.push(Ok(line.output.to_string())); },
                    Err(error) => { self.outputs.push(Err(error.to_string())); }
                },
                None => { self.outputs.push(Ok("".to_string())); }
            }
        }
    }
} 