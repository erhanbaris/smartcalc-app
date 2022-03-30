use poll_promise::Promise;

use eframe::egui;

pub struct Request {
    promise: Promise<String>,
    pub extra: Option<String>
}

impl Request {
    pub fn get_with_extra(url: &str, ctx: &egui::Context, extra: String) -> Self {
        Request::inner_get(url, ctx, Some(extra))
    }

    pub fn get(url: &str, ctx: &egui::Context) -> Self {
        Request::inner_get(url, ctx, None)
    }

    fn inner_get(url: &str, ctx: &egui::Context, extra: Option<String>) -> Self {
        let ctx = ctx.clone();
        let (sender, promise) = Promise::new();
        let request = ehttp::Request::get(url);
        
        ehttp::fetch(request, move |response: ehttp::Result<ehttp::Response>| {
            println!("Http received");
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
        println!("Http created");

        Self { promise, extra }
    }

    pub fn get_data(&self) -> Option<&String> {
        self.promise.ready()
    }
}