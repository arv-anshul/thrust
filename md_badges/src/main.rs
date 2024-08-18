use std::env;

pub mod cli;
pub mod data;
pub mod icon;

use crate::icon::Icon;

fn main() {
    let args: Vec<String> = env::args().collect();
    let cli = cli::BadgeCLI::new(&args);

    let icons = Icon::load();
    Icon::dump(&icons).unwrap();
    cli.pretty_print(&icons);
}
