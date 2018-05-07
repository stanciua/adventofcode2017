#[macro_use]
extern crate nom;

use nom::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::str;

fn main() {
    let path = "input.txt";
    let mut input = File::open(path).expect("Unable to open file!");
    let mut input_txt = String::new();
    match input.read_to_string(&mut input_txt) {
        Err(_) => return,
        Ok(n) => println!("Read {} bytes", n),
    }

    let mut rules = Vec::new();
    for line in input_txt.lines() {
        rules.push(rule(line.as_bytes()).unwrap().1);
    }

    let mut initial_grid = vec![
        vec![0,1,0],
        vec![0,0,1],
        vec![1,1,1],
    ];
    let no_iterations = 18;
    let rules = rotate_and_flip_all_rules(rules);
    enhance_image(&mut initial_grid, &rules, no_iterations);

    println!(
        "Number of pixels that are on after {} iterations is: {}",
        no_iterations,
        initial_grid
            .into_iter()
            .flat_map(|v| v.into_iter())
            .filter(|p| *p == 1)
            .count()
    );
}
named!(
    chars<Vec<char>>,
    map!(
        map_res!(take_till!(|ch| ch != b'.' && ch != b'#'), str::from_utf8),
        |s| s.chars().collect::<Vec<_>>()
    )
);

named!(
    pixels<Vec<Vec<char>>>,
    separated_list_complete!(char!('/'), chars)
);

named!(
    rule<(Vec<Vec<u8>>, Vec<Vec<u8>>)>,
    do_parse!(p: pixels >> space >> tag!("=>") >> space >> r: pixels >> (p.iter().map(|v| v.iter().map(|e| if *e == '.' { 0 } else {1}).collect::<Vec<_>>()).collect::<Vec<_>>(), r.iter().map(|v| v.iter().map(|e| if *e == '.' { 0 } else {1}).collect::<Vec<_>>()).collect::<Vec<_>>()))
);

fn rotate_clockwise_90_deg<T: Copy>(input: &mut [Vec<T>]) {
    let size = input.len();

    let no_cycles = (size as f64 / 2.0).floor() as usize;

    let mut width = size;
    for c in 0..no_cycles {
        for i in 0..width - 1 {
            let tmp = input[c + i][c];
            input[c + i][c] = input[width + c - 1][c + i];
            input[width + c - 1][c + i] = input[width + c - i - 1][width + c - 1];
            input[width + c - i - 1][width + c - 1] = input[c][width - i + c - 1];
            input[c][width - i + c - 1] = tmp;
        }
        width -= 2;
    }
}

fn flip_ud<T: Copy>(matrix: &mut [Vec<T>]) {
    let lgth = matrix.len();
    for idx in 0..lgth / 2 {
        assert!(matrix[idx].len() == matrix[lgth - idx - 1].len());
        let row_lgth = matrix[idx].len();
        for i in 0..row_lgth {
            let tmp = matrix[idx][i];
            matrix[idx][i] = matrix[lgth - idx - 1][i];
            matrix[lgth - idx - 1][i] = tmp;
        }
    }
}

