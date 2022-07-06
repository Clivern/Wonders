// Copyright 2022 Clivern. All rights reserved.
// Use of this source code is governed by the MIT
// license that can be found in the LICENSE file.

use reqwest;
use std::fs::File;
use std::io::copy;
use std::path::Path;
use uuid::Uuid;

pub fn download_image(url: String, cache_dir: String) -> Result<String, String> {
    let binding = url.to_owned();
    let ext = Path::new(&binding)
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("");

    let id = Uuid::new_v4();
    let name = format!("{}/{}.{}", cache_dir, id, ext);

    match reqwest::blocking::get(url.to_owned()) {
        Ok(response) => match File::create(name.to_owned()) {
            Ok(mut file) => {
                match copy(&mut response.bytes().unwrap().as_ref(), &mut file) {
                    Ok(_) => Ok(name),
                    Err(_) => Err(String::from("Unable to store image locally.")),
                }
            }
            Err(_) => Err(String::from("Unable to create image locally.")),
        },
        Err(_) => Err(String::from("Unable to fetch image.")),
    }
}
