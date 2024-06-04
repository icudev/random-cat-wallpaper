#![windows_subsystem = "windows"]

use reqwest::blocking::Client;
use std::collections::HashMap;

const API_KEY: &str = "live_******"; // Replace with your API key

fn get_image_url(client: &Client) -> Option<String> {
    if
        let Ok(res) = client
            .get("https://api.thecatapi.com/v1/images/search")
            .header("x-api-key", API_KEY)
            .send()
    {
        let response: String = res.text().unwrap();
        let json: Vec<HashMap<String, serde_json::Value>> = serde_json
            ::from_str(&response)
            .unwrap();
        let image_url: String = json[0]["url"].to_string().replace("\"", "");

        return Some(image_url);
    }

    None
}

fn main() {
    let client = Client::new();

    loop {
        if let Some(image_url) = get_image_url(&client) {
            wallpaper::set_from_url(&image_url).unwrap();
        }

        std::thread::sleep(std::time::Duration::from_secs(3600));
    }
}
