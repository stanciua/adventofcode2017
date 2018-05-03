#[macro_use]
extern crate pretty_assertions;
#[macro_use]
extern crate nom;
#[macro_use]
extern crate lazy_static;

use nom::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::str;

lazy_static! {
    static ref STATES: HashMap<char, usize> = {
        let mut m = HashMap::new();
        m.insert('A', 0);
        m.insert('B', 1);
        m.insert('C', 2);
        m.insert('D', 3);
        m.insert('E', 4);
        m.insert('F', 5);
        m
    };
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct State {
    curr_val: usize,
    next_val: usize,
    slot: i32,
    next_state: usize,
}

#[derive(Debug, Eq, PartialEq)]
struct Machine {
    begin_state: usize,
    steps_to_diag: usize,
    instructions: Vec<(State, State)>,
}

#[derive(Debug, Eq, PartialEq)]
struct MachineSimulator {
    machine: Machine,
    tape: Vec<usize>,
    cursor: usize,
    curr_state: usize,
}

impl MachineSimulator {
    fn with_machine(machine: Machine) -> MachineSimulator {
        MachineSimulator {
            machine: machine,
            tape: vec![0; 32],
            cursor: 16,
            curr_state: 0,
        }
    }

    fn realloc_tape(&mut self) {
        let old_size = self.tape.len();
        let mut new_tape = vec![0; old_size * 2];
        let new_size = new_tape.len();
        let offset = (new_size - old_size) / 2;
        for (idx, i) in self.tape.iter().enumerate() {
            new_tape[idx + offset] = *i;
        }
        self.cursor = self.cursor + offset;
        self.tape = new_tape;
    }

    fn simulate(&mut self) -> usize {        
        self.curr_state = self.machine.begin_state;
        for _ in 0..self.machine.steps_to_diag {                        
            if self.cursor <= 0 || self.cursor >= self.tape.len() - 1 {
                self.realloc_tape();
            }
            let state = if self.tape[self.cursor] == 0 {
                self.machine.instructions[self.curr_state].0
            } else {
                self.machine.instructions[self.curr_state].1
            };

            // write the value under the cursor
            self.tape[self.cursor] = state.next_val;
            // move cursor to left or right
            if state.slot > 0 {
                self.cursor += 1;
            } else {
                self.cursor -= 1;
            }

            self.curr_state = state.next_state;
            // println!("Self: {:?}", self);
        }
        self.tape.iter().filter(|&&v| v == 1).count()
    }
}

fn main() {
    let path = "input.txt";
    let mut input = File::open(path).expect("Unable to open file!");
    let mut input_txt = String::new();
    match input.read_to_string(&mut input_txt) {
        Err(_) => return,
        Ok(n) => println!("Read {} bytes", n),
    }

    let machine = parse_instructions(input_txt.as_bytes()).unwrap().1;
    let mut machine_simulator = MachineSimulator::with_machine(machine);
    println!(
        "Number of 1's after checksum: {}",
        machine_simulator.simulate()
    );
}
named!(
    begin_state<usize>,
    do_parse!(tag!("Begin in state") >> space >> s: anychar >> anychar >> eol >> (STATES[&s]))
);
named!(
    steps_to_diagnostic<u32>,
    do_parse!(
        tag!("Perform a diagnostic checksum after") >> space >> s: map_res!(digit, str::from_utf8)
            >> space >> tag!("steps") >> anychar >> eol >> eol
            >> (s.parse::<u32>().unwrap())
    )
);

named!(
    in_state<usize>,
    do_parse!(tag!("In state") >> space >> s: anychar >> anychar >> eol >> (STATES[&s]))
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
    next_state<usize>,
    do_parse!(
        tag!("    - Continue with state") >> space >> s: anychar >> char!('.')
            >> opt!(complete!(eol)) >> (STATES[&s])
    )
);

named!(
    one_state<State>,
    do_parse!(
        c: curr_value >> w: write_val >> m: move_cursor >> n: next_state >> (State {
            curr_val: c as usize,
            next_val: w as usize,
            slot: m,
            next_state: n
        })
    )
);

named!(
    two_state<(State, State)>,
    do_parse!(in_state >> s1: one_state >> s2: one_state >> (s1, s2))
);

named!(
    machine_instructions<Vec<(State, State)>>,
    separated_list_complete!(eol, two_state)
);

named!(
    parse_instructions<Machine>,
    do_parse!(
        b: dbg_dmp!(begin_state) >> s: dbg_dmp!(steps_to_diagnostic)
            >> mi: dbg_dmp!(machine_instructions) >> (Machine {
            begin_state: b,
            steps_to_diag: s as usize,
            instructions: mi,
        })
    )
);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_begin_state() {
        assert_eq!(
            begin_state(b"Begin in state A.\n"),
            IResult::Done(&b""[..], STATES[&'A'])
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
            IResult::Done(&b""[..], STATES[&'B'])
        );
    }
    #[test]
    fn test_next_state_no_newline_at_the_end() {
        assert_eq!(
            next_state(b"    - Continue with state B."),
            IResult::Done(&b""[..], STATES[&'B'])
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
                    next_state: STATES[&'B']
                }
            )
        )
    }
    #[test]
    fn test_two_states() {
        assert_eq!(
            two_state(
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
                    State {
                        curr_val: 0,
                        next_val: 1,
                        slot: 1,
                        next_state: 1
                    },
                    State {
                        curr_val: 1,
                        next_val: 0,
                        slot: -1,
                        next_state: 1
                    }
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
                Machine {
                    begin_state: 0,
                    steps_to_diag: 6,
                    instructions: vec![
                        (
                            State {
                                curr_val: 0,
                                next_val: 1,
                                slot: 1,
                                next_state: 1,
                            },
                            State {
                                curr_val: 1,
                                next_val: 0,
                                slot: -1,
                                next_state: 1,
                            },
                        ),
                        (
                            State {
                                curr_val: 0,
                                next_val: 1,
                                slot: -1,
                                next_state: 0,
                            },
                            State {
                                curr_val: 1,
                                next_val: 1,
                                slot: 1,
                                next_state: 0,
                            },
                        ),
                    ],
                }
            )
        )
    }

    #[test]
    fn test_realloc_tape() {
        let machine = Machine {
            begin_state: 0,
            steps_to_diag: 0,
            instructions: vec![]
        };

        let mut machine_simulator = MachineSimulator{
            machine: machine,
            tape: vec![1,2,3,4,5,6,7,8,9,0,1,2,3,4,5,6,7,8,9,0,1,2,3,4,5,6,7,8,9,0,1,2],
            cursor: 31,
            curr_state: 0         
        };

        machine_simulator.realloc_tape();
        println!("tape: {:?}", machine_simulator.tape);
        assert_eq!(machine_simulator.cursor, 47);
        assert_eq!(machine_simulator.tape.len(), 64);
        assert_eq!(machine_simulator.tape, vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,2,3,4,5,6,7,8,9,0,1,2,3,4,5,6,7,8,9,0,1,2,3,4,5,6,7,8,9,0,1,2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0])
    }

}
