use regex::Regex;
use std::{collections::HashMap, fs::read_to_string};

const _EX: &str = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

pub fn solve() -> (String, String) {
    // let input = EX.to_string();
    let input = read_to_string("inputs/input3.txt")
        .expect("input not readable")
        .replace("\r\n", "\n");
    let line_len = input.lines().next().unwrap().len();
    let input = ".".repeat(line_len) + "\n" + &input + &".".repeat(line_len);
    let input: String = input
        .lines()
        .map(|line| ".".to_string() + line + ".\n")
        .collect();
    let line_len = input.lines().next().unwrap().len() + 1;

    let re_num = Regex::new(r"\d+").unwrap();
    let number_matches: Vec<_> = re_num
        .captures_iter(&input)
        .map(|cap| cap.get(0).unwrap())
        .collect();
    let mut p1 = 0u32;
    for num in &number_matches {
        let a = num.start();
        let b = num.end();
        let check_string = input[(a - 1 - line_len)..(b + 1 - line_len)].to_string()
            + &input[(a - 1)..(b + 1)]
            + &input[(a - 1 + line_len)..(b + 1 + line_len)];
        if check_string.chars().any(is_special_character) {
            p1 += num.as_str().parse::<u32>().unwrap();
        }
    }

    let mut star_matches: HashMap<usize, Vec<u32>> = HashMap::new();
    for num in &number_matches {
        let a = num.start();
        let b = num.end();

        for i in (a - 1 - line_len)..(b + 1 - line_len) {
            if input.chars().nth(i) == Some('*') {
                let parsed_num = num.as_str().parse().unwrap();
                star_matches
                    .entry(i)
                    .and_modify(|v| {
                        v.push(parsed_num);
                    })
                    .or_insert(vec![parsed_num]);
            }
        }
        for i in (a - 1)..(b + 1) {
            if input.chars().nth(i) == Some('*') {
                let parsed_num = num.as_str().parse().unwrap();
                star_matches
                    .entry(i)
                    .and_modify(|v| {
                        v.push(parsed_num);
                    })
                    .or_insert(vec![parsed_num]);
            }
        }
        for i in (a - 1 + line_len)..(b + 1 + line_len) {
            if input.chars().nth(i) == Some('*') {
                let parsed_num = num.as_str().parse().unwrap();
                star_matches
                    .entry(i)
                    .and_modify(|v| {
                        v.push(parsed_num);
                    })
                    .or_insert(vec![parsed_num]);
            }
        }
    }
    let p2: u32 = star_matches
        .iter()
        .filter(|(_, nums)| nums.len() == 2)
        .map(|(_, nums)| nums.iter().product::<u32>())
        .sum();

    (p1.to_string(), p2.to_string())
}

fn is_special_character(ch: char) -> bool {
    !ch.is_alphanumeric() && ch != '.' && !ch.is_whitespace()
}
