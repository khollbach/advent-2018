use std::collections::HashSet;
use std::io;
use crate::solutions::day10::input::read_input;

mod input;
mod point;

pub struct Sky {
    stars: Vec<Star>,
}

#[derive(Copy, Clone)]
struct Star {
    pos: Point,
    vel: Point,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

pub fn main() {
    let sky = read_input(io::stdin().lock());

    part_1(sky);
}

fn part_1(mut sky: Sky) {
    // Find a local minimum.
    let best = loop {
        let next = sky.next();
        if next.entropy() > sky.entropy() {
            break sky;
        }
        sky = next;
    };

    best.print();
}

impl Sky {
    fn next(&self) -> Sky {
        let stars = self.stars.iter().map(|&(mut star)| {
            star.pos += star.vel;
            star
        }).collect();

        Sky { stars }
    }

    /// We're hoping that this quantity is minimized when the message is displayed.
    fn entropy(&self) -> f64 {
        let n = self.stars.len() as f64;
        let avg_x = self.stars.iter().map(|s| s.pos.x).sum::<i32>() as f64 / n;
        let avg_y = self.stars.iter().map(|s| s.pos.y).sum::<i32>() as f64 / n;

        let x_entropy: f64 = self.stars.iter().map(|s| (avg_x - s.pos.x as f64).powf(2.)).sum();
        let y_entropy: f64 = self.stars.iter().map(|s| (avg_y - s.pos.y as f64).powf(2.)).sum();
        x_entropy + y_entropy
    }

    fn print(&self) {
        let stars: HashSet<_> = self.stars.iter().map(|s| s.pos).collect();

        let x_min = stars.iter().map(|s| s.x).min().unwrap();
        let x_max = stars.iter().map(|s| s.x).max().unwrap();

        let y_min = stars.iter().map(|s| s.y).min().unwrap();
        let y_max = stars.iter().map(|s| s.y).max().unwrap();

        for y in y_min..=y_max {
            for x in x_min..=x_max {
                if stars.contains(&Point { x, y }) {
                    print!("#");
                } else {
                    print!(" ");
                }
            }
            println!();
        }
    }
}
