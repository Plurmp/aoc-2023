use std::fs::read_to_string;

use glam::{U64Vec2, u64vec2, uvec2};
use itertools::Itertools;

const _EX: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

pub fn solve() -> (String, String) {
    // let input = _EX.to_string();
    let input = read_to_string("inputs/input11.txt").expect("could not read input");

    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut row_index = 0usize;
    let mut width = grid.first().unwrap().len();
    let mut row_spaces = vec![];
    let mut unscaled_row_index = 0usize;
    while row_index < width {
        if grid.iter().all(|row| row[row_index] == '.') {
            grid.iter_mut().for_each(|row| row.insert(row_index, '.'));
            row_index += 1;
            width += 1;
            row_spaces.push(unscaled_row_index);
        }
        row_index += 1;
        unscaled_row_index += 1;
    }
    let mut col_index = 0usize;
    let mut height = grid.len();
    let mut col_spaces = vec![];
    let mut unscaled_col_index = 0usize;
    while col_index < height {
        if grid[col_index].iter().all(|ch| *ch == '.') {
            grid.insert(col_index, vec!['.'; width]);
            col_index += 1;
            height += 1;
            col_spaces.push(unscaled_col_index);
        }
        col_index += 1;
        unscaled_col_index += 1;
    }
    // let grid_display = grid
    //     .iter()
    //     .map(|row| row.iter().collect::<String>())
    //     .join("\n");
    // println!("{}", grid_display);
    let positions: Vec<_> = grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter().enumerate().flat_map(move |(x, ch)| {
                if *ch == '#' {
                    Some(uvec2(x as u32, y as u32))
                } else {
                    None
                }
            })
        })
        .collect();
    let p1: u32 = positions
        .iter()
        .tuple_combinations()
        .map(|(a, b)| a.manhattan_distance(*b))
        .sum();

    let positions: Vec<_> = input
        .lines()
        .enumerate()
        .flat_map(|(y, row)| {
            row.char_indices().flat_map(move |(x, ch)| {
                if ch == '#' {
                    Some(u64vec2(x as u64, y as u64))
                } else {
                    None
                }
            })
        })
        .collect();
    let mut old_and_new_pos: Vec<_> = positions.into_iter().map(|pos| (pos, pos)).collect();
    for space_x in &row_spaces {
        for (old, new) in &mut old_and_new_pos {
            if old.x > *space_x as u64 {
                *new += U64Vec2::X * 999_999;
            }
        }
    }
    for space_y in &col_spaces {
        for (old, new) in &mut old_and_new_pos {
            if old.y > *space_y as u64 {
                *new += U64Vec2::Y * 999_999;
            }
        }
    }
    let p2: u64 = old_and_new_pos
        .iter()
        .tuple_combinations()
        .map(|(a, b)| a.1.manhattan_distance(b.1))
        .sum();

    (p1.to_string(), p2.to_string())
}
