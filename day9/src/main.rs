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

    for line in input_txt.lines() {
        let clean_chars = remove_cleanup_chars(line);
        let clean_garbage = remove_garbage_chars(&clean_chars)
            .chars()
            .filter(|&c| c != ',')
            .collect::<String>();
        println!("Group: {:?}", clean_garbage);
        println!("Group score is: {:?}", count_group_score(&clean_garbage));
    }
}

fn remove_garbage_chars(group: &str) -> String {
    group
        .chars()
        .fold((String::new(), false), |mut acc, c| {
            if c == '<' && !acc.1 {
                acc.1 = true;
            }
            if !acc.1 {
                acc.0.push(c);
            }

            if c == '>' && acc.1 {
                acc.1 = false;
            }
            acc
        })
        .0
}

fn remove_cleanup_chars(group: &str) -> String {
    group
        .chars()
        .fold((String::new(), 'i'), |mut acc, c| {
            if c != '!' && acc.1 != '!' {
                acc.0.push(c);
            }
            // we need to reset the previoius char if both current
            // and previous char are '!'
            if c == '!' && acc.1 == '!' {
                acc.1 = 'i'
            } else {
                acc.1 = c;
            }
            acc
        })
        .0
}

fn count_group_score(group: &str) -> i32 {
    group
        .chars()
        .fold((0, 0), |mut acc, c| {
            // increment by nesting level
            if c == '{' {
                acc.1 += 1;
                acc.0 += acc.1;
            }

            if c == '}' {
                acc.1 -= 1;
            }
            acc
        })
        .0
}
