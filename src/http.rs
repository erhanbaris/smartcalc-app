/*
 * smartcalc-app v1.0.8
 * Copyright (c) Erhan BARIS (Ruslan Ognyanov Asenov)
 * Licensed under the GNU General Public License v2.0.
 */

use poll_promise::Promise;

use eframe::egui;

pub struct Request {
    promise: Promise<String>,
    pub extra: Option<String>
}

impl Request {
   #[allow(dead_code)]
    pub fn get_with_extra(url: &str, ctx: &egui::Context, extra: String) -> Self {
        Request::inner_get(url, ctx, Some(extra))
    }

    pub fn get(url: &str, ctx: &egui::Context) -> Self {
        Request::inner_get(url, ctx, None)
    }

    fn inner_get(url: &str, _: &egui::Context, extra: Option<String>) -> Self {
        let (sender, promise) = Promise::new();
        let request = ehttp::Request::get(url);
        
        ehttp::fetch(request, move |response: ehttp::Result<ehttp::Response>| {
            //ctx.request_repaint();
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

        Self { promise, extra }
    }

    pub fn get_data(&self) -> Option<&String> {
        self.promise.ready()
    }
}