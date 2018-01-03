use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let path = "input.txt";
    let input = File::open(path).expect("Unable to open file!");
    let buffered = BufReader::new(input);

    let mut checksum = 0;
    let mut checksum_divide = 0;
    for line in buffered.lines() {
        let line = line.unwrap();
        let (min, max) = min_max_val(&line);
        checksum += max - min;
        checksum_divide += divide_evenly_val(&line);
    }

    println!("Calculated checksum for min/max is: {}", checksum);
    println!(
        "Calculated checksum for divided evenly is: {}",
        checksum_divide
    );
}

fn min_max_val(val: &str) -> (i32, i32) {
    let digits = val.split_whitespace()
        .map(|d| d.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let min = *digits.iter().min().unwrap();
    let max = *digits.iter().max().unwrap();
    (min, max)
}
fn divide_evenly_val(val: &str) -> i32 {
    let digits = val.split_whitespace()
        .map(|d| d.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let mut sum = 0;
    let mut digits_slice = digits.as_slice();
    for d in digits.iter() {
        digits_slice = &digits_slice[1..];
        for cd in digits_slice {
            if d % cd == 0 {
                sum += d / cd;
                continue;
            }
            if cd % d == 0 {
                sum += cd / d;
            }
        }
    }
    sum
}
