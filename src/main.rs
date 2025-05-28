#![windows_subsystem = "windows"]

use reqwest::blocking::Client;
use std::{ thread, time::Duration };

const API_URL: &str = "https://api.thecatapi.com/v1/images/search";


#[derive(serde::Deserialize)]
struct ImageData {
    url: String,
}

fn get_image_url(client: &Client) -> Option<String>  {
    let res = client
        .get(API_URL)
        .send()
        .ok()?;

    if !res.status().is_success() {
        return None;
    }

    let body = res.text().ok()?;
    let image_list: Vec<ImageData> = serde_json::from_str(&body).ok()?;
    let image = image_list.into_iter().next()?;

    Some(image.url)
}

fn main() {
    let http_client = Client::new();

    loop {
        if let Some(image_url) = get_image_url(&http_client) {
            let _ = wallpaper::set_from_url(&image_url);
        }

        thread::sleep(Duration::from_secs(3600));
    }
}
