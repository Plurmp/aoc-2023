use std::collections::HashMap;
use std::fs::read_to_string;

pub fn solve() -> (String, String) {
    let input = read_to_string("./src/day1/input.txt").expect("input not readable");

    let p1: u32 = input
        .lines()
        .map(|line| {
            let digits: Vec<_> = line.chars().filter(|ch| ch.is_ascii_digit()).collect();
            let calibration_value =
                digits.first().unwrap().to_string() + &digits.last().unwrap().to_string();

            calibration_value.parse::<u32>().unwrap()
        })
        .sum();

    let digit_words = HashMap::from([
        ("zero", "0"),
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);

    let p2: u32 = input
        .lines()
        .map(|line| {
            let mut digits: Vec<String> = vec![];
            for (i, ch) in line.chars().enumerate() {
                if ch.is_ascii_digit() {
                    digits.push(ch.to_string());
                } else {
                    for k in digit_words.keys() {
                        if line[i..].starts_with(k) {
                            digits.push(digit_words.get(k).unwrap().to_string());
                        }
                    }
                }
            }
            let calibration_value =
                digits.first().unwrap().to_string() + &digits.last().unwrap().to_string();

            calibration_value.parse::<u32>().unwrap()
        })
        .sum();

    (p1.to_string(), p2.to_string())
}
