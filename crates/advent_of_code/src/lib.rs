pub fn read_data_filename<'a>(args: &'a [String]) -> &'a str {
    if args.len() < 2 {
        panic!("Please specify data filepath.");
    }

    match args.last() {
        Some(filename) => filename,
        None => panic!("Please specify data filepath."),
    }
}
