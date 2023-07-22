use std::{
    fs::File,
    io::{self, BufRead},
};

pub fn read_lines(filename: &str, print_items: bool) -> impl Iterator<Item = String> {
    let file = File::open(filename).unwrap();
    io::BufReader::new(file).lines().map({
        move |l| {
            if print_items {
                println!("{}", l.as_ref().unwrap());
            }
            l.unwrap()
        }
    })
}

pub fn read_file(filename: &str) -> String {
    std::fs::read_to_string(filename).unwrap()
}

pub fn print_line(line: &str) {
    println!("{}", line);
}

pub fn prepare_example(example: &str) -> String {
    String::from(example)
        .lines()
        .map(|l| l.trim())
        .collect::<Vec<_>>()
        .join("\n")
}
