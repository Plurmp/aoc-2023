use std::{fs::read_to_string, ops::Add};

use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete,
    multi::{separated_list0, separated_list1},
};

#[derive(Debug)]
struct Game {
    game_number: u32,
    cube_sets: Vec<CubeSet>,
}

#[derive(Debug, Default, Clone, Copy)]
struct CubeSet {
    red: u32,
    green: u32,
    blue: u32,
}

impl Add<CubeSet> for CubeSet {
    type Output = CubeSet;

    fn add(self, rhs: Self) -> Self::Output {
        CubeSet {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue,
        }
    }
}

#[derive(Debug)]
enum Color {
    Red,
    Blue,
    Green,
}

pub fn solve() -> (String, String) {
    let input = read_to_string("inputs/input2.txt").expect("input could not be read");

    let (_, games) = parse_games(&input).expect("could not parse");
    let p1: u32 = games
        .iter()
        .filter(|game| {
            game.cube_sets
                .iter()
                .all(|CubeSet { red, green, blue }| *red <= 12 && *green <= 13 && *blue <= 14)
        })
        .map(|game| game.game_number)
        .sum();

    let p2: u32 = games
        .iter()
        .map(|game| {
            game.cube_sets
                .iter()
                .fold(CubeSet::default(), |acc, n| CubeSet {
                    red: acc.red.max(n.red),
                    green: acc.green.max(n.green),
                    blue: acc.blue.max(n.blue),
                })
        })
        .map(|CubeSet { red, green, blue }| red * green * blue)
        .sum();

    (p1.to_string(), p2.to_string())
}

fn parse_games(input: &str) -> IResult<&str, Vec<Game>> {
    separated_list1(complete::line_ending, parse_game).parse(input)
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, _) = tag("Game ")(input)?;
    let (input, game_number) = complete::u32(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, cube_sets) = parse_cube_sets(input)?;

    Ok((
        input,
        Game {
            game_number,
            cube_sets,
        },
    ))
}

fn parse_cube_sets(input: &str) -> IResult<&str, Vec<CubeSet>> {
    separated_list0(tag("; "), parse_cube_set).parse(input)
}

fn parse_cube_set(input: &str) -> IResult<&str, CubeSet> {
    let (input, cube_counts) = separated_list0(tag(", "), parse_cube_count).parse(input)?;

    let mut red = 0u32;
    let mut green = 0u32;
    let mut blue = 0u32;

    for (color, count) in cube_counts {
        match color {
            Color::Red => red = count,
            Color::Green => green = count,
            Color::Blue => blue = count,
        }
    }

    Ok((input, CubeSet { red, green, blue }))
}

fn parse_cube_count(input: &str) -> IResult<&str, (Color, u32)> {
    let (input, count) = complete::u32(input)?;
    let (input, _) = complete::space1(input)?;
    let (input, color_str) = alt((tag("red"), tag("green"), tag("blue"))).parse(input)?;
    let color = match color_str {
        "red" => Color::Red,
        "green" => Color::Green,
        "blue" => Color::Blue,
        _ => unreachable!(),
    };

    Ok((input, (color, count)))
}
