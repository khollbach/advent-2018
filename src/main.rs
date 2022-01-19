use std::env;
use std::process;

fn main() {
    let mut args = env::args();

    let prog_name = args.next().unwrap();
    let args: Vec<_> = args.collect();

    let usage = || -> ! {
        eprintln!();
        eprintln!("Usage: `{prog_name} <num>`");
        eprintln!("where <num> is a number from 1 through 25.");
        process::exit(1);
    };

    if args.len() != 1 {
        eprintln!("Expected 1 argument, got {}.", args.len());
        usage();
    }

    let day: u32 = match args[0].parse() {
        Ok(n) if 1 <= n && n <= 25 => {
            n
        }
        _ => {
            eprintln!("Expected a number from 1 through 25, got: {}", args[0]);
            usage();
        }
    };

    advent_2018::solve(day);
}
