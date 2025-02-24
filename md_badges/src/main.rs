pub mod cli;
pub mod data;
pub mod icon;

use crate::cli::BadgeCLI;
use clap::Parser;
use icon::Icon;
use std::fs;

fn main() {
    let cli = BadgeCLI::parse();

    let icons_json_path = Icon::get_icons_json_path();
    // Fetch and dump the icons if not exists
    if !icons_json_path.exists() {
        let icons = Icon::load();
        Icon::dump(&icons).expect("Error while dumping the icons!");
        println!("Icons cached at {:?}", icons_json_path);
    }

    if cli.fetch_icons {
        if icons_json_path.exists() {
            fs::remove_file(&icons_json_path)
                .expect(&format!("Error while deleting `{:?}`", icons_json_path))
        }

        // Now load the icons from web and dump to path
        println!("Downloading icons from web...");
        let icons = Icon::load();
        Icon::dump(&icons).expect("Error while dumping the icons!");
        println!("Icons cached at {:?}", icons_json_path);
    }

    let icons = Icon::load();
    let queried_icons = cli.icons_from_slug(&icons);
    cli.print_icons(&queried_icons);
}
