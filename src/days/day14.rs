use std::{collections::BTreeSet, fs::read_to_string, hash::Hash};

use cached::UnboundCache;
use cached::proc_macro::cached;
use glam::{IVec2, ivec2};
use hashable::HashableHashSet;
use itertools::Itertools;

#[derive(PartialEq, Eq, Clone, Copy)]
enum Rock {
    Round,
    Cube,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn to_ivec2(self) -> IVec2 {
        match self {
            Direction::North => ivec2(0, -1),
            Direction::South => ivec2(0, 1),
            Direction::East => ivec2(1, 0),
            Direction::West => ivec2(-1, 0),
        }
    }
}

const _EX: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

pub fn solve() -> (String, String) {
    // let input = _EX.to_string();
    let input = read_to_string("inputs/input14.txt").expect("input could not be read");

    let rock_positions = input.lines().enumerate().flat_map(|(y, line)| {
        line.char_indices().flat_map(move |(x, ch)| {
            let pos = ivec2(x as i32, y as i32);

            match ch {
                'O' => Some((pos, Rock::Round)),
                '#' => Some((pos, Rock::Cube)),
                _ => None,
            }
        })
    });
    let round_positions: Vec<_> = rock_positions
        .clone()
        .filter(|(_, rock)| rock == &Rock::Round)
        .map(|(pos, _)| pos)
        .collect();
    let cube_positions: Vec<_> = rock_positions
        .filter(|(_, rock)| rock == &Rock::Cube)
        .map(|(pos, _)| pos)
        .collect();

    let width = input.lines().next().unwrap().len() as i32;
    let height = input.lines().count() as i32;

    let mut round_positions_p1 = round_positions.clone();
    round_positions_p1 = move_rocks(
        round_positions_p1,
        &cube_positions,
        Direction::North,
        width,
        height,
    );

    let p1: i32 = round_positions_p1.iter().map(|pos| height - pos.y).sum();

    let mut round_positions_p2 = round_positions.clone();
    let mut previous_cycles: Vec<BTreeSet<(i32, i32)>> = vec![
        round_positions_p2
            .iter()
            .map(|pos| std::convert::Into::<(i32, i32)>::into(*pos))
            .collect::<BTreeSet<_>>(),
    ];
    let mut loop_length = 0;
    let mut loop_start = 0;
    for i in 0.. {
        round_positions_p2 = cycle(round_positions_p2, &cube_positions, width, height);
        println!("{round_positions_p2:?}");
        let this_cycle = round_positions_p2
            .iter()
            .map(|pos| <(i32, i32)>::from(*pos))
            .collect::<BTreeSet<_>>();
        if let Some((prev_idx, _)) = previous_cycles
            .iter()
            .find_position(|cycle| **cycle == this_cycle)
        {
            loop_length = i - prev_idx + 1;
            loop_start = prev_idx;
            break;
        } else {
            previous_cycles.push(this_cycle);
        }
    }
    // let previous_cycles = &previous_cycles[loop_start..];

    dbg!(loop_start);
    dbg!(loop_length);

    let j = ((1_000_000_000 - loop_start) % loop_length) + loop_start;
    dbg!(j);

    for set in &previous_cycles {
        println!("{}", set.iter().map(|(_, y)| height - y).sum::<i32>());
    }

    let p2: i32 = previous_cycles[j].iter().map(|(_, y)| height - y).sum();

    (p1.to_string(), p2.to_string())
}

#[cached(
    ty = "UnboundCache<HashableHashSet<IVec2>, Vec<IVec2>>",
    create = "{ UnboundCache::new() }",
    convert = "{
        round_positions
            .iter()
            .cloned()
            .collect::<HashableHashSet<_>>()
    }"
)]
fn cycle(
    round_positions: Vec<IVec2>,
    cube_positions: &[IVec2],
    width: i32,
    height: i32,
) -> Vec<IVec2> {
    let directions = [
        Direction::North,
        Direction::West,
        Direction::South,
        Direction::East,
    ];

    let mut round_positions = round_positions;
    for direction in directions {
        round_positions = move_rocks(round_positions, cube_positions, direction, width, height);
    }

    round_positions
}

#[cached(
    ty = "UnboundCache<(HashableHashSet<IVec2>, Direction), Vec<IVec2>>",
    create = "{ UnboundCache::new() }",
    convert = "{(
        round_positions
            .iter()
            .cloned()
            .collect::<HashableHashSet<_>>(), 
        direction
    )}"
)]
fn move_rocks(
    round_positions: Vec<IVec2>,
    cube_positions: &[IVec2],
    direction: Direction,
    width: i32,
    height: i32,
) -> Vec<IVec2> {
    let mut round_positions = round_positions;
    match direction {
        Direction::North => round_positions.sort_by(|a, b| a.y.cmp(&b.y)),
        Direction::East => round_positions.sort_by(|a, b| b.x.cmp(&a.x)),
        Direction::South => round_positions.sort_by(|a, b| b.y.cmp(&a.y)),
        Direction::West => round_positions.sort_by(|a, b| a.x.cmp(&b.x)),
    }
    let direction_ivec2 = direction.to_ivec2();

    for i in 0..round_positions.len() {
        let mut above_pos = round_positions[i] + direction_ivec2;
        while !cube_positions.contains(&above_pos)
            && !round_positions.contains(&above_pos)
            && (0..height).contains(&above_pos.y)
            && (0..width).contains(&above_pos.x)
        {
            round_positions[i] += direction_ivec2;
            above_pos += direction_ivec2;
        }
    }

    round_positions
}