fn flip_lr<T: Copy>(matrix: &mut [Vec<T>]) {
    let lgth = matrix.len();
    for idx in 0..lgth / 2 {
        let row_lgth = matrix.len();
        for i in 0..row_lgth {
            let tmp = matrix[i][idx];
            matrix[i][idx] = matrix[i][lgth - idx - 1];
            matrix[i][lgth - idx - 1] = tmp;
        }
    }
}
fn rotate_and_flip_all_rules(
    rules: Vec<(Vec<Vec<u8>>, Vec<Vec<u8>>)>,
) -> HashMap<Vec<Vec<u8>>, Vec<Vec<u8>>> {
    let mut output = HashMap::new();
    for rule in rules {
        // first insert the rule from the input file
        output.insert(rule.0.clone(), rule.1.clone()).is_none();
        // flip up - down
        let mut square_ud = rule.0.clone();
        flip_ud(square_ud.as_mut_slice());
        output.insert(square_ud, rule.1.clone()).is_none();

        // flip left - right
        let mut square_lr = rule.0.clone();
        flip_lr(square_lr.as_mut_slice());
        output.insert(square_lr, rule.1.clone()).is_none();

        // rotate 90 clockwise
        let mut square_rotate = rule.0.clone();
        rotate_clockwise_90_deg(square_rotate.as_mut_slice());

        output
            .insert(square_rotate.clone(), rule.1.clone())
            .is_none();

        // flip up - down
        let mut square_ud = square_rotate.clone();
        flip_ud(square_ud.as_mut_slice());
        output.insert(square_ud, rule.1.clone()).is_none();

        // flip left - right
        let mut square_lr = square_rotate.clone();
        flip_lr(square_lr.as_mut_slice());
        output.insert(square_lr, rule.1.clone()).is_none();

        // rotate 180 clockwise
        rotate_clockwise_90_deg(square_rotate.as_mut_slice());

        output
            .insert(square_rotate.clone(), rule.1.clone())
            .is_none();

        // flip up - down
        let mut square_ud = square_rotate.clone();
        flip_ud(square_ud.as_mut_slice());
        output.insert(square_ud, rule.1.clone()).is_none();

        // flip left - right
        let mut square_lr = square_rotate.clone();
        flip_lr(square_lr.as_mut_slice());
        output.insert(square_lr, rule.1.clone()).is_none();

        // rotate 270 clockwise
        rotate_clockwise_90_deg(square_rotate.as_mut_slice());

        output
            .insert(square_rotate.clone(), rule.1.clone())
            .is_none();

        // flip up - down
        let mut square_ud = square_rotate.clone();
        flip_ud(square_ud.as_mut_slice());
        output.insert(square_ud, rule.1.clone()).is_none();

        // flip left - right
        let mut square_lr = square_rotate.clone();
        flip_lr(square_lr.as_mut_slice());
        output.insert(square_lr, rule.1.clone()).is_none();
    }
    output
}

fn enhance_image(
    image: &mut Vec<Vec<u8>>,
    rules: &HashMap<Vec<Vec<u8>>, Vec<Vec<u8>>>,
    no_iterations: u32,
) {
    let mut enhanced_size = image.len();    
    for _ in 0..no_iterations {        
        let sub_images: Vec<Vec<u8>>;
        if enhanced_size % 2 == 0 {
            sub_images = split_image_into_subimages(image, 2);
        } else {
            sub_images = split_image_into_subimages(image, 3);
        }

        let size = sub_images[0].len();
        let squares = sub_images
            .as_slice()
            .chunks(size)
            .map(|c| c.to_vec())
            .map(|s| rules[&s].clone())
            .collect::<Vec<_>>();
        enhanced_size = get_enhanced_size(&squares);
        *image = assemble_sub_images_into_image(&squares, enhanced_size);
    }
}

fn get_enhanced_size(grid: &Vec<Vec<Vec<u8>>>) -> usize {
    let no_elements = grid.len() * grid[0].len() * grid[0][0].len();
    for i in 1.. {
        if i * i == no_elements {
            return i;
        }
    }
    0
}

fn split_image_into_subimages<'a>(image: &'a Vec<Vec<u8>>, dimension: usize) -> Vec<Vec<u8>> {
    let mut sub_images = Vec::new();
    let mut i = 0;
    let mut j = 0;
    while i <= image.len() - 1 {
        while j <= image[0].len() - 1 {
            for d in 0..dimension {
                sub_images.push((&image[i + d][j..j + dimension]).to_vec());
            }
            j += dimension;
        }
        i += dimension;
        j = 0;
    }
    sub_images
}

fn assemble_sub_images_into_image(
    sub_images: &Vec<Vec<Vec<u8>>>,
    enhanced_size: usize,
) -> Vec<Vec<u8>> {
    let mut image: Vec<Vec<u8>> = vec![vec![0; enhanced_size]; enhanced_size];
    let mut offset = (0, 0);
    for vv in sub_images {        
        for r in 0..vv.len() {            
            for c in 0..vv[0].len() {                
                image[r + offset.0][c + offset.1] = vv[r][c];                
            }            
        }        

        offset.0 += vv.len();
        offset.1 += vv.len();

        if offset.0 == enhanced_size && offset.1 == enhanced_size {            
            return image;
        }

        if offset.1 != enhanced_size {            
            offset.0 -= vv.len();
        } else {
            offset.1 = 0;
        }
    }
    image
}
