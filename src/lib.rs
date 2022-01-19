mod solutions;

pub fn solve(day: u32) {
    assert!(1 <= day && day <= 25);

    use solutions::*;
    match day {
        8 => day8::main(),
        _ => panic!("Not yet implemented: day {day}"),
    }
}
