mod days;
use days::{day1, day2, day3, day4};

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("improper amount of args");
    }

    let day = args[1].parse::<u8>().expect("not a valid day");

    let func = get_day_solver(day);

    let (p1, p2) = func();

    println!("Solution 1: {}", p1);
    println!("Solution 2: {}", p2);
}

fn get_day_solver(day: u8) -> fn() -> (String, String) {
    match day {
        1 => day1::solve::solve,
        2 => day2::solve::solve,
        3 => day3::solve::solve,
        4 => day4::solve::solve,
        _ => unimplemented!(),
    }
}
