use std::{env, fs};

use regex::Regex;

fn read_data_filename<'a>(args: &'a [String]) -> &'a str {
    if args.len() < 2 {
        panic!("Please specify data filepath.");
    }

    match args.last() {
        Some(filename) => filename,
        None => panic!("Please specify data filepath."),
    }
}

fn mul(a: i32, b: i32) -> i32 {
    a * b
}

fn main() {
    // Read file and split lines
    let args = &env::args().into_iter().collect::<Vec<String>>();
    // Keep file contents alive
    let text = fs::read_to_string(read_data_filename(args)).expect("read data file");

    let do_re = Regex::new(r"do\(\)").unwrap();
    let don_re = Regex::new(r"don't\(\)").unwrap();
    let re = Regex::new(r"mul\((?P<a>\d{1,3}),\s?(?P<b>\d{1,3})\)").unwrap();

    let mut dos_vec: Vec<&str> = vec![];

    // First extract the do's parts (from left to right)
    for part in do_re.split(&text) {
        // Strip the part after don's part (from the end)
        if let Some(spart) = don_re.splitn(part, 2).next() {
            dos_vec.push(spart);
        }
    }

    let mut result = 0;
    for part in dos_vec.iter().flat_map(|&part| re.captures_iter(part)) {
        let (_, [a, b]) = part.extract();
        result += mul(a.parse().unwrap(), b.parse().unwrap())
    }

    println!("Result of day03b: {result}",);
}
