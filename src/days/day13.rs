use std::fs::read_to_string;

use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::line_ending,
    multi::{many1, separated_list1},
    sequence::pair,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Terrain {
    Ash,
    Rock,
}

type Grid = Vec<Vec<Terrain>>;

const _EX: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

pub fn solve() -> (String, String) {
    // let input = _EX.to_string();
    let input = read_to_string("inputs/input13.txt").expect("could not read input");

    let (_, grids) = parse_grids(&input).expect("epic parse fail");
    // dbg!(&grids);

    // dbg!(check_horizontal_reflection(&grids[0]));

    let p1: usize = grids
        .iter()
        .map(|grid| {
            let vertical_total: usize = check_vertical_reflection(grid).iter().sum();
            let horizontal_total: usize = check_horizontal_reflection(grid).iter().sum();

            vertical_total + (horizontal_total * 100)
        })
        .sum();

    let p2: usize = grids
        .iter()
        .map(|grid| {
            let vertical_total: usize = check_vertical_smudge(grid).iter().sum();
            let horizontal_total: usize = check_horizontal_smudge(grid).iter().sum();

            vertical_total + (horizontal_total * 100)
        })
        .sum();

    (p1.to_string(), p2.to_string())
}

fn check_horizontal_reflection(grid: &Grid) -> Vec<usize> {
    let mut reflection_lines = vec![];

    for i in 1..grid.len() {
        if (0..i).rev().zip(i..grid.len()).all(|(a, b)| {
            // dbg!(&grid[a], &grid[b]);
            grid[a] == grid[b]
        }) {
            reflection_lines.push(i);
        }
    }

    reflection_lines
}

fn check_vertical_reflection(grid: &Grid) -> Vec<usize> {
    let new_grid: Vec<Vec<Terrain>> = (0..grid[0].len())
        .map(|i| grid.iter().map(|inner| inner[i]).collect::<Vec<Terrain>>())
        .collect();

    check_horizontal_reflection(&new_grid)
}

fn check_horizontal_smudge(grid: &Grid) -> Vec<usize> {
    let mut reflection_lines = vec![];

    'outer: for i in 1..grid.len() {
        let mut smudges = 1i32;
        smudges -= grid[i]
            .iter()
            .zip(grid[i - 1].iter())
            .filter(|(a, b)| a != b)
            .count() as i32;
        if smudges < 0 {
            continue;
        }
        for j in 1..=i {
            if i + j >= grid.len() || (i as isize - j as isize - 1) < 0 {
                break;
            }
            smudges -= grid[i - j - 1]
                .iter()
                .zip(grid[i + j].iter())
                .filter(|(a, b)| a != b)
                .count() as i32;
            if smudges < 0 {
                continue 'outer;
            }
        }
        if smudges == 0 {
            reflection_lines.push(i);
        }
    }

    reflection_lines
}

fn check_vertical_smudge(grid: &Grid) -> Vec<usize> {
    let new_grid: Vec<Vec<Terrain>> = (0..grid[0].len())
        .map(|i| grid.iter().map(|inner| inner[i]).collect::<Vec<Terrain>>())
        .collect();

    check_horizontal_smudge(&new_grid)
}

fn parse_grids(input: &str) -> IResult<&str, Vec<Grid>> {
    separated_list1(pair(line_ending, line_ending), parse_grid).parse(input)
}

fn parse_grid(input: &str) -> IResult<&str, Grid> {
    separated_list1(line_ending, parse_row).parse(input)
}

fn parse_row(input: &str) -> IResult<&str, Vec<Terrain>> {
    let (input, raw_terrains) = many1(alt((tag("#"), tag(".")))).parse(input)?;
    let terrains = raw_terrains
        .iter()
        .map(|terrain| match *terrain {
            "." => Terrain::Ash,
            "#" => Terrain::Rock,
            _ => unreachable!(),
        })
        .collect();

    Ok((input, terrains))
}
