#[macro_use]
extern crate nom;

use std::str;
use nom::{alpha, alphanumeric, anychar, space};
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

// snd X plays a sound with a frequency equal to the value of X.
// set X Y sets register X to the value of Y.
// add X Y increases register X by the value of Y.
// mul X Y sets register X to the result of multiplying the value contained in register X by the value of Y.
// mod X Y sets register X to the remainder of dividing the value contained in register X by the value of Y (that is, it sets X to the result of X modulo Y).
// rcv X recovers the frequency of the last sound played, but only when the value of X is not zero. (If it is zero, the command does nothing.)
// jgz X Y jumps with an offset of the value of Y, but only if the value of X is greater than zero. (An offset of 2 skips the next instruction, an offset of -1 jumps to the previous instruction, and so on.)

named!(name<&str>, map_res!(alpha, str::from_utf8));
named!(value<&str>, map_res!(alphanumeric, str::from_utf8));

named!(
    instruction<Instruction>,
    do_parse!(
        n: name >> space >> r: anychar >> opt!(complete!(space)) >> s: opt!(complete!(char!('-')))
            >> v: opt!(complete!(value)) >> (Instruction {
            name: n,
            register: r,
            value: v.map(|val| {
                let mut new_val = s.map_or("", |sign| "-").to_string();
                new_val.extend(val.chars());
                new_val
            }),
        })
    )
);

#[derive(Debug)]
struct Instruction<'a> {
    name: &'a str,
    register: char,
    value: Option<String>,
}

fn main() {
    let path = "input.txt";
    let mut input = File::open(path).expect("Unable to open file!");
    let mut input_txt = String::new();
    match input.read_to_string(&mut input_txt) {
        Err(_) => return,
        Ok(n) => println!("Read {} bytes", n),
    }

    let mut instructions = Vec::new();
    for line in input_txt.lines() {
        instructions.push(instruction(line.as_bytes()).unwrap().1);
    }
    // println!("{:#?}", instructions);
    println!(
        "Last play sound frequency is: {:?}",
        get_val_of_recovered_freq(instructions.as_slice())
    );
}

fn get_input_val_or_register_val(value: &str, registers: &HashMap<char, i64>) -> i64 {
    let first_char = value.chars().take(1).next().unwrap();
    if first_char.is_alphabetic() {
        return registers[&first_char];
    } else {
        value.parse::<i64>().unwrap()
    }
}
fn get_val_of_recovered_freq(instructions: &[Instruction]) -> i64 {
    let mut registers = instructions.iter().map(|i| i.register).fold(
        HashMap::new(),
        |mut acc, v| {
            acc.entry(v).or_insert(0);
            acc
        },
    );

    let mut last_played_sound = 0;
    let mut instr_counter = 0i64;
    loop {
        let i = &instructions[instr_counter as usize];
        match i.name {
            "snd" => {
                last_played_sound = registers[&i.register];
            }
            "set" => {
                *registers.entry(i.register).or_insert(0) =
                    get_input_val_or_register_val(&i.value.as_ref().unwrap(), &registers)
            }
            "add" => {
                *registers.entry(i.register).or_insert(0) +=
                    get_input_val_or_register_val(&i.value.as_ref().unwrap(), &registers)
            }
            "mul" => {
                *registers.entry(i.register).or_insert(0) *=
                    get_input_val_or_register_val(&i.value.as_ref().unwrap(), &registers)
            }
            "mod" => {
                *registers.entry(i.register).or_insert(0) %=
                    get_input_val_or_register_val(&i.value.as_ref().unwrap(), &registers)
            }
            "rcv" => {
                if registers[&i.register] == 0 {
                    instr_counter += 1;
                    continue;
                }
                break;
            }
            "jgz" => {
                if registers[&i.register] == 0 {
                    instr_counter += 1;
                    continue;
                }

                instr_counter +=
                    get_input_val_or_register_val(&i.value.as_ref().unwrap(), &registers);
                continue;
            }
            _ => unreachable!(),
        }
        instr_counter += 1;
    }
    last_played_sound
}
