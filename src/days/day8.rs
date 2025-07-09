use std::{collections::HashMap, fs::read_to_string};

use nom::{
    IResult, Parser,
    bytes::complete::{tag, take},
    character::complete::line_ending,
    multi::{many_till, separated_list1},
    sequence::separated_pair,
};

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

struct Path<'a> {
    left: &'a str,
    right: &'a str,
}

type Network<'a> = HashMap<&'a str, Path<'a>>;

const _EX: &str = r"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

const _EX2: &str = r"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

const _EX3: &str = r"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

pub fn solve() -> (String, String) {
    // let input = _EX3.to_string();
    let input = read_to_string("inputs/input8.txt").expect("could not read input");

    let (_, (directions, network)) = parse_instructions(&input).expect("epic parse fail");

    let mut p1 = 0u32;
    let mut current_node = "AAA";
    let mut directions_cycle = directions.iter().cycle();
    while current_node != "ZZZ" {
        let path = network.get(current_node).expect("node to be in network");
        current_node = match directions_cycle.next().unwrap() {
            Direction::Left => path.left,
            Direction::Right => path.right,
        };
        p1 += 1;
    }

    let starting_nodes: Vec<_> = network
        .keys()
        .filter(|node| node.ends_with("A"))
        .cloned()
        .collect();
    let path_lengths: Vec<_> = starting_nodes
        .iter()
        .map(|node| {
            let mut count = 0u64;
            let mut node = *node;
            let mut directions_cycle = directions.iter().cycle();
            while !node.ends_with("Z") {
                let path = network.get(node).expect("node not in network");
                node = match directions_cycle.next().unwrap() {
                    Direction::Left => path.left,
                    Direction::Right => path.right,
                };
                count += 1;
            }

            count
        })
        .collect();
    let p2 = lcm(&path_lengths);

    (p1.to_string(), p2.to_string())
}

fn parse_instructions(input: &str) -> IResult<&str, (Vec<Direction>, Network)> {
    separated_pair(parse_directions, line_ending, parse_network).parse(input)
}

fn parse_directions(input: &str) -> IResult<&str, Vec<Direction>> {
    let (input, (directions, _)) = many_till(take(1usize), line_ending).parse(input)?;
    let directions: Vec<_> = directions
        .iter()
        .map(|ch| match *ch {
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => unreachable!(),
        })
        .collect();

    Ok((input, directions))
}

fn parse_network(input: &str) -> IResult<&str, Network> {
    let (input, paths) = separated_list1(line_ending, parse_path).parse(input)?;
    let network: Network = paths.into_iter().collect();

    Ok((input, network))
}

fn parse_path(input: &str) -> IResult<&str, (&str, Path)> {
    let (input, start) = take(3usize)(input)?;
    let (input, _) = tag(" = (")(input)?;
    let (input, left) = take(3usize)(input)?;
    let (input, _) = tag(", ")(input)?;
    let (input, right) = take(3usize)(input)?;
    let (input, _) = tag(")")(input)?;

    Ok((input, (start, Path { left, right })))
}

fn gcf(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }
    gcf(b, a % b)
}

fn lcm(nums: &[u64]) -> u64 {
    if nums.len() == 1 {
        return nums[0];
    }

    let a = nums[0];
    let b = lcm(&nums[1..]);

    a * b / gcf(a, b)
}
