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
use std::time::Duration;

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
    println!(
        "Last play sound frequency is: {:?}",
        get_val_of_recovered_freq(instructions.as_slice())
    );
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

    // we wait for pid 1 to send us the count times he sent a value
    match rx_count.recv() {
        Ok(v) => println!("Program 1 has sent: {} times", v),
        Err(_) => return,
    }
}

fn run_program(
    program_id: i64,
    tx: Sender<i64>,
    rx: Receiver<i64>,
    tx_count: Sender<i64>,
    instructions: &[Instruction],
) {
    let mut regs = instructions
        .iter()
        .map(|i| i.register)
        .filter(|c| !c.is_digit(10))
        .fold(HashMap::new(), |mut acc, v| {
            if v == 'p' {
                acc.entry(v).or_insert(program_id);
            } else {
                acc.entry(v).or_insert(0);
            }
            acc
        });
    let mut pc = 0i64;
    let mut count = 0i64;
    loop {
        let ref i = instructions[pc as usize];
        match i.name.as_str() {
            "snd" => {
                tx.send(get_val(i.register.to_string().as_str(), &regs))
                    .unwrap();
                count += 1;
            }
            "set" => {
                *regs.get_mut(&i.register).unwrap() = get_val(&i.value.as_ref().unwrap(), &regs)
            }
            "add" => {
                *regs.get_mut(&i.register).unwrap() += get_val(&i.value.as_ref().unwrap(), &regs)
            }
            "mul" => {
                *regs.get_mut(&i.register).unwrap() *= get_val(&i.value.as_ref().unwrap(), &regs)
            }
            "mod" => {
                *regs.get_mut(&i.register).unwrap() %= get_val(&i.value.as_ref().unwrap(), &regs)
            }
            "rcv" => match rx.recv_timeout(Duration::from_secs(1)) {
                Ok(v) => {
                    *regs.entry(i.register).or_insert(0) = v;
                }
                Err(_) => {
                    if program_id == 1 {
                        tx_count.send(count).unwrap();
                    }
                    return;
                }
            },

            "jgz" => {
                let reg_val = if let Some(r) = i.register.to_digit(10) {
                    r as i64
                } else {
                    regs[&i.register]
                };

                if reg_val > 0 {
                    pc += get_val(&i.value.as_ref().unwrap(), &regs) - 1;
                }
            }
            _ => unreachable!(),
        }
        pc += 1;
    }
}

fn get_val(value: &str, regs: &HashMap<char, i64>) -> i64 {
    let first_char = value.chars().take(1).next().unwrap();
    if first_char.is_alphabetic() {
        return regs[&first_char];
    } else {
        value.parse::<i64>().unwrap()
    }
}
fn get_val_of_recovered_freq(instructions: &[Instruction]) -> i64 {
    let mut regs = instructions
        .iter()
        .map(|i| i.register)
        .fold(HashMap::new(), |mut acc, v| {
            acc.entry(v).or_insert(0);
            acc
        });

    let mut last_played_sound = 0;
    let mut pc = 0i64;
    loop {
        let i = &instructions[pc as usize];
        match i.name.as_str() {
            "snd" => {
                last_played_sound = regs[&i.register];
            }
            "set" => {
                *regs.get_mut(&i.register).unwrap() = get_val(&i.value.as_ref().unwrap(), &regs)
            }
            "add" => {
                *regs.get_mut(&i.register).unwrap() += get_val(&i.value.as_ref().unwrap(), &regs)
            }
            "mul" => {
                *regs.get_mut(&i.register).unwrap() *= get_val(&i.value.as_ref().unwrap(), &regs)
            }
            "mod" => {
                *regs.get_mut(&i.register).unwrap() %= get_val(&i.value.as_ref().unwrap(), &regs)
            }
            "rcv" => {
                if regs[&i.register] != 0 {
                    break;
                }
            }
            "jgz" => {
                if regs[&i.register] > 0 {
                    pc += get_val(&i.value.as_ref().unwrap(), &regs) - 1;
                }
            }
            _ => unreachable!(),
        }
        pc += 1;
    }
    last_played_sound
}
