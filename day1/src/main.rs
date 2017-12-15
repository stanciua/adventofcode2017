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

    // add the first character at the end in order to make the comparison
    if let Some(first_char) = input_txt.chars().next() {
        input_txt.push(first_char);
    }

    let mut input_txt_iter = input_txt.chars().peekable();
    let mut curr_elem = input_txt_iter.next();
    let mut sum = 0;
    while let Some(_) = curr_elem {
        {
            if let Some(&val) = input_txt_iter.peek() {
                if val == curr_elem.unwrap() {
                    sum += val.to_digit(10).unwrap();
                }
            }
        }
        curr_elem = input_txt_iter.next();
    }
    println!("Captcha result is: {}", sum);
}
