#[macro_use]
extern crate nom;
use std::fs::File;
use std::io::Read;
use std::str;
use nom::{alphanumeric, IResult};

named!(
    num<i64>,
    do_parse!(
        s: opt!(tag!("-")) >> n: map_res!(alphanumeric, str::from_utf8) >> (if let Some(_) = s {
            -n.parse::<i64>().unwrap()
        } else {
            n.parse::<i64>().unwrap()
        })
    )
);

named!(
    data<Vec<i64>>,
    delimited!(tag!("<"), separated_list!(char!(','), num), tag!(">"))
);

named!(
    particle<Particle>,
    do_parse!(
        tag!("p=") >> p: data >> tag!(", v=") >> s: data >> tag!(", a=") >> a: data
            >> (Particle::with((p[0], p[1], p[2]), (s[0], s[1], s[2]), (a[0], a[1], a[2])))
    )
);
#[derive(Copy, Clone, Debug, PartialEq)]
struct Particle {
    position: (i64, i64, i64),
    speed: (i64, i64, i64),
    acceleration: (i64, i64, i64),
}

impl Particle {
    fn with(
        position: (i64, i64, i64),
        speed: (i64, i64, i64),
        acceleration: (i64, i64, i64),
    ) -> Particle {
        Particle {
            position: position,
            speed: speed,
            acceleration: acceleration,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_neg_num() {
        assert_eq!(num(b"-3"), IResult::Done(&b""[..], -3i64));
    }
    #[test]
    fn test_position() {
        assert_eq!(data(b"<-1,-2,3>"), IResult::Done(&b""[..], vec![-1, -2, 3]));
    }
}
fn main() {
    let path = "input.txt";
    let mut input = File::open(path).expect("Unable to open file!");
    let mut input_txt = String::new();
    match input.read_to_string(&mut input_txt) {
        Err(_) => return,
        Ok(n) => println!("Read {} bytes", n),
    }

    let mut particles = Vec::new();
    for line in input_txt.lines() {
        particles.push(particle(line.as_bytes()).unwrap().1);
    }
    println!("{:#?}", particles);
}
