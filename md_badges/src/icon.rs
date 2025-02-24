use std::io::{BufReader, BufWriter};
use std::path::Path;
use std::process;
use std::{fs::File, path::PathBuf};

use dirs;
use serde::{de::Error, Deserialize, Serialize};

use crate::data::{IconHex, IconSlug};

#[derive(Clone, Deserialize, Serialize)]
pub struct Icon {
    pub title: String,
    pub slug: String,
    pub hex: String,
}

impl Icon {
    pub fn new(title: String, slug: String, hex: String) -> Icon {
        Icon { title, slug, hex }
    }

    pub fn as_url(&self) -> String {
        let formatted_title = self.title.replace("-", "--").replace(" ", "%20");
        format!(
            "https://img.shields.io/badge/{}-{}?logo={}&logoColor=fff",
            formatted_title, self.hex, self.slug,
        )
    }

    pub fn as_markdown(&self) -> String {
        format!("![{}]({})", self.title, self.as_url())
    }

    pub fn as_html(&self) -> String {
        format!(r#"<img src="{}" alt="{}">"#, self.as_url(), self.title)
    }

    pub fn get_icons_json_path() -> PathBuf {
        let home_dir = match dirs::cache_dir() {
            Some(path) => path,
            None => {
                eprintln!("Are you using windows! Need to fix this program for that.");
                process::exit(1);
            }
        };
        let path = Path::new("md_badges_icons.json");
        home_dir.join(path)
    }

    fn load_form_path() -> Result<Vec<Icon>, Box<dyn std::error::Error>> {
        let path = Icon::get_icons_json_path();
        if !path.exists() {
            return Err("path not exists".into());
        };
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        Ok(serde_json::from_reader(reader)?)
    }

    fn load_from_web() -> Vec<Icon> {
        let icons_with_hex = match IconHex::fetch() {
            Ok(val) => val,
            Err(_) => {
                eprintln!("error while fetching icon's hex data");
                process::exit(1);
            }
        };
        let icons_with_slug = match IconSlug::fetch() {
            Ok(val) => val,
            Err(_) => {
                eprintln!("error while fetching icon's slug data");
                process::exit(1);
            }
        };

        let mut icons: Vec<Icon> = Vec::new();
        for slug in icons_with_slug.into_iter() {
            for hex in icons_with_hex.iter() {
                if slug.title == hex.title {
                    icons.push(Icon {
                        title: slug.title,
                        slug: slug.slug,
                        hex: hex.hex.to_owned(),
                    });
                    break;
                }
            }
        }
        icons
    }

    pub fn load() -> Vec<Icon> {
        match Icon::load_form_path() {
            Ok(x) => x,
            Err(_) => Icon::load_from_web(),
        }
    }

    pub fn dump(icons: &Vec<Icon>) -> serde_json::Result<()> {
        let icons_json_path = Icon::get_icons_json_path();
        if !icons_json_path.exists() {
            let file = File::create(icons_json_path).map_err(serde_json::Error::custom)?;
            let writer = BufWriter::new(file);
            serde_json::to_writer(writer, icons)?;
        }
        Ok(())
    }
}
