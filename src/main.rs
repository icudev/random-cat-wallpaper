use dotenv::dotenv;
use serde_json;
use std::collections::HashMap;
use wallpaper;

fn get_image(api_key: &String) -> Option<String> {
    let client = reqwest::blocking::Client::new();

    if let Ok(res) = client
        .get("https://api.thecatapi.com/v1/images/search")
        .header("x-api-key", api_key)
        .send()
        {
            let response: String = res.text().unwrap();

            let json: Vec<HashMap<String, serde_json::Value>> = serde_json::from_str(&response).unwrap();
            let image_url: String = json[0]["url"].to_string().replace("\"", "");

            return Some(image_url);
        }
    
    return None;
}

fn main() {
    dotenv().ok();

    let api_key: String = std::env::var("THECATAPI_KEY").expect("Couldn\'t find api key in .env");

    loop {
        if let Some(image_url) = get_image(&api_key) {
            wallpaper::set_from_url(&image_url).unwrap();
        }

        std::thread::sleep(std::time::Duration::from_secs(3600));
    }
}
