#![feature(iterator_step_by)]

#[macro_use]
extern crate nom;

use std::str;
use nom::{alpha, alphanumeric, anychar, space};
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
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
    println!(
        "Mulitply was invoked {:?} times.",
        run_program(false, &instructions)
    );

    // read the second part 2 optimized solution and print the result of h
    let path = "optimized_input.txt";
    let mut optimized_input = File::open(path).expect("Unable to open file!");
    let mut optimized_input_txt = String::new();
    match optimized_input.read_to_string(&mut optimized_input_txt) {
        Err(_) => return,
        Ok(n) => println!("Read {} bytes", n),
    }
    let mut instructions = Vec::new();
    for line in optimized_input_txt.lines() {
        instructions.push(instruction(line.as_bytes()).unwrap().1);
    }
    println!(
        "Last value of 'h' register is: {}",
        run_program(true, &instructions)
    );
}

fn run_program(debug_mode: bool, instructions: &[Instruction]) -> i64 {
    let mut regs = instructions
        .iter()
        .map(|i| i.register)
        .filter(|c| !c.is_digit(10))
        .fold(HashMap::new(), |mut acc, v| {
            if v == 'a' && debug_mode {
                acc.entry(v).or_insert(1);
            } else {
                acc.entry(v).or_insert(0);
            }
            acc
        });

    let mut pc = 0i64;
    let mut count = 0i64;
    let no_instructions = instructions.len();
    loop {
        if pc >= no_instructions as i64 {
            break;
        }
        let ref i = instructions[pc as usize];
        match i.name.as_str() {
            "set" => {
                *regs.get_mut(&i.register).unwrap() = get_val(&i.value.as_ref().unwrap(), &regs)
            }
            "sub" => {
                *regs.get_mut(&i.register).unwrap() -= get_val(&i.value.as_ref().unwrap(), &regs)
            }
            "mul" => {
                *regs.get_mut(&i.register).unwrap() *= get_val(&i.value.as_ref().unwrap(), &regs);
                count += 1;
            }
            // optimize the two inner loops away by using a dedicated prime function 
            "prm" => {
                let is_prime = is_prime(get_val(&i.value.as_ref().unwrap(), &regs));
                if !is_prime {
                    *regs.get_mut(&i.register).unwrap() = 0;
                }
            }
            "jnz" => {
                let reg_val = if let Some(r) = i.register.to_digit(10) {
                    r as i64
                } else {
                    regs[&i.register]
                };

                if reg_val != 0 {
                    pc += get_val(&i.value.as_ref().unwrap(), &regs) - 1;
                }
            }

            _ => unreachable!(),
        }
        pc += 1;
    }
    
    if debug_mode {
        regs[&'h']
    } else {
        count
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

fn is_prime(n: i64) -> bool {
    n == 1 || n == 2
        || (2..((n as f64).sqrt() + 1 as f64) as i64)
            .into_iter()
            .map(|v| n % v == 0)
            .all(|v| v == false)
}
