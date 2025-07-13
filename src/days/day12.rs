use std::{collections::HashMap, fs::read_to_string};

use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, line_ending, space1},
    multi::{many1, separated_list1},
    sequence::separated_pair,
};

#[derive(Debug)]
struct Row {
    springs: Vec<Condition>,
    groups: Vec<usize>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Condition {
    Operational,
    Damaged,
    Unknown,
}

const _EX: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

pub fn solve() -> (String, String) {
    // let input = _EX.to_string();
    let input = read_to_string("inputs/input12.txt").expect("input could not be read");

    let (_, rows) = parse_rows(&input).expect("epic parse fail");

    let p1: usize = rows
        .iter()
        .map(|row| solve_row(&row.springs, &row.groups, &mut HashMap::new()))
        .sum();

    let rows_p2 = rows.iter().map(|row| {
        let springs = vec![row.springs.clone(); 5].join(&Condition::Unknown);
        let groups = row.groups.repeat(5);

        Row { springs, groups }
    });

    let p2: usize = rows_p2
        .map(|row| solve_row(&row.springs, &row.groups, &mut HashMap::new()))
        .sum();

    (p1.to_string(), p2.to_string())
}

fn solve_row<'a>(
    springs: &'a [Condition],
    groups: &'a [usize],
    dp: &mut HashMap<(&'a [Condition], &'a [usize]), usize>,
) -> usize {
    if groups.is_empty() {
        if !springs.contains(&Condition::Damaged) {
            return 1;
        } else {
            return 0;
        }
    }

    if let Some(r) = dp.get(&(springs, groups)) {
        return *r;
    }

    let min_remaining_len = groups.iter().sum::<usize>() + groups.len() - 1;
    if springs.len() < min_remaining_len {
        return 0;
    }

    if springs[0] == Condition::Operational {
        return solve_row(&springs[1..], groups, dp);
    }

    let current_group = groups[0];
    let all_springs_valid = springs[0..current_group]
        .iter()
        .all(|spring| *spring != Condition::Operational);
    let last_spring_valid = springs.len() == current_group
        || springs[current_group..(current_group + 1)]
            .iter()
            .all(|spring| *spring != Condition::Damaged);
    let mut total = 0usize;
    if all_springs_valid && last_spring_valid {
        let max_idx = springs.len().min(current_group + 1);
        total += solve_row(&springs[max_idx..], &groups[1..], dp);
    }

    if springs[0] != Condition::Damaged {
        total += solve_row(&springs[1..], groups, dp);
    }

    dp.insert((springs, groups), total);
    total
}

fn parse_rows(input: &str) -> IResult<&str, Vec<Row>> {
    separated_list1(line_ending, parse_row).parse(input)
}

fn parse_row(input: &str) -> IResult<&str, Row> {
    let (input, (springs, groups)) =
        separated_pair(parse_springs, space1, parse_groups).parse(input)?;

    Ok((input, Row { springs, groups }))
}

fn parse_springs(input: &str) -> IResult<&str, Vec<Condition>> {
    let (input, springs) = many1(alt((tag("#"), tag("."), tag("?")))).parse(input)?;
    let springs = springs
        .iter()
        .map(|&spring| match spring {
            "." => Condition::Operational,
            "#" => Condition::Damaged,
            "?" => Condition::Unknown,
            _ => unreachable!(),
        })
        .collect();

    Ok((input, springs))
}

fn parse_groups(input: &str) -> IResult<&str, Vec<usize>> {
    separated_list1(tag(","), complete::usize).parse(input)
}
