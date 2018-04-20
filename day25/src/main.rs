#[macro_use]
extern crate pretty_assertions;
#[macro_use]
extern crate nom;

use nom::*;
use std::fs::File;
use std::io::Read;
use std::str;

fn main() {
    let path = "input.txt";
    let mut input = File::open(path).expect("Unable to open file!");
    let mut input_txt = String::new();
    match input.read_to_string(&mut input_txt) {
        Err(_) => return,
        Ok(n) => println!("Read {} bytes", n),
    }

    // let mut rules = Vec::new();
    for line in input_txt.lines() {
        // rules.push(rule(line.as_bytes()).unwrap().1);
    }
    // println!("{:#?}", rules);
}
named!(
    begin_state<char>,
    do_parse!(tag!("Begin in state") >> space >> s: anychar >> anychar >> eol >> (s))
);
named!(
    steps_to_diagnostic<u32>,
    do_parse!(
        tag!("Perform a diagnostic checksum after") >> space >> s: anychar >> space >> tag!("steps")
            >> anychar >> eol >> eol >> (s.to_digit(10).unwrap())
    )
);

named!(
    curr_value<u32>,
    do_parse!(
        tag!("  If the current value is") >> space >> s: anychar >> char!(':') >> eol
            >> (s.to_digit(10).unwrap())
    )
);

named!(
    write_val<u32>,
    do_parse!(
        tag!("    - Write the value") >> space >> s: anychar >> char!('.') >> eol
            >> (s.to_digit(10).unwrap())
    )
);

named!(
    move_cursor<i32>,
    do_parse!(
        tag!("    - Move one slot to the") >> space
            >> dir: map_res!(alt!(tag!("right") | tag!("left")), str::from_utf8)
            >> char!('.') >> eol >> (if dir == "left" { -1 } else { 1 })
    )
);

named!(
    next_state<char>,
    do_parse!(tag!("    - Continue with state") >> space >> s: anychar >> char!('.') >> eol >> (s))
);

// named!(
//     chars<Vec<char>>,
//     map!(
//         map_res!(take_till!(|ch| ch != b'.' && ch != b'#'), str::from_utf8),
//         |s| s.chars().collect::<Vec<_>>()
//     )
// );

// named!(
//     pixels<Vec<Vec<char>>>,
//     separated_list_complete!(char!('/'), chars)
// );

// named!(
//     rule<Rule>,
//     do_parse!(p: pixels >> space >> tag!("=>") >> space >> r: pixels >> (Rule::from_pixels(p, r)))
// );

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_begin_state() {
        assert_eq!(
            begin_state(b"Begin in state A.\n"),
            IResult::Done(&b""[..], 'A')
        );
    }
    #[test]
    fn test_steps_to_diagnostic() {
        assert_eq!(
            steps_to_diagnostic(b"Perform a diagnostic checksum after 6 steps.\n\n"),
            IResult::Done(&b""[..], 6)
        );
    }
    #[test]
    fn test_curr_state() {
        assert_eq!(
            curr_value(b"  If the current value is 0:\n"),
            IResult::Done(&b""[..], 0)
        );
    }

    #[test]
    fn test_write_val() {
        assert_eq!(
            write_val(b"    - Write the value 1.\n"),
            IResult::Done(&b""[..], 1)
        );
    }

    #[test]
    fn test_move_cursor() {
        assert_eq!(
            move_cursor(b"    - Move one slot to the right.\n"),
            IResult::Done(&b""[..], 1)
        );
    }

    #[test]
    fn test_next_state() {
        assert_eq!(
            next_state(b"    - Continue with state B.\n"),
            IResult::Done(&b""[..], 'B')
        );
    }

}
