/*
 * smartcalc-app v1.0.6
 * Copyright (c) Erhan BARIS (Ruslan Ognyanov Asenov)
 * Licensed under the GNU General Public License v2.0.
 */

pub mod coin;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, Response};

pub async fn get<T: for<'a> serde::Deserialize<'a>>(url: String) -> Result<T, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");

    let request = Request::new_with_str_and_init(&url, &opts)?;
    let window = match web_sys::window() {
        Some(window) => window,
        None => return Err("window not found".into())
    };
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into()?;
    let json = JsFuture::from(resp.json()?).await?;
    let data: T = match json.into_serde() {
        Ok(data) => data,
        Err(error) => return Err(format!("JSON parse error: {:?}", error).into())
    };
    Ok(data)
}