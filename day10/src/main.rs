use std::str;
use std::fs::File;
use std::io::Read;

// The list begins as [0] 1 2 3 4 (where square brackets indicate the current position).
// The first length, 3, selects ([0] 1 2) 3 4 (where parentheses indicate the sublist to be reversed).
// After reversing that section (0 1 2 into 2 1 0), we get ([2] 1 0) 3 4.
// Then, the current position moves forward by the length, 3, plus the skip size, 0: 2 1 0 [3] 4. Finally, the skip size increases to 1.

// The second length, 4, selects a section which wraps: 2 1) 0 ([3] 4.
// The sublist 3 4 2 1 is reversed to form 1 2 4 3: 4 3) 0 ([1] 2.
// The current position moves forward by the length plus the skip size, a total of 5, causing it not to move because it wraps around: 4 3 0 [1] 2. The skip size increases to 2.

// The third length, 1, selects a sublist of a single element, and so reversing it has no effect.
// The current position moves forward by the length (1) plus the skip size (2): 4 [3] 0 1 2. The skip size increases to 3.

// The fourth length, 5, selects every element starting with the second: 4) ([3] 0 1 2. Reversing this sublist (3 0 1 2 4 into 4 2 1 0 3) produces: 3) ([4] 2 1 0.
// Finally, the current position moves forward by 8: 3 4 2 1 [0]. The skip size increases to 4.

fn main() {
    let path = "input.txt";
    let mut input = File::open(path).expect("Unable to open file!");
    let mut input_txt = String::new();
    match input.read_to_string(&mut input_txt) {
        Err(_) => return,
        Ok(n) => println!("Read {} bytes", n),
    }

    let lengths = input_txt
        .split(',')
        .map(|s| str::parse::<i32>(s).unwrap())
        .collect::<Vec<_>>();

    println!(
        "Result is: {:?}",
        get_result(&mut (0..256).collect::<Vec<_>>(), lengths.as_slice()),
    );
}

fn get_result(list: &mut [i32], lengths: &[i32]) -> i32 {
    let len = list.len();
    lengths
        .iter()
        .fold((list, 0i32, 0i32), |mut state, &n| {
            let sublist = state
                .0
                .iter()
                .cycle()
                .skip(state.1 as usize)
                .take(len)
                .cloned()
                .collect::<Vec<_>>();

            let mut rev_sublist = sublist
                .iter()
                .take(n as usize)
                .collect::<Vec<_>>()
                .into_iter()
                .cloned()
                .rev()
                .collect::<Vec<_>>();
            rev_sublist.extend(
                sublist
                    .into_iter()
                    .skip(n as usize)
                    .take(len - n as usize)
                    .collect::<Vec<_>>(),
            );

            for (idx, e) in rev_sublist.into_iter().enumerate() {
                state.0[(state.1 as usize + idx) % len] = e;
            }

            state.1 += n + state.2;
            state.2 += 1;
            state
        })
        .0
        .iter()
        .take(2)
        .product()
}
