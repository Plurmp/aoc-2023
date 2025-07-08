mod days;
use days::{day1, day2, day3, day4, day5, day6, day7, day8};

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
        1 => day1::solve,
        2 => day2::solve,
        3 => day3::solve,
        4 => day4::solve,
        5 => day5::solve,
        6 => day6::solve,
        7 => day7::solve,
        8 => day8::solve,
        _ => unimplemented!(),
    }
}
