#[macro_use]
extern crate nom;
use std::fs::File;
use std::io::Read;
use std::str;
use std::collections::HashMap;
use nom::alphanumeric;

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

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
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
    let mut particles_copy = particles.clone();
    let closest_particle = get_closest_particle(&mut particles, 1000);
    println!("Particle closest to <0, 0, 0> is: {:#?}", closest_particle);

    println!(
        "The number of particles remaining are: {:?}",
        get_no_of_particles(&mut particles_copy, 1_000)
    );
}

fn get_closest_particle(particles: &mut Vec<Particle>, ticks: usize) -> usize {
    for _ in 0..ticks {
        for particle in particles.iter_mut() {
            particle.speed = (
                particle.speed.0 + particle.acceleration.0,
                particle.speed.1 + particle.acceleration.1,
                particle.speed.2 + particle.acceleration.2,
            );
            particle.position = (
                particle.position.0 + particle.speed.0,
                particle.position.1 + particle.speed.1,
                particle.position.2 + particle.speed.2,
            );
        }
    }

    particles
        .iter()
        .map(|p| p.position)
        .fold(Vec::new(), |mut acc, v| {
            acc.push(v.0.abs() + v.1.abs() + v.2.abs());
            acc
        })
        .iter()
        .enumerate()
        .min_by(|e1, e2| e1.1.cmp(e2.1))
        .unwrap()
        .0
}

fn get_no_of_particles(particles: &mut Vec<Particle>, ticks: usize) -> usize {
    for _ in 0..ticks {
        for particle in particles.iter_mut() {
            particle.speed = (
                particle.speed.0 + particle.acceleration.0,
                particle.speed.1 + particle.acceleration.1,
                particle.speed.2 + particle.acceleration.2,
            );
            particle.position = (
                particle.position.0 + particle.speed.0,
                particle.position.1 + particle.speed.1,
                particle.position.2 + particle.speed.2,
            );
        }

        // we check which particles have the same position
        let same_position = particles
            .iter()
            .map(|p| p.position)
            .fold(HashMap::new(), |mut acc, v| {
                *acc.entry(v).or_insert(0) += 1;

                acc
            })
            .into_iter()
            .filter(|&(_, v)| v > 1)
            .map(|(k, _)| k)
            .collect::<Vec<_>>();

        for p in same_position.iter() {
            let mut idx = 0;
            'infinite: loop {
                let mut found = false;
                for (idx_inner, p_inner) in particles.iter().enumerate() {
                    if p_inner.position == *p {
                        found = true;
                        idx = idx_inner;
                    }
                }

                if found {
                    particles.remove(idx);
                } else {
                    break 'infinite;
                }
            }
        }
    }

    particles.len()
}
