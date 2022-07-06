// Copyright 2022 Clivern. All rights reserved.
// Use of this source code is governed by the MIT
// license that can be found in the LICENSE file.

use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Debug, Deserialize, Serialize)]
pub struct Apod {
    copyright: String,
    date: String,
    explanation: String,
    hdurl: String,
    media_type: String,
    service_version: String,
    title: String,
    url: String,
}

pub fn get_image(url: String, api_key: String) -> Result<Apod, String> {
    let client = Client::new();

    match client
        .get(url.to_owned())
        .header("X-API-KEY", api_key.to_owned())
        .send()
        .expect("HTTP reqwest error")
        .text()
    {
        Ok(body) => {
            let apod: Apod = serde_json::from_str(&body.to_owned()).unwrap();
            Ok(apod)
        }
        Err(_) => Err(String::from("Unable to fetch today image!")),
    }
}
