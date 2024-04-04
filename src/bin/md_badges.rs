/// Try to complete this program.
use std::collections::HashMap;

const SLUG_DATA_URL: &str = "https://github.com/simple-icons/simple-icons/raw/develop/slugs.md";
const ICON_JSON_URL: &str =
    "https://github.com/simple-icons/simple-icons/raw/develop/_data/simple-icons.json";

fn fetch_slug_data() -> HashMap<String, String> {
    let resp = reqwest::blocking::get(SLUG_DATA_URL)
        .unwrap()
        .text()
        .unwrap();
    let mut slug_map = HashMap::new();
    for line in resp.lines().skip(9) {
        let parts: Vec<&str> = line
            .strip_prefix("| `")
            .unwrap()
            .strip_suffix("` |")
            .unwrap()
            .split("` | `")
            .collect();
        if let [title, slug] = &parts[..] {
            slug_map.insert(title.to_string(), slug.to_string());
        }
    }
    slug_map
}

fn fetch_icon_json_data() -> Vec<serde_json::Value> {
    let resp = reqwest::blocking::get(ICON_JSON_URL)
        .unwrap()
        .json::<HashMap<String, Vec<serde_json::Value>>>()
        .unwrap();
    resp.get("icons").unwrap().to_vec()
}

struct Icon {
    title: String,
    hex: String,
    source: String,
    aliases: Option<HashMap<String, Vec<String>>>,
}

impl Icon {
    fn has_alias(&self) -> bool {
        self.aliases.is_some()
    }

    fn as_url(&self, slug_map: &HashMap<String, String>) -> String {
        let title = if let Some(ref aliases) = self.aliases {
            self.shortest_alias(aliases)
        } else {
            self.title.clone()
        };
        if let Some(slug) = slug_map.get(&title) {
            format!(
                "https://img.shields.io/badge/{title}-{hex}?logo={slug}&logoColor=FFF",
                title = title,
                hex = self.hex,
                slug = slug
            )
        } else {
            eprintln!("slug not found for {}", self.title);
            String::new()
        }
    }

    fn as_markdown(&self, slug_map: &HashMap<String, String>) -> String {
        let title = if let Some(ref aliases) = self.aliases {
            self.shortest_alias(aliases)
        } else {
            self.title.to_lowercase()
        };
        format!(
            "![{title}]({url})",
            title = title,
            url = self.as_url(slug_map)
        )
    }

    fn shortest_alias(&self, aliases: &HashMap<String, Vec<String>>) -> String {
        if let Some((alias, _)) = aliases.iter().max_by_key(|(_, v)| v.len()) {
            alias.clone()
        } else {
            panic!("Object has no alias.")
        }
    }

    fn match_name(&self, s: &str) -> bool {
        self.title.to_lowercase().starts_with(&s.to_lowercase())
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let icons = fetch_icon_json_data();
    let slug_map = fetch_slug_data();

    for arg in args {
        let mut found = false;
        for icon in &icons {
            let temp_icon = Icon {
                title: icon["value"].to_string(),
                hex: icon["hex"].to_string(),
                source: icon["source"].to_string(),
                aliases: icon.get("aliases"),
            };
            if temp_icon.match_name(&arg) {
                println!("{}", temp_icon.as_markdown(&slug_map));
                found = true;
                break;
            }
        }
        if !found {
            eprintln!("Not found! {:?}", arg);
        }
    }
}
