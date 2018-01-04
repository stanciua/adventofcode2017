use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{HashMap, HashSet};

fn main() {
    let path = "input.txt";
    let input = File::open(path).expect("Unable to open file!");
    let buffered = BufReader::new(input);

    let mut count = 0;
    let mut count_anagram = 0;
    for line in buffered.lines() {
        let line = line.unwrap();
        if is_valid_passphrase(&line) {
            count += 1;
        }
        if is_valid_passphrase_anagram(&line) {
            count_anagram += 1;
        }
    }

    println!("Valid passphrase: {}", count);
    println!("Valid passphrase anagram: {}", count_anagram);
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
fn is_valid_passphrase_anagram(line: &str) -> bool {
    let words = line.split_whitespace().fold(Vec::new(), |mut acc, v| {
        acc.push(v.chars().collect::<HashSet<_>>());
        acc
    });
    let mut words_slice = words.as_slice();
    while words_slice.len() > 0 {
        let word = &words_slice[0];
        for other_word in &words_slice[1..] {
            if *word == *other_word {
                return false;
            }
        }
        words_slice = &words_slice[1..];
    }
    true
}
