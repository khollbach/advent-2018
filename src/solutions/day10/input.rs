use std::io::BufRead;
use regex::Regex;
use crate::solutions::day10::{Point, Sky, Star};

/// E.g.:
/// position=<-39892,  -9859> velocity=< 4,  1>
pub fn read_input(input: impl BufRead) -> Sky {
    let n = r" *(-?\d+)"; // Note the optional leading whitespace.
    let regex = Regex::new(&format!(r"^position=<{n}, {n}> velocity=<{n}, {n}>$")).unwrap();

    let stars = input.lines().map(|line| {
        let caps = regex.captures(line.as_ref().unwrap()).unwrap();

        let x = caps[1].parse().unwrap();
        let y = caps[2].parse().unwrap();
        let pos = Point { x, y };

        let x = caps[3].parse().unwrap();
        let y = caps[4].parse().unwrap();
        let vel = Point { x, y };

        Star { pos, vel }
    }).collect();

    Sky { stars }
}
