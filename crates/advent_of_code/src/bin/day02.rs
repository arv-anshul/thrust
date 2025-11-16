use std::{env, fs};

fn read_data_filename<'a>(args: &'a [String]) -> &'a str {
    if args.len() < 2 {
        panic!("Please specify data filepath.");
    }

    match args.last() {
        Some(filename) => filename,
        None => panic!("Please specify data filepath."),
    }
}

fn main() {
    // Read file and split lines
    let args = &env::args().into_iter().collect::<Vec<String>>();
    let result_vec: Vec<bool> = fs::read_to_string(read_data_filename(args))
        .expect("read data file")
        .lines()
        .map(|line| {
            let mut numbers = line.split_whitespace().map(|s| s.parse::<u32>().unwrap());

            // Extract first and second numbers
            let first_num = numbers.next().unwrap();
            let mut curr_num = match numbers.next() {
                Some(value) => value,
                None => return true, // One element row should always be safe
            };

            // Checks for first and second numbers
            let increasing = first_num < curr_num;
            let diff = first_num.abs_diff(curr_num);
            if diff == 0 || diff > 3 {
                return false;
            }

            for next_num in numbers {
                let diff = curr_num.abs_diff(next_num);
                if diff == 0 || diff > 3 {
                    return false;
                }
                if increasing && next_num < curr_num {
                    return false;
                }
                if !increasing && next_num > curr_num {
                    return false;
                }
                curr_num = next_num;
            }
            return true;
        })
        .collect();

    println!(
        "Result of day02: {}",
        result_vec.iter().filter(|&n| *n).count()
    );
}
