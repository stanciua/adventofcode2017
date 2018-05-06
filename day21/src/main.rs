#[macro_use]
extern crate nom;

use nom::*;
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
        vec!['.', '#', '.'],
        vec!['.', '.', '#'],
        vec!['#', '#', '#'],
    ];
    enhance_image(&mut initial_grid, &rules, 18);

    println!(
        "Number of pixels that are on after {} iterations is: {}",
        5,
        initial_grid
            .into_iter()
            .flat_map(|v| v.into_iter())
            .filter(|p| *p == '#')
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
    rule<Rule>,
    do_parse!(p: pixels >> space >> tag!("=>") >> space >> r: pixels >> (Rule::from_pixels(p, r)))
);

#[derive(Debug)]
struct Rule {
    rule: Vec<Vec<char>>,
    enhancement: Vec<Vec<char>>,
}

impl Rule {
    fn from_pixels(rule: Vec<Vec<char>>, enhancement: Vec<Vec<char>>) -> Rule {
        Rule {
            rule: rule,
            enhancement: enhancement,
        }
    }
}

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
fn get_flipped_rotated_squares(square: &Vec<Vec<char>>) -> Vec<Vec<Vec<char>>> {
    let mut output = Vec::new();
    // flip up - down
    let mut square_ud = square.clone();
    flip_ud(square_ud.as_mut_slice());
    output.push(square_ud);

    // flip left - right
    let mut square_lr = square.clone();
    flip_lr(square_lr.as_mut_slice());
    output.push(square_lr);

    // rotate 90 clockwise
    let mut square_rotate = square.clone();
    rotate_clockwise_90_deg(square_rotate.as_mut_slice());
    output.push(square_rotate.clone());

    // flip up - down
    let mut square_ud = square_rotate.clone();
    flip_ud(square_ud.as_mut_slice());
    output.push(square_ud);

    // flip left - right
    let mut square_lr = square_rotate.clone();
    flip_lr(square_lr.as_mut_slice());
    output.push(square_lr);

    // rotate 180 clockwise
    rotate_clockwise_90_deg(square_rotate.as_mut_slice());
    output.push(square_rotate.clone());

    // flip up - down
    let mut square_ud = square_rotate.clone();
    flip_ud(square_ud.as_mut_slice());
    output.push(square_ud);

    // flip left - right
    let mut square_lr = square_rotate.clone();
    flip_lr(square_lr.as_mut_slice());
    output.push(square_lr);

    // rotate 270 clockwise
    rotate_clockwise_90_deg(square_rotate.as_mut_slice());
    output.push(square_rotate.clone());

    // flip up - down
    let mut square_ud = square_rotate.clone();
    flip_ud(square_ud.as_mut_slice());
    output.push(square_ud);

    // flip left - right
    let mut square_lr = square_rotate.clone();
    flip_lr(square_lr.as_mut_slice());
    output.push(square_lr);

    output
}
fn match_rule_to_square(rule: &Vec<Vec<char>>, squares: &Vec<Vec<Vec<char>>>) -> bool {
    for square in squares {
        if rule == square {
            return true;
        }
    }
    false
}

fn enhance_image(image: &mut Vec<Vec<char>>, rules: &Vec<Rule>, no_iterations: u32) {
    let mut enhanced_size = image.len();
    for _ in 0..no_iterations {
        let sub_images: Vec<Vec<char>>;
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
            .map(|s| {
                let flipped_rotated_squares = get_flipped_rotated_squares(&s);
                let rule = rules
                    .iter()
                    .filter(|r| r.rule.len() == s.len())
                    .find(|r| match_rule_to_square(&r.rule, &flipped_rotated_squares))
                    .unwrap();
                rule.enhancement.clone()
            })
            .collect::<Vec<_>>();
        enhanced_size = get_enhanced_size(&squares);
        *image = assemble_sub_images_into_image(&squares, enhanced_size);
    }
}

fn get_enhanced_size(grid: &Vec<Vec<Vec<char>>>) -> usize {
    let no_elements = grid.len() * grid[0].len() * grid[0][0].len();
    for i in 1.. {
        if i * i == no_elements {
            return i;
        }
    }
    0
}

fn split_image_into_subimages<'a>(image: &'a Vec<Vec<char>>, dimension: usize) -> Vec<Vec<char>> {
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
    sub_images: &Vec<Vec<Vec<char>>>,
    enhanced_size: usize,
) -> Vec<Vec<char>> {
    let mut image: Vec<Vec<char>> = vec![vec!['x'; enhanced_size]; enhanced_size];
    for vv in sub_images {
        let mut offset = (0, 0);
        'outer: for r in 0..image.len() {
            for c in 0..image[0].len() {
                if image[r][c] == 'x' {
                    offset.0 = r;
                    offset.1 = c;
                    break 'outer;
                }
            }
        }

        for r in 0..vv.len() {
            for c in 0..vv[0].len() {
                image[r + offset.0][c + offset.1] = vv[r][c];
            }
        }
    }
    image
}
#[cfg(test)]
mod test {
    use super::*;

