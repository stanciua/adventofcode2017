fn main() {
    let mut prev_val_a = 65;
    let mut prev_val_b = 8921;
    let factor_a = 16807;
    let factor_b = 48271;

    let mut iterations = Vec::new();
    for i in 0..5 {
        prev_val_a = generate_val(prev_val_a, factor_a);
        prev_val_b = generate_val(prev_val_b, factor_b);
        iterations.push((prev_val_a, prev_val_b));
    }
    println!("{:?}", iterations);
}

fn generate_val(prev_val: u64, factor: u64) -> u64 {
    prev_val * factor % 2147483647
}

