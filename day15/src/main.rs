fn main() {
    let mut prev_val_a = 618;
    let mut prev_val_b = 814;
    let factor_a = 16807;
    let factor_b = 48271;

    println!(
        "{:?}",
        count_matches(
            40_000_000,
            &mut prev_val_a,
            &mut prev_val_b,
            factor_a,
            factor_b
        )
    );
}

fn count_matches(
    iterations: u64,
    prev_val_a: &mut u64,
    prev_val_b: &mut u64,
    factor_a: u64,
    factor_b: u64,
) -> u64 {
    let mut count = 0;
    for _ in 0..iterations {
        *prev_val_a = generate_val(*prev_val_a, factor_a);
        *prev_val_b = generate_val(*prev_val_b, factor_b);

        if *prev_val_a & <u16>::max_value() as u64 == *prev_val_b & <u16>::max_value() as u64 {
            count += 1;
        }
    }
    count
}
fn generate_val(prev_val: u64, factor: u64) -> u64 {
    prev_val * factor % 2147483647
}
