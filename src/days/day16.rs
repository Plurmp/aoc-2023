use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

use glam::{IVec2, ivec2};
use itertools::Itertools;
use rayon::prelude::*;

#[derive(Debug)]
enum Obstacle {
    Mirror(Mirror),
    Splitter(Splitter),
}

#[derive(Debug)]
enum Mirror {
    NE,
    NW,
}

#[derive(Debug)]
enum Splitter {
    NS,
    EW,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    N,
    S,
    E,
    W,
}

impl Direction {
    fn to_ivec2(self) -> IVec2 {
        match self {
            Self::N => ivec2(0, -1),
            Self::S => ivec2(0, 1),
            Self::E => ivec2(1, 0),
            Self::W => ivec2(-1, 0),
        }
    }
}

const _EX: &str = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";

pub fn solve() -> (String, String) {
    // let input = _EX.to_string();
    let input = read_to_string("inputs/input16.txt").expect("input could not be read");

    let obstacles: HashMap<_, _> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.char_indices().filter_map(move |(x, ch)| match ch {
                '/' => Some((ivec2(x as i32, y as i32), Obstacle::Mirror(Mirror::NE))),
                '\\' => Some((ivec2(x as i32, y as i32), Obstacle::Mirror(Mirror::NW))),
                '|' => Some((ivec2(x as i32, y as i32), Obstacle::Splitter(Splitter::NS))),
                '-' => Some((ivec2(x as i32, y as i32), Obstacle::Splitter(Splitter::EW))),
                _ => None,
            })
        })
        .collect();
    let width = input.lines().next().unwrap().len() as i32;
    let height = input.lines().count() as i32;
    let mut visited = HashSet::new();
    traverse(
        ivec2(0, 0),
        Direction::E,
        &obstacles,
        width,
        height,
        &mut visited,
    );
    let p1 = visited
        .iter()
        .map(|(pos, _)| pos)
        .collect::<HashSet<_>>()
        .len();

    let p2 = (0..width)
        .flat_map(|x| {
            [
                (ivec2(x, 0), Direction::S),
                (ivec2(x, height - 1), Direction::N),
            ]
        })
        .chain((0..height).flat_map(|y| {
            [
                (ivec2(0, y), Direction::E),
                (ivec2(width - 1, y), Direction::W),
            ]
        }))
        .collect::<Vec<_>>()
        .into_par_iter()
        .map(|(pos, dir)| {
            let mut visited = HashSet::new();
            traverse(pos, dir, &obstacles, width, height, &mut visited);

            visited
                .iter()
                .map(|(pos, _)| pos)
                .collect::<HashSet<_>>()
                .len()
        })
        .max()
        .unwrap();

    (p1.to_string(), p2.to_string())
}

fn traverse(
    pos: IVec2,
    dir: Direction,
    obstacles: &HashMap<IVec2, Obstacle>,
    width: i32,
    height: i32,
    visited: &mut HashSet<(IVec2, Direction)>,
) {
    // dbg!(pos, dir);
    if visited.contains(&(pos, dir))
        || !(0..width).contains(&pos.x)
        || !(0..height).contains(&pos.y)
    {
        return;
    }

    visited.insert((pos, dir));
    // println!("{}\n", display_visited(&visited.iter().map(|(pos, _)| pos).collect(), width as usize, height as usize));

    let current_obstacle = obstacles.get(&pos);

    use Direction as D;
    let next_dirs = match current_obstacle {
        None => vec![dir],
        Some(Obstacle::Mirror(Mirror::NE)) => match dir {
            D::N => vec![D::E],
            D::E => vec![D::N],
            D::S => vec![D::W],
            D::W => vec![D::S],
        },
        Some(Obstacle::Mirror(Mirror::NW)) => match dir {
            D::N => vec![D::W],
            D::W => vec![D::N],
            D::S => vec![D::E],
            D::E => vec![D::S],
        },
        Some(Obstacle::Splitter(Splitter::NS)) => match dir {
            D::N | D::S => vec![dir],
            D::E | D::W => vec![D::N, D::S],
        },
        Some(Obstacle::Splitter(Splitter::EW)) => match dir {
            D::E | D::W => vec![dir],
            D::N | D::S => vec![D::E, D::W],
        },
    };

    for next_dir in next_dirs {
        traverse(
            pos + next_dir.to_ivec2(),
            next_dir,
            obstacles,
            width,
            height,
            visited,
        );
    }
}

#[allow(unused)]
fn display_visited(visited: &HashSet<&IVec2>, width: usize, height: usize) -> String {
    vec![".".repeat(width); height]
        .join("\n")
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.char_indices()
                .map(|(x, _)| {
                    if visited.contains(&ivec2(x as i32, y as i32)) {
                        '#'
                    } else {
                        '.'
                    }
                })
                .collect::<String>()
        })
        .join("\n")
}
