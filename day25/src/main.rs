#[macro_use]
extern crate pretty_assertions;
#[macro_use]
extern crate nom;

use nom::*;
use std::fs::File;
use std::io::Read;
use std::str;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct State {
    curr_val: u32,
    next_val: u32,
    slot: i32,
    next_state: char,
}

fn main() {
    let path = "input.txt";
    let mut input = File::open(path).expect("Unable to open file!");
    let mut input_txt = String::new();
    match input.read_to_string(&mut input_txt) {
        Err(_) => return,
        Ok(n) => println!("Read {} bytes", n),
    }

    let (begin_state, steps_to_diag, instructions) =
        parse_instructions(input_txt.as_bytes()).unwrap().1;
    println!("begin state: {:?}", begin_state);
    println!("steps to diagnostic: {:?}", steps_to_diag);
    println!("instructions: {:?}", instructions);
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
    in_state<char>,
    do_parse!(tag!("In state") >> space >> s: anychar >> anychar >> eol >> (s))
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
    do_parse!(
        tag!("    - Continue with state") >> space >> s: anychar >> char!('.')
            >> opt!(complete!(eol)) >> (s)
    )
);

named!(
    one_state<State>,
    do_parse!(
        c: curr_value >> w: write_val >> m: move_cursor >> n: next_state >> (State {
            curr_val: c,
            next_val: w,
            slot: m,
            next_state: n
        })
    )
);

named!(
    two_state<(State, State)>,
    do_parse!(s1: one_state >> s2: one_state >> (s1, s2))
);
named!(
    with_state<(char, (State, State))>,
    do_parse!(s: in_state >> ts: two_state >> (s, ts))
);

named!(
    machine_instructions<Vec<(char, (State, State))>>,
    separated_list_complete!(eol, with_state)
);

named!(
    parse_instructions<(char, u32, Vec<(char, (State, State))>)>,
    do_parse!(b: begin_state >> s: steps_to_diagnostic >> mi: machine_instructions >> (b, s, mi))
);

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
    #[test]
    fn test_next_state_no_newline_at_the_end() {
        assert_eq!(
            next_state(b"    - Continue with state B."),
            IResult::Done(&b""[..], 'B')
        );
    }

    #[test]
    fn test_single_state() {
        assert_eq!(
            one_state(
                r###"  If the current value is 0:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state B.
"###
                    .as_bytes(),
            ),
            IResult::Done(
                &b""[..],
                State {
                    curr_val: 0,
                    next_val: 1,
                    slot: 1,
                    next_state: 'B'
                }
            )
        )
    }
    #[test]
    fn test_two_states() {
        assert_eq!(
            two_state(
                r###"  If the current value is 0:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state B.
  If the current value is 1:
    - Write the value 0.
    - Move one slot to the left.
    - Continue with state B.
"###
                    .as_bytes(),
            ),
            IResult::Done(
                &b""[..],
                (
                    State {
                        curr_val: 0,
                        next_val: 1,
                        slot: 1,
                        next_state: 'B'
                    },
                    State {
                        curr_val: 1,
                        next_val: 0,
                        slot: -1,
                        next_state: 'B'
                    }
                )
            )
        )
    }
    #[test]
    fn test_in_state() {
        assert_eq!(
            with_state(
                r###"In state A:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state B.
  If the current value is 1:
    - Write the value 0.
    - Move one slot to the left.
    - Continue with state B.
"###
                    .as_bytes(),
            ),
            IResult::Done(
                &b""[..],
                (
                    'A',
                    (
                        State {
                            curr_val: 0,
                            next_val: 1,
                            slot: 1,
                            next_state: 'B'
                        },
                        State {
                            curr_val: 1,
                            next_val: 0,
                            slot: -1,
                            next_state: 'B'
                        }
                    )
                )
            )
        )
    }
    #[test]
    fn test_parse_instructions() {
        assert_eq!(
            parse_instructions(
                r###"Begin in state A.
Perform a diagnostic checksum after 6 steps.

In state A:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state B.
  If the current value is 1:
    - Write the value 0.
    - Move one slot to the left.
    - Continue with state B.

In state B:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the left.
    - Continue with state A.
  If the current value is 1:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state A."###
                    .as_bytes(),
            ),
            IResult::Done(
                &b""[..],
                (
                    'A',
                    6,
                    vec![
                        (
                            'A',
                            (
                                State {
                                    curr_val: 0,
                                    next_val: 1,
                                    slot: 1,
                                    next_state: 'B',
                                },
                                State {
                                    curr_val: 1,
                                    next_val: 0,
                                    slot: -1,
                                    next_state: 'B',
                                },
                            ),
                        ),
                        (
                            'B',
                            (
                                State {
                                    curr_val: 0,
                                    next_val: 1,
                                    slot: -1,
                                    next_state: 'A',
                                },
                                State {
                                    curr_val: 1,
                                    next_val: 1,
                                    slot: 1,
                                    next_state: 'A',
                                },
                            ),
                        ),
                    ]
                )
            )
        )
    }

}
