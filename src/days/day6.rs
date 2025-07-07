use std::fs::read_to_string;

use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{self, line_ending, space1},
    multi::separated_list1,
};

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

const _EX: &str = r"Time:      7  15   30
Distance:  9  40  200";

pub fn solve() -> (String, String) {
    // let input = _EX.to_string();
    let input = read_to_string("inputs/input6.txt").expect("could not read file");

    let (_, races) = parse_races(&input).expect("epic parse fail");

    let p1: u32 = races
        .iter()
        .map(|Race { time, distance }| {
            let mut count = 0u32;
            for i in 1..*time {
                if i * (time - i) > *distance {
                    count += 1;
                }
            }
            count
        })
        .product();

    let (time_p2, distance_p2) = races
        .iter()
        .fold(("".to_string(), "".to_string()), |acc, n| (acc.0 + &n.time.to_string(), acc.1 + &n.distance.to_string()));

    let time_p2: u64 = time_p2.parse().unwrap();
    let distance_p2: u64 = distance_p2.parse().unwrap();

    let first_win = (0..time_p2)
        .find(|i| i * (time_p2 - i) > distance_p2)
        .unwrap();
    let p2 = time_p2 - (first_win * 2) + 1; 

    (p1.to_string(), p2.to_string())
}

fn parse_races(input: &str) -> IResult<&str, Vec<Race>> {
    let (input, _) = (tag("Time:"), space1).parse(input)?;
    let (input, times) = separated_list1(space1, complete::u64).parse(input)?;
    let (input, _) = (line_ending, tag("Distance:"), space1).parse(input)?;
    let (input, distances) = separated_list1(space1, complete::u64).parse(input)?;

    Ok((
        input,
        times
            .into_iter()
            .zip(distances.into_iter())
            .map(|(time, distance)| Race { time, distance })
            .collect(),
    ))
}
