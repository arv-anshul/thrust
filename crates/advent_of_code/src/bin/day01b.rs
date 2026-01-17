use advent_of_code::read_data_filename;
use std::{env, fs};

fn main() {
    let mut left_vec: Vec<u32> = vec![];
    let mut right_vec: Vec<u32> = vec![];

    // Read file and split lines
    let args = &env::args().collect::<Vec<String>>();
    for val in fs::read_to_string(read_data_filename(args))
        .expect("read data file")
        .lines()
    {
        // parse to numbers
        let mut v = val.split_whitespace();

        let left_num = v.next().unwrap().parse::<u32>().unwrap();
        left_vec.push(left_num);

        let right_num = v.next().unwrap().parse::<u32>().unwrap();
        right_vec.push(right_num);
    }

    let score: Vec<u32> = left_vec
        .iter()
        .map(|num| num * right_vec.iter().filter(|&n| *n == *num).count() as u32)
        .collect();

    println!("Result of day02: {}", score.iter().sum::<u32>());
}
