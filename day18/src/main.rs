#[macro_use]
extern crate nom;

use std::str;
use nom::{alpha, alphanumeric, anychar, space};
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use std::thread;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::clone::Clone;

named!(name<&str>, map_res!(alpha, str::from_utf8));
named!(value<&str>, map_res!(alphanumeric, str::from_utf8));

named!(
    instruction<Instruction>,
    do_parse!(
        n: name >> space >> r: anychar >> opt!(complete!(space)) >> s: opt!(complete!(char!('-')))
            >> v: opt!(complete!(value)) >> (Instruction {
            name: n.to_string(),
            register: r,
            value: v.map(|val| {
                let mut new_val = s.map_or("", |sign| "-").to_string();
                new_val.extend(val.chars());

                new_val
            }),
        })
    )
);

#[derive(Debug, Clone)]
struct Instruction {
    name: String,
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
    // println!(
    //     "Last play sound frequency is: {:?}",
    //     get_val_of_recovered_freq(instructions.as_slice())
    // );
    // p0 -> p1 channel
    let (tx01, rx01) = mpsc::channel();
    // p1 -> p0 channel
    let (tx10, rx10) = mpsc::channel();
    // p1 -> main thread
    let (tx_count, rx_count) = mpsc::channel();

    let instructions_p0 = instructions.clone();
    let instructions_p1 = instructions.clone();

    // spawn program 0
    let tx_count_copy = mpsc::Sender::clone(&tx_count);
    thread::spawn(move || {
        run_program(0, tx01, rx10, tx_count_copy, instructions_p0.as_slice());
    });

    // spawn program 1
    thread::spawn(move || {
        run_program(1, tx10, rx01, tx_count, instructions_p1.as_slice());
    });

    let mut rcv_count = 0;
    for count in rx_count {
        rcv_count = count;
    }

    println!("Program 1 has sent: {} times", rcv_count);
}

fn run_program(
    program_id: i64,
    tx: Sender<i64>,
    rx: Receiver<i64>,
    tx_count: Sender<i64>,
    instructions: &[Instruction],
) {
    let mut registers = instructions.iter().map(|i| i.register).fold(
        HashMap::new(),
        |mut acc, v| {
            acc.entry(v).or_insert(if program_id == 0 { 0 } else { 1 });
            acc
        },
    );

    let mut instr_counter = 0i64;
    let mut snd_count = 0i64;
    loop {
        let i = &instructions[instr_counter as usize];
        match i.name.as_str() {
            "snd" => {
                let tx_val =
                    get_input_val_or_register_val(i.register.to_string().as_str(), &registers);
                tx.send(tx_val).unwrap();
                snd_count += 1;

                println!(
                    "program {} -> snd: {:?}, counter: {}",
                    program_id, tx_val, instr_counter
                );
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
            "rcv" => match rx.recv() {
                Ok(v) => {
                    println!(
                        "program {} -> rcv: {:?}, counter: {}",
                        program_id, v, instr_counter
                    );
                    *registers.entry(i.register).or_insert(0) = v;
                }
                Err(_) => {
                    println!("*************Err***********");
                    tx_count.send(snd_count).unwrap();
                    return;
                }
            },

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
        match i.name.as_str() {
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
