use std::error;

use serde::{Deserialize, Serialize};

const SLUG_DATA_URL: &str = "https://github.com/simple-icons/simple-icons/raw/develop/slugs.md";
const ICON_JSON_URL: &str =
    "https://github.com/simple-icons/simple-icons/raw/develop/data/simple-icons.json";

#[derive(Deserialize, Serialize)]
pub struct IconSlug {
    pub title: String,
    pub slug: String,
}

#[derive(Deserialize, Serialize)]
pub struct IconHex {
    pub title: String,
    pub hex: String,
}

impl IconSlug {
    pub fn fetch() -> Result<Vec<IconSlug>, Box<dyn error::Error>> {
        let response = ureq::get(SLUG_DATA_URL).call()?.into_string()?;

        let mut slug_map: Vec<IconSlug> = Vec::new();
        for line in response.lines().skip(9) {
            let parts: Vec<&str> = line
                .strip_prefix("| `")
                .unwrap()
                .strip_suffix("` |")
                .unwrap()
                .split("` | `")
                .collect();
            if let [title, slug] = &parts[..] {
                slug_map.push(IconSlug {
                    title: title.to_string(),
                    slug: slug.to_string(),
                });
            }
        }
        Ok(slug_map)
    }
}

impl IconHex {
    pub fn fetch() -> Result<Vec<IconHex>, Box<dyn error::Error>> {
        let response = ureq::get(ICON_JSON_URL)
            .call()?
            .into_json::<serde_json::Value>()?
            .to_owned();
        let hex_data = serde_json::from_value::<Vec<IconHex>>(response).unwrap();
        Ok(hex_data)
    }
}
