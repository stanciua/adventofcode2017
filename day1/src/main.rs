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
    println!(
        "Captcha result for next digit is: {}",
        calc_sum_with_offset(&input_txt, 1)
    );
    println!(
        "Captcha result for half away digit is: {}",
        calc_sum_with_offset(&input_txt, input_txt.len() / 2)
    );
}

fn calc_sum_with_offset(input_txt: &str, offset: usize) -> u32 {
    let mut sum = 0;
    for (idx, c) in input_txt.chars().enumerate() {
        let ch = input_txt.chars().cycle().skip(idx + offset).next().unwrap();

        if c == ch {
            sum += c.to_digit(10).unwrap();
        }
    }
    sum
}
