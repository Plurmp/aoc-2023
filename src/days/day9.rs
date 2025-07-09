use std::fs::read_to_string;

use nom::{
    IResult, Parser,
    character::complete::{self, line_ending, space1},
    multi::separated_list1,
};

use itertools::Itertools;

const _EX: &str = r"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

pub fn solve() -> (String, String) {
    // let input = _EX.to_string();
    let input = read_to_string("inputs/input9.txt").expect("could not read input");

    let (_, initial_sequences) = parse_sequences(&input).expect("epic parse fail");

    let forward_extrapolated: Vec<_> = initial_sequences
        .iter()
        .map(|initial_sequence| get_difference_sequences(initial_sequence))
        .map(|difference_sequences| {
            let mut difference_sequences = difference_sequences;
            difference_sequences.last_mut().unwrap().push(0);
            for i in (1..difference_sequences.len()).rev() {
                let a = *difference_sequences[i].last().unwrap();
                let b = *difference_sequences[i - 1].last().unwrap();
                difference_sequences[i - 1].push(a + b);
            }
            // dbg!(&difference_sequences);
            difference_sequences
        })
        .collect();
    let p1: i64 = forward_extrapolated
        .iter()
        .map(|difference_sequences| *difference_sequences[0].last().unwrap())
        .sum();

    let backward_extrapolated = forward_extrapolated
        .into_iter()
        .map(|difference_sequences| {
            let mut difference_sequences = difference_sequences;
            difference_sequences.last_mut().unwrap().insert(0, 0);
            for i in (1..difference_sequences.len()).rev() {
                let a = *difference_sequences[i].first().unwrap();
                let b = *difference_sequences[i - 1].first().unwrap();
                difference_sequences[i - 1].insert(0, b - a);
            }
            difference_sequences
        });
    let p2: i64 = backward_extrapolated
        .map(|difference_sequences| *difference_sequences[0].first().unwrap())
        .sum();

    (p1.to_string(), p2.to_string())
}

fn parse_sequences(input: &str) -> IResult<&str, Vec<Vec<i64>>> {
    separated_list1(line_ending, separated_list1(space1, complete::i64)).parse(input)
}

fn get_difference_sequences(initial_sequence: &[i64]) -> Vec<Vec<i64>> {
    let mut difference_sequences = vec![initial_sequence.to_vec()];
    while let Some(last_sequence) = difference_sequences.last() {
        if last_sequence.iter().all(|n| *n == 0) {
            break;
        }
        let new_sequence: Vec<_> = last_sequence
            .iter()
            .tuple_windows()
            .map(|(a, b)| b - a)
            .collect();
        difference_sequences.push(new_sequence);
    }
    difference_sequences
}
