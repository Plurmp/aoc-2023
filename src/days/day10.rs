use std::{collections::HashMap, fs::read_to_string};

use glam::{IVec2, ivec2};
use itertools::Itertools;
use pathfinding::prelude::dijkstra_reach;

type PipeDiagram = HashMap<IVec2, PipeType>;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum PipeType {
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
}

impl PipeType {
    fn to_directions(self) -> Vec<IVec2> {
        match self {
            PipeType::NorthSouth => vec![ivec2(0, -1), ivec2(0, 1)],
            PipeType::EastWest => vec![ivec2(-1, 0), ivec2(1, 0)],
            PipeType::NorthEast => vec![ivec2(0, -1), ivec2(1, 0)],
            PipeType::NorthWest => vec![ivec2(0, -1), ivec2(-1, 0)],
            PipeType::SouthWest => vec![ivec2(0, 1), ivec2(-1, 0)],
            PipeType::SouthEast => vec![ivec2(0, 1), ivec2(1, 0)],
        }
    }
}

const _EX: &str = r"..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

pub fn solve() -> (String, String) {
    // let input = _EX.to_string();
    let input = read_to_string("inputs/input10.txt").expect("could not read input");

    let mut starting_point: IVec2 = ivec2(0, 0);
    let mut pipe_diagram: PipeDiagram = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == 'S' {
                starting_point = ivec2(x as i32, y as i32);
                continue;
            }
            let pipe_type = match ch {
                '|' => PipeType::NorthSouth,
                '-' => PipeType::EastWest,
                'L' => PipeType::NorthEast,
                'J' => PipeType::NorthWest,
                '7' => PipeType::SouthWest,
                'F' => PipeType::SouthEast,
                _ => continue,
            };
            pipe_diagram.insert(ivec2(x as i32, y as i32), pipe_type);
        }
    }
    // order: north, east, south, west
    let adjacent_pipes = (
        pipe_diagram
            .get(&(starting_point + ivec2(0, -1)))
            .map(|pipe_type| {
                [
                    PipeType::NorthSouth,
                    PipeType::SouthEast,
                    PipeType::SouthWest,
                ]
                .contains(pipe_type)
            })
            .unwrap_or(false),
        pipe_diagram
            .get(&(starting_point + ivec2(1, 0)))
            .map(|pipe_type| {
                [PipeType::EastWest, PipeType::NorthWest, PipeType::SouthWest].contains(pipe_type)
            })
            .unwrap_or(false),
        pipe_diagram
            .get(&(starting_point + ivec2(0, 1)))
            .map(|pipe_type| {
                [
                    PipeType::NorthEast,
                    PipeType::NorthSouth,
                    PipeType::NorthWest,
                ]
                .contains(pipe_type)
            })
            .unwrap_or(false),
        pipe_diagram
            .get(&(starting_point + ivec2(-1, 0)))
            .map(|pipe_type| {
                [PipeType::EastWest, PipeType::SouthEast, PipeType::NorthEast].contains(pipe_type)
            })
            .unwrap_or(false),
    );
    dbg!(&adjacent_pipes);
    let starting_pipe = match adjacent_pipes {
        (true, false, true, false) => PipeType::NorthSouth,
        (true, true, false, false) => PipeType::NorthEast,
        (true, false, false, true) => PipeType::NorthWest,
        (false, true, true, false) => PipeType::SouthEast,
        (false, true, false, true) => PipeType::EastWest,
        (false, false, true, true) => PipeType::SouthWest,
        _ => unreachable!(),
    };
    pipe_diagram.insert(starting_point, starting_pipe);
    let reached_nodes: Vec<_> = dijkstra_reach(&starting_point, |pos| {
        if let Some(pipe_type) = pipe_diagram.get(pos) {
            pipe_type
                .to_directions()
                .iter()
                .map(|direction| (direction + pos, 1))
                .collect()
        } else {
            vec![]
        }
    })
    .collect();
    let p1 = reached_nodes
        .iter()
        .map(|item| item.total_cost)
        .max()
        .unwrap();

    let main_loop: PipeDiagram = reached_nodes
        .iter()
        .map(|item| item.node)
        .map(|pos| (pos, *pipe_diagram.get(&pos).unwrap()))
        .collect();
    let width = input.lines().next().unwrap().len() as i32;
    let height = input.lines().count() as i32;
    let p2 = (1..(width - 1))
        .cartesian_product(1..(height - 1))
        .map(|(x, y)| ivec2(x, y))
        .filter(|pos| !main_loop.keys().contains(pos))
        .filter(|pos| {
            let mut intersect_count = 0i32;
            // traverse from the node to the east, checking for intersections by the even-odd rule
            for x in (pos.x + 1)..width {
                let check_pos = pos.with_x(x);
                if let Some(pipe_type) = main_loop.get(&check_pos) {
                    // take the position of the intersection to be the upper part of the node
                    if [
                        PipeType::NorthSouth,
                        PipeType::NorthWest,
                        PipeType::NorthEast,
                    ]
                    .contains(pipe_type)
                    {
                        intersect_count += 1;
                    }
                }
            }
            intersect_count % 2 == 1
        })
        .count();
    (p1.to_string(), p2.to_string())
}
