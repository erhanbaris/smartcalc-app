use smartcalc::SmartCalc;

use crate::settings::Settings;

#[derive(Default, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Calculation {
    pub code: String,

    #[serde(skip)]
    pub outputs: Vec<Result<String, String>>,

    #[serde(skip)]
    pub smartcalc: SmartCalc
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

    pub fn configure(&mut self, settings: &Settings) {
        self.smartcalc.set_date_rule("en", settings.date_format.datas.to_vec());
        self.smartcalc.set_decimal_seperator(settings.decimal_seperator.to_string());
        self.smartcalc.set_thousand_separator(settings.thousand_separator.to_string());

        if let Err(error) = self.smartcalc.set_timezone(settings.timezone.abbr()) {
            tracing::warn!("Timezone not valid ({}). Error: {}", settings.timezone.abbr(), error);
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