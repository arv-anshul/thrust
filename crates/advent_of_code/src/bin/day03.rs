use std::{env, fs};

use advent_of_code::read_data_filename;
use regex::Regex;

fn mul(a: i32, b: i32) -> i32 {
    a * b
}

fn main() {
    // Read file and split lines
    let args = &env::args().into_iter().collect::<Vec<String>>();
    let result_vec: Vec<i32> = fs::read_to_string(read_data_filename(args))
        .expect("read data file")
        .lines()
        .map(|line| {
            let re = Regex::new(r"mul\((?P<a>\d{1,3}),\s?(?P<b>\d{1,3})\)").unwrap();
            let matches = re.captures_iter(line);

            matches
                .map(|m| {
                    let (_, [a, b]) = m.extract();
                    mul(a.parse().unwrap(), b.parse().unwrap())
                })
                .sum::<i32>()
        })
        .collect();

    println!("Result of day03: {}", result_vec.iter().sum::<i32>());
}
