use std::io::BufRead;
use std::{io, iter};
use itertools::Itertools;
use regex::Regex;
use crate::solutions::day9::ring::Ring;

mod ring;

fn read_input(input: impl BufRead) -> (usize, u32) {
    let (line,) = input.lines().map(Result::unwrap).collect_tuple().unwrap();

    let regex = Regex::new(r"^(\d+) players; last marble is worth (\d+) points$").unwrap();
    let caps = regex.captures(&line).unwrap();

    (caps[1].parse().unwrap(), caps[2].parse().unwrap())
}

pub fn main() {
    let (num_players, last_marble) = read_input(io::stdin().lock());

    println!("{}", solve(num_players, last_marble));
    println!("{}", solve(num_players, last_marble * 100));
}

fn solve(num_players: usize, last_marble: u32) -> u32 {
    let mut scores = vec![0; num_players];

    // Place the initial marble.
    let mut ring = Ring::singleton(0);
    let mut current = 0;

    let marbles = 1..=last_marble;
    let turns = iter::repeat(0..num_players).flatten();

    for (marble, player) in marbles.zip(turns) {
        if marble % 23 == 0 {
            scores[player] += marble;

            let bonus = ring.before(current, 7);
            current = ring.after(bonus, 1);
            ring.remove(bonus);

            scores[player] += bonus;
        } else {
            ring.insert_after(ring.after(current, 1), marble);
            current = marble;
        }
    }

    scores.into_iter().max().unwrap()
}
