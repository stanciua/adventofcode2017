use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn main() {
    let path = "input.txt";
    let input = File::open(path).expect("Unable to open file!");
    let buffered = BufReader::new(input);

    let mut count = 0;
    for line in buffered.lines() {
        if is_valid_passphrase(&line.unwrap()) {
            count += 1;
        }
    }

    println!("Valid passphrase: {}", count);
}

fn is_valid_passphrase(line: &str) -> bool {
    let mut map: HashMap<&str, i32> = HashMap::new();
    for word in line.split_whitespace().map(|s| s.trim()) {
        let counter = map.entry(word).or_insert(0);
        *counter += 1;
        if *counter > 1 {
            return false;
        }
    }
    true
}
