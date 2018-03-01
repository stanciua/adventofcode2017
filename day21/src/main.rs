#[macro_use]
extern crate nom;

use nom::*;
use std::str;
use std::fs::File;
use std::io::Read;

fn main() {
    let path = "input.txt";
    let mut input = File::open(path).expect("Unable to open file!");
    let mut input_txt = String::new();
    match input.read_to_string(&mut input_txt) {
        Err(_) => return,
        Ok(n) => println!("Read {} bytes", n),
    }

    let mut rules = Vec::new();
    for line in input_txt.lines() {
        rules.push(rule(line.as_bytes()).unwrap().1);
    }
    println!("{:#?}", rules);
}
named!(
    chars<Vec<char>>,
    map!(
        map_res!(take_till!(|ch| ch != b'.' && ch != b'#'), str::from_utf8),
        |s| s.chars().collect::<Vec<_>>()
    )
);

named!(
    pixels<Vec<Vec<char>>>,
    separated_list_complete!(char!('/'), chars)
);

named!(
    rule<Rule>,
    do_parse!(p: pixels >> space >> tag!("=>") >> space >> r: pixels >> (Rule::from_pixels(p, r)))
);
// ##/## => ..#/#.#/..#
// .../.../... => .#../#..#/#.../.#..
// first row -> last column
// ...
// last row -> first column

#[derive(Debug)]
struct Rule {
    rule: Vec<Vec<char>>,
    enhancement: Vec<Vec<char>>,
}

impl Rule {
    fn from_pixels(rule: Vec<Vec<char>>, enhancement: Vec<Vec<char>>) -> Rule {
        Rule {
            rule: rule,
            enhancement: enhancement,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_chars_match() {
        assert_eq!(chars(b"##"), IResult::Done(&b""[..], vec!['#', '#']));
    }
    #[test]
    fn test_chars_separated_match() {
        assert_eq!(
            pixels(b"##/##"),
            IResult::Done(&b""[..], vec![vec!['#', '#'], vec!['#', '#']])
        );
    }
}

fn rotate_clockwise_90_deg(input: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut output = Vec::new();

    // 1 2
    // 3 4
    // ->
    //
    output
}
