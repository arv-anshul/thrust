use crate::icon::Icon;
use clap::Parser;
use clap::ValueEnum;

#[derive(ValueEnum, Clone)]
pub enum BadgeFormat {
    Markdown,
    Url,
    Html,
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct BadgeCLI {
    /// Slug of badges
    pub slugs: Vec<String>,

    /// Output format of badges
    #[arg(long, value_enum, default_value_t = BadgeFormat::Markdown)]
    pub format: BadgeFormat,

    /// Force to download the icons from web
    #[arg(long)]
    pub force_download: bool,
}

impl BadgeCLI {
    pub fn icons_from_slug(&self, icons: &[Icon]) -> Vec<Icon> {
        let mut results: Vec<Icon> = Vec::new();
        for slug in &self.slugs {
            let mut found = false;
            for icon in icons.iter() {
                if &icon.slug == slug {
                    results.push(icon.clone());
                    found = true;
                    break;
                }
            }
            if !found {
                eprintln!("Oops! {:?} not found.", slug);
            }
        }
        results
    }

    pub fn print_icons(&self, icons: &Vec<Icon>) {
        for icon in icons {
            match self.format {
                BadgeFormat::Html => println!("{}", icon.as_html()),
                BadgeFormat::Markdown => println!("{}", icon.as_markdown()),
                BadgeFormat::Url => println!("{}", icon.as_url()),
            }
        }
    }
}
