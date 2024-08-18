use std::process;

use crate::icon::Icon;

pub struct BadgeCLI {
    pub query: Vec<String>,
    pub format: BadgeFormat,
}

pub enum BadgeFormat {
    Markdown,
    Url,
    Html,
}

impl BadgeCLI {
    pub fn new(args: &[String]) -> Self {
        if args.len() < 2 {
            eprintln!("error: not enough arguments");
            process::exit(1);
        }
        BadgeCLI {
            query: args[1]
                .clone()
                .split(',')
                .map(|s| s.to_string())
                .filter(|x| !x.is_empty())
                .collect(),
            format: if args.len() == 2 {
                BadgeFormat::Markdown
            } else {
                match args[2].clone().to_lowercase().as_str() {
                    "url" => BadgeFormat::Url,
                    "html" => BadgeFormat::Html,
                    "md" => BadgeFormat::Markdown,
                    _ => {
                        eprintln!("bad format, switching to default 'md'");
                        BadgeFormat::Markdown
                    }
                }
            },
        }
    }

    pub fn get_icon_from_slug(&self, icons: &Vec<Icon>) -> Vec<Vec<Icon>> {
        let mut results: Vec<Vec<Icon>> = Vec::new();
        for slug in &self.query {
            results.push(
                icons
                    .iter()
                    .filter(|icon| icon.slug.contains(slug.to_lowercase().as_str()))
                    .cloned()
                    .collect(),
            );
        }
        return results;
    }

    pub fn pretty_print(&self, icons: &Vec<Icon>) {
        let queried_icons = self.get_icon_from_slug(icons);
        for icons in queried_icons {
            for icon in icons {
                println!(
                    "{}",
                    match self.format {
                        BadgeFormat::Markdown => icon.as_markdown(),
                        BadgeFormat::Url => icon.as_url(),
                        BadgeFormat::Html => icon.as_html(),
                    }
                )
            }
            println!();
        }
    }
}
