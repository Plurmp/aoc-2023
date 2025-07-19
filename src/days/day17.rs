use std::{
    cmp::{Ordering, Reverse}, collections::{BinaryHeap, HashMap, HashSet}, fs::read_to_string
};

use glam::{IVec2, ivec2};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum D {
    N,
    S,
    E,
    W,
}

impl D {
    fn to_ivec2(self) -> IVec2 {
        match self {
            D::N => ivec2(0, -1),
            D::S => ivec2(0, 1),
            D::E => ivec2(1, 0),
            D::W => ivec2(-1, 0),
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Step {
    pos: IVec2,
    dir: D,
    cur_cost: u32,
    step_count: u8,
}

impl Ord for Step {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.pos == other.pos {
            (self.dir, self.cur_cost, self.step_count).cmp(&(other.dir, other.cur_cost, other.step_count))
        } else {
            (self.pos.x, self.pos.y).cmp(&(other.pos.x, other.pos.y))
        }
    }
}

impl PartialOrd for Step {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

const _EX: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

pub fn solve() -> (String, String) {
    // let input = _EX.to_string();
    let input = read_to_string("inputs/input17.txt").expect("could not read input");

    let costs: HashMap<_, _> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.char_indices()
                .map(move |(x, ch)| (ivec2(x as i32, y as i32), ch.to_digit(10).unwrap()))
        })
        .collect();
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    dbg!(width, height);

    let p1 = find_cheapest_path(&costs, ivec2(width as i32 - 1, height as i32 - 1), 1, 3);

    let p2 = find_cheapest_path(&costs, ivec2(width as i32 - 1, height as i32 - 1), 4, 10);

    (p1.to_string(), p2.to_string())
}

fn find_cheapest_path(
    costs: &HashMap<IVec2, u32>,
    end: IVec2,
    min_steps: u8,
    max_steps: u8
) -> u32 {
    let mut queue = BinaryHeap::new();
    queue.push(Reverse((0u32, Step{ pos: ivec2(0, 0), dir: D::E, cur_cost: 0, step_count: 0})));
    queue.push(Reverse((0, Step{ pos: ivec2(0, 0), dir: D::S, cur_cost: 0, step_count: 0})));

    let mut visited = HashSet::new();
    visited.insert((ivec2(0, 0), D::E, 0u8));
    visited.insert((ivec2(0, 0), D::S, 0));

    while let Some(cur_queue_item) = queue.pop() {
        let cur_step = cur_queue_item.0.1;

        if cur_step.pos == end {
            if cur_step.step_count < min_steps {
                continue;
            } else {
                return cur_step.cur_cost;
            }
        }

        let next_dirs = if cur_step.step_count < min_steps {
            vec![cur_step.dir]
        } else if cur_step.step_count >= max_steps {
            match cur_step.dir {
                D::N | D::S => vec![D::E, D::W],
                D::E | D::W => vec![D::N, D::S],
            }
        } else {
            match cur_step.dir {
                D::N | D::S => vec![D::E, D::W, cur_step.dir],
                D::E | D::W => vec![D::N, D::S, cur_step.dir],
            }
        };

        for next_dir in next_dirs {
            let next_pos = cur_step.pos + next_dir.to_ivec2();
            let Some(next_cost) = costs.get(&next_pos) else {
                continue;
            };
            let next_step = Step {
                pos: next_pos,
                dir: next_dir,
                cur_cost: cur_step.cur_cost + next_cost,
                step_count: if cur_step.dir == next_dir {
                    cur_step.step_count + 1
                } else {
                    1
                }
            };
            let heuristic = cur_step.cur_cost + next_cost + next_step.pos.manhattan_distance(end);
            if visited.insert((next_step.pos, next_step.dir, next_step.step_count)) {
                queue.push(Reverse((heuristic, next_step)));
            }
        }
    }

    u32::MAX
}
