use smartcalc::SmartCalc;
use chrono::TimeZone;
use chrono::Local;
use chrono_tz::{Tz, OffsetName};

pub struct Calculation {
    pub code: String,
    pub outputs: Vec<String>,
    pub smartcalc: SmartCalc
}

impl Calculation {
    pub fn new() -> Self {
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
    
        let mut smartcalc = SmartCalc::default();
        
        smartcalc.set_decimal_seperator(",".to_string());
        smartcalc.set_thousand_separator(".".to_string());
        smartcalc.set_timezone(timezone).unwrap();

        Self {
            code: "data = 1
data * 2
1024 dkk".to_string(),
            outputs: Vec::new(),
            smartcalc
        }
    }

    pub fn calculate(&mut self) {
        tracing::warn!("Calculate: {}", &self.code);
        let results = self.smartcalc.execute("en", &self.code[..]);
        self.outputs.clear();

        for result in results.lines.iter() {
            match result {
                Some(result) => match &result.result {
                    Ok(line) => { self.outputs.push(line.output.to_string()); },
                    Err(_) => { self.outputs.push("".to_string()); }
                },
                None => { self.outputs.push("".to_string()); }
            }
        }
    }
} 