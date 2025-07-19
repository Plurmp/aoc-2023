use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

use glam::{IVec2, UVec2, ivec2, uvec2};
use itertools::Itertools;
use nom::{
    IResult, Parser,
    bytes::complete::{tag, take_while_m_n},
    character::complete::{self, line_ending, one_of, space1},
    combinator::map_res,
    multi::separated_list1,
};

#[derive(Debug, Clone, Copy)]
struct Instruction {
    dir: D,
    meters: u8,
    color: Color,
}

#[derive(Debug, Clone, Copy)]
enum D {
    U,
    D,
    L,
    R,
}

#[derive(Debug, Clone, Copy)]
struct Color(u8, u8, u8);

impl D {
    fn to_ivec2(self) -> IVec2 {
        match self {
            D::U => ivec2(0, 1),
            D::D => ivec2(0, -1),
            D::L => ivec2(-1, 0),
            D::R => ivec2(1, 0),
        }
    }
}

const _EX: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

pub fn solve() -> (String, String) {
    let input = _EX.to_string();
    // let input = read_to_string("inputs/input18.txt").expect("could not read input");

    let (_, instructions) = parse_instructions(&input).expect("epic parse fail");

    let trench = dig_trench(&instructions);
    println!("{}", display_field(&trench));
    let p1 = trench.len();

    (p1.to_string(), "".to_string())
}

fn dig_trench(instructions: &[Instruction]) -> HashSet<UVec2> {
    let mut trench = HashSet::new();
    let mut digger = ivec2(0, 0);
    for instruction in instructions {
        for _ in 0..instruction.meters {
            digger += instruction.dir.to_ivec2();
            trench.insert(digger);
        }
    }
    let mut trench = normalize_set(&trench);
    // dbg!(&trench);

    // let mut grid = set_to_grid(&trench);
    // grid = grid
    //     .into_iter()
    //     .map(|row| {
    //         row.into_iter()
    //             .chunk_by(|elem| *elem)
    //             .into_iter()
    //             .flat_map(|(key, chunk)| {
    //                 let chunk: Vec<_> = chunk.collect();
    //                 if key && chunk.len() >= 3 {
    //                     let mut v = vec![false; chunk.len() - 2];
    //                     v.insert(0, true);
    //                     v.push(true);

    //                     v
    //                 } else {
    //                     chunk
    //                 }
    //             })
    //             .collect()
    //     })
    //     .collect();
    // trench = grid_to_set(&grid);

    let topmost = trench.iter().map(|pos| pos.y).max().unwrap();
    let rightmost = trench.iter().map(|pos| pos.x).max().unwrap();

    println!("{}\n", display_field(&trench));

    for y in 0..=topmost {
        let mut fill = false;
        let mut last_was_trench = false;
        for x in 0..=rightmost {
            let pos = uvec2(x, y);
            if trench.contains(&pos) {
                last_was_trench = true;
                continue;
            }
            if last_was_trench {
                fill = !fill;
                if fill {
                    trench.insert(pos);
                }
            }
            last_was_trench = false;
        }
    }

    trench
}

fn display_field(trench: &HashSet<UVec2>) -> String {
    let topmost = trench.iter().map(|pos| pos.y).max().unwrap();
    let rightmost = trench.iter().map(|pos| pos.x).max().unwrap();

    // dbg!(topmost, rightmost);

    vec![".".repeat((0..=rightmost).count()); (0..=topmost).count()]
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.char_indices()
                .map(|(x, _)| {
                    if trench.contains(&uvec2(x as u32, y as u32)) {
                        "#"
                    } else {
                        "."
                    }
                })
                .collect::<String>()
        })
        .join("\n")
}

fn parse_instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list1(line_ending, parse_instruction).parse(input)
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, (dir, _, meters, _, color)) =
        (parse_dir, space1, complete::u8, space1, parse_color).parse(input)?;

    let color = Color(color.0, color.1, color.2);

    Ok((input, Instruction { dir, meters, color }))
}

fn parse_dir(input: &str) -> IResult<&str, D> {
    let (input, raw_dir) = one_of("UDLR")(input)?;
    let dir = match raw_dir {
        'U' => D::U,
        'D' => D::D,
        'L' => D::L,
        'R' => D::R,
        _ => unreachable!(),
    };

    Ok((input, dir))
}

fn parse_color(input: &str) -> IResult<&str, (u8, u8, u8)> {
    let (input, _) = tag("(#")(input)?;
    let (input, (r, g, b)) = (get_hex_color, get_hex_color, get_hex_color).parse(input)?;
    let (input, _) = tag(")")(input)?;

    Ok((input, (r, g, b)))
}

fn get_hex_color(input: &str) -> IResult<&str, u8> {
    map_res(take_while_m_n(2, 2, |ch: char| ch.is_digit(16)), |s| {
        u8::from_str_radix(s, 16)
    })
    .parse(input)
}

fn normalize_set(set: &HashSet<IVec2>) -> HashSet<UVec2> {
    if set.is_empty() {
        return HashSet::new();
    }

    let leftmost = set.iter().map(|pos| pos.x).min().unwrap();
    let bottommost = set.iter().map(|pos| pos.y).min().unwrap();

    set.iter()
        .map(|pos| (pos - ivec2(leftmost, bottommost)).as_uvec2())
        .collect()
}

fn set_to_grid(set: &HashSet<UVec2>) -> Vec<Vec<bool>> {
    if set.is_empty() {
        return vec![];
    }

    let width = set.iter().map(|pos| pos.x).max().unwrap() + 1;
    let height = set.iter().map(|pos| pos.y).max().unwrap() + 1;

    (0..height)
        .map(|y| {
            (0..width)
                .map(move |x| set.contains(&uvec2(x, y)))
                .collect()
        })
        .collect()
}

fn grid_to_set(grid: &Vec<Vec<bool>>) -> HashSet<UVec2> {
    if grid.is_empty() {
        return HashSet::new();
    }

    grid.iter()
        .enumerate()
        .flat_map(|(x, col)| {
            col.iter()
                .enumerate()
                .flat_map(move |(y, b)| b.then_some(uvec2(x as u32, y as u32)))
        })
        .collect()
}
