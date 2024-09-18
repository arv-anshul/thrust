pub mod cli;
pub mod data;
pub mod icon;

use crate::cli::BadgeCLI;
use clap::Parser;
use icon::Icon;
use std::fs;

fn main() {
    let cli = BadgeCLI::parse();

    if cli.fetch_icons {
        eprintln!("Downloading icons from github...");
        let icons_json_path = Icon::get_icons_json_path();
        if icons_json_path.exists() {
            if fs::remove_file(icons_json_path).is_err() {
                eprintln!("Error while deleting `md_badges.json` file.");
            }
        }
    }

    let icons = Icon::load();
    let queried_icons = cli.icons_from_slug(&icons);
    cli.print_icons(&queried_icons);
}
