use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let path = "input.txt";
    let input = File::open(path).expect("Unable to open file!");
    let buffered = BufReader::new(input);

    let mut checksum = 0;
    for line in buffered.lines() {
        let (min, max) = min_max_val(&line.unwrap());

        checksum += max - min;
    }

    println!("Calculated checksum is: {}", checksum);
}

fn min_max_val(val: &str) -> (i32, i32) {
    let digits = val.split_whitespace()
        .map(|d| d.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let min = *digits.iter().min().unwrap();
    let max = *digits.iter().max().unwrap();
    (min, max)
}
