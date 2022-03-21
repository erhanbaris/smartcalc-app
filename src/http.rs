use poll_promise::Promise;

use eframe::egui;

pub struct Request {
    promise: Promise<String>
}

impl Request {
    pub fn get(url: &str, ctx: &egui::Context) -> Self {
        let ctx = ctx.clone();
        let (sender, promise) = Promise::new();
        let request = ehttp::Request::get(url);
        
        ehttp::fetch(request, move |response: ehttp::Result<ehttp::Response>| {
            ctx.request_repaint();
            let response = match response {
                Ok(response) => match response.text() {
                    Some(response) => response.to_string(),
                    None => {
                        println!("Fetch no data");
                        "".to_string()
                    }
                },
                Err(error) => {
                    println!("Fetch error: {}", error);
                    "".to_string()
                }
            };

            sender.send(response);
        });

        Self { promise }
    }

    pub fn get_data(&self) -> Option<&String> {
        self.promise.ready()
    }
}