    #[ignore]
    fn test_chars_match() {
        assert_eq!(chars(b"##"), IResult::Done(&b""[..], vec!['#', '#']));
    }
    #[ignore]
    fn test_chars_separated_match() {
        assert_eq!(
            pixels(b"##/##"),
            IResult::Done(&b""[..], vec![vec!['#', '#'], vec!['#', '#']])
        );
    }
    #[ignore]
    fn rotate_by_90_degrees_2_by_2_matrix() {
        let mut input = vec![vec![1, 2], vec![3, 4]];
        rotate_clockwise_90_deg(input.as_mut_slice());
        assert_eq!(input, vec![vec![3, 1], vec![4, 2]]);
    }
    #[ignore]
    fn rotate_by_90_degrees_3_by_3_matrix() {
        let mut input = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        rotate_clockwise_90_deg(input.as_mut_slice());
        assert_eq!(input, vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]]);
    }

    #[ignore]
    fn rotate_by_90_degrees_4_by_4_matrix() {
        let mut input = vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
            vec![13, 14, 15, 16],
        ];
        rotate_clockwise_90_deg(input.as_mut_slice());
        assert_eq!(
            input,
            vec![
                vec![13, 9, 5, 1],
                vec![14, 10, 6, 2],
                vec![15, 11, 7, 3],
                vec![16, 12, 8, 4],
            ]
        );
    }
    #[ignore]
    fn rotate_by_90_degrees_5_by_5_matrix() {
        let mut input = vec![
            vec![1, 2, 3, 4, 5],
            vec![6, 7, 8, 9, 10],
            vec![11, 12, 13, 14, 15],
            vec![16, 17, 18, 19, 20],
            vec![21, 22, 23, 24, 25],
        ];
        rotate_clockwise_90_deg(input.as_mut_slice());
        assert_eq!(
            input,
            vec![
                vec![21, 16, 11, 6, 1],
                vec![22, 17, 12, 7, 2],
                vec![23, 18, 13, 8, 3],
                vec![24, 19, 14, 9, 4],
                vec![25, 20, 15, 10, 5],
            ]
        );
    }

    #[ignore]
    fn test_flip_ud_2d() {
        let mut matrix = vec![vec![1, 2], vec![3, 4]];
        flip_ud(matrix.as_mut_slice());
        assert_eq!(matrix, vec![vec![3, 4], vec![1, 2]]);
    }
    #[ignore]
    fn test_flip_ud_3d() {
        let mut matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        flip_ud(matrix.as_mut_slice());
        assert_eq!(matrix, vec![vec![7, 8, 9], vec![4, 5, 6], vec![1, 2, 3]]);
    }
    #[ignore]
    fn test_flip_ud_4d() {
        let mut matrix = vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
            vec![13, 14, 15, 16],
        ];
        flip_ud(matrix.as_mut_slice());
        assert_eq!(
            matrix,
            vec![
                vec![13, 14, 15, 16],
                vec![9, 10, 11, 12],
                vec![5, 6, 7, 8],
                vec![1, 2, 3, 4],
            ]
        );
    }
    #[ignore]
    fn test_flip_ud_5d() {
        let mut matrix = vec![
            vec![1, 2, 3, 4, 5],
            vec![6, 7, 8, 9, 10],
            vec![11, 12, 13, 14, 15],
            vec![16, 17, 18, 19, 20],
            vec![21, 22, 23, 24, 25],
        ];
        flip_ud(matrix.as_mut_slice());
        assert_eq!(
            matrix,
            vec![
                vec![21, 22, 23, 24, 25],
                vec![16, 17, 18, 19, 20],
                vec![11, 12, 13, 14, 15],
                vec![6, 7, 8, 9, 10],
                vec![1, 2, 3, 4, 5],
            ]
        );
    }

    // flip_lr tests
    #[ignore]
    fn test_flip_lr_2d() {
        let mut matrix = vec![vec![1, 2], vec![3, 4]];
        flip_lr(matrix.as_mut_slice());
        assert_eq!(matrix, vec![vec![2, 1], vec![4, 3]]);
    }
    #[ignore]
    fn test_flip_lr_3d() {
        let mut matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        flip_lr(matrix.as_mut_slice());
        assert_eq!(matrix, vec![vec![3, 2, 1], vec![6, 5, 4], vec![9, 8, 7]]);
    }
    #[ignore]
    fn test_flip_lr_4d() {
        let mut matrix = vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
            vec![13, 14, 15, 16],
        ];
        flip_lr(matrix.as_mut_slice());
        assert_eq!(
            matrix,
            vec![
                vec![4, 3, 2, 1],
                vec![8, 7, 6, 5],
                vec![12, 11, 10, 9],
                vec![16, 15, 14, 13],
            ]
        );
    }
    #[ignore]
    fn test_flip_lr_5d() {
        let mut matrix = vec![
            vec![1, 2, 3, 4, 5],
            vec![6, 7, 8, 9, 10],
            vec![11, 12, 13, 14, 15],
            vec![16, 17, 18, 19, 20],
            vec![21, 22, 23, 24, 25],
        ];
        flip_lr(matrix.as_mut_slice());
        assert_eq!(
            matrix,
            vec![
                vec![5, 4, 3, 2, 1],
                vec![10, 9, 8, 7, 6],
                vec![15, 14, 13, 12, 11],
                vec![20, 19, 18, 17, 16],
                vec![25, 24, 23, 22, 21],
            ]
        );
    }

    // test match rules by rotating or flipping squares
    #[ignore]
    fn test_rotating_180_degrees() {
        assert_eq!(
            match_rule_to_square(
                &vec![vec!['.', '.'], vec!['.', '#']],
                &vec![vec!['#', '.'], vec!['.', '.']]
            ),
            true
        );
    }
    #[ignore]
    fn test_flip_up_down() {
        assert_eq!(
            match_rule_to_square(
                &vec![vec!['.', '.'], vec!['.', '#']],
                &vec![vec!['.', '#'], vec!['.', '.']]
            ),
            true
        );
    }
    #[ignore]
    fn test_flip_left_right() {
        assert_eq!(
            match_rule_to_square(
                &vec![vec!['.', '.'], vec!['.', '#']],
                &vec![vec!['.', '.'], vec!['#', '.']]
            ),
            true
        );
    }
    #[ignore]
    fn test_no_flip_or_rotate() {
        assert_eq!(
            match_rule_to_square(
                &vec![vec!['.', '.'], vec!['.', '#']],
                &vec![vec!['.', '.'], vec!['.', '#']]
            ),
            true
        );
    }

    #[ignore]
    fn test_image_to_2by2_sub_images() {
        assert_eq!(
            split_image_into_subimages(
                &vec![
                    vec!['1', '2', '3', '4'],
                    vec!['1', '2', '3', '4'],
                    vec!['1', '2', '3', '4'],
                    vec!['1', '2', '3', '4'],
                ],
                2
            ),
            vec![
                &['1', '2'],
                &['1', '2'],
                &['3', '4'],
                &['3', '4'],
                &['1', '2'],
                &['1', '2'],
                &['3', '4'],
                &['3', '4'],
            ]
        )
    }
    #[ignore]
    fn test_image_to_3by3_sub_images() {
        assert_eq!(
            split_image_into_subimages(
                &vec![
                    vec!['1', '2', '3'],
                    vec!['1', '2', '3'],
                    vec!['1', '2', '3'],
                ],
                3
            ),
            vec![&['1', '2', '3'], &['1', '2', '3'], &['1', '2', '3']]
        )
    }

    #[ignore]
    fn test_enhance_image() {
        let mut input = vec![
            vec!['1', '2', '3'],
            vec!['1', '2', '3'],
            vec!['1', '2', '3'],
        ];
        enhance_image(
            &mut input,
            &vec![
                Rule {
                    rule: vec![
                        vec!['1', '2', '3'],
                        vec!['1', '2', '3'],
                        vec!['1', '2', '3'],
                    ],

                    enhancement: vec![
                        vec!['1', '2', '3', '4'],
                        vec!['1', '2', '3', '4'],
                        vec!['1', '2', '3', '4'],
                        vec!['1', '2', '3', '4'],
                    ],
                },
            ],
            1,
        );
        assert_eq!(
            input,
            vec![
                vec!['1', '2', '3', '4'],
                vec!['1', '2', '3', '4'],
                vec!['1', '2', '3', '4'],
                vec!['1', '2', '3', '4'],
            ],
        );
    }

    #[test]
    fn test_assemble_sub_images_into_image() {
        let sub_images = vec![
            vec![
                vec!['1', '2', '3'],
                vec!['1', '2', '3'],
                vec!['1', '2', '3'],
            ],
            vec![
                vec!['1', '2', '3'],
                vec!['1', '2', '3'],
                vec!['1', '2', '3'],
            ],
            vec![
                vec!['1', '2', '3'],
                vec!['1', '2', '3'],
                vec!['1', '2', '3'],
            ],
            vec![
                vec!['1', '2', '3'],
                vec!['1', '2', '3'],
                vec!['1', '2', '3'],
            ],
        ];
        assert_eq!(
            assemble_sub_images_into_image(&sub_images, 6),
            vec![
                vec!['1', '2', '3', '1', '2', '3'],
                vec!['1', '2', '3', '1', '2', '3'],
                vec!['1', '2', '3', '1', '2', '3'],
                vec!['1', '2', '3', '1', '2', '3'],
                vec!['1', '2', '3', '1', '2', '3'],
                vec!['1', '2', '3', '1', '2', '3'],
            ]
        );
    }
    #[test]
    fn test_get_enhanced_size() {
        let sub_images = vec![
            vec![
                vec!['1', '2', '3'],
                vec!['1', '2', '3'],
                vec!['1', '2', '3'],
            ],
            vec![
                vec!['1', '2', '3'],
                vec!['1', '2', '3'],
                vec!['1', '2', '3'],
            ],
            vec![
                vec!['1', '2', '3'],
                vec!['1', '2', '3'],
                vec!['1', '2', '3'],
            ],
            vec![
                vec!['1', '2', '3'],
                vec!['1', '2', '3'],
                vec!['1', '2', '3'],
            ],
        ];
        assert_eq!(get_enhanced_size(&sub_images), 6);
    }
}
