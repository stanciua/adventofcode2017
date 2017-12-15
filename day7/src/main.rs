// pbga (66)
// xhth (57)
// ebii (61)
// havc (66)
// ktlj (57)
// fwft (72) -> ktlj, cntj, xhth
// qoyq (66)
// padx (45) -> pbga, havc, qoyq
// tknk (41) -> ugml, padx, fwft
// jptl (61)
// ugml (68) -> gyxo, ebii, jptl
// gyxo (61)
// cntj (57)
#[macro_use]
extern crate nom;

use std::str;
use nom::alphanumeric;

named!(name<&str>, map_res!(alphanumeric, str::from_utf8));
named!(
    weight<i32>,
    map_res!(
        map_res!(
            delimited!(char!('('), is_not!(")"), char!(')')),
            str::from_utf8
        ),
        str::parse::<i32>
    )
);

named!(
    children<Vec<&str>>,
    many0!(map_res!(delimted!(char!(','), char!(' ')), str::from_utf8))
);
#[cfg(test)]
mod test {
    use super::*;
    use nom::IResult::*;
    #[test]
    fn test_name_macro() {
        assert_eq!(name(b"pbga (66)"), Done(&b" (66)"[..], "pbga"));
    }

    #[test]
    fn test_name_weight() {
        assert_eq!(weight(b"(66)"), Done(&b""[..], 66));
    }
}
fn main() {
    println!("Hello, world!");
}
