fn find_value_after_2017(init_val: usize, repeat: usize, steps: usize) -> usize {
    let mut buffer = Vec::with_capacity(repeat + 1);
    buffer.push(init_val);
    let mut curr_pos = 0;
    for i in 1..repeat + 1 {
        curr_pos = (curr_pos + steps) % buffer.len() + 1;
        buffer.insert(curr_pos, i);
    }
    buffer[curr_pos + 1]
}

fn find_value_after_zero(init_val: usize, repeat: usize, steps: usize) -> usize {
    let mut curr_pos = 0;

    let mut last_val_after_zero = 0;
    for i in 1..repeat + 1 {
        if (curr_pos + steps) % i == 0 {
            last_val_after_zero = i;
        }
        curr_pos = (curr_pos + steps) % i + 1;
    }
    last_val_after_zero
}
fn main() {
    println!(
        "The value after 2017 is: {:?}",
        find_value_after_2017(0, 2017, 3)
    );
    println!(
        "The value after 0 is: {:?}",
        find_value_after_zero(0, 50_000_000, 348)
    );
}
