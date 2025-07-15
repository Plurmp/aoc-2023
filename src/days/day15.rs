use std::{fs::read_to_string, num::Wrapping};

use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::{self, alpha1},
    multi::separated_list1,
};

type Box<'a> = Vec<Lens<'a>>;

#[derive(Debug, Clone, Copy)]
struct Lens<'a> {
    label: &'a str,
    focal_len: u8,
}

enum Action {
    Remove,
    FocalLen(u8),
}

const _EX: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

pub fn solve() -> (String, String) {
    // let input = _EX.to_string();
    let input = read_to_string("inputs/input15.txt").expect("input could not be read");

    let (_, sequence) = parse_sequence(&input).expect("epic parse fail");

    let p1: u32 = sequence.iter().map(|step| hash(step) as u32).sum();

    let mut boxes = vec![Box::new(); 256];
    for step in sequence {
        let (_, (label, action)) = parse_step(step).expect("epic parse fail");
        let hash = hash(label);
        if let Some(lens_box) = boxes.get_mut(hash as usize) {
            match action {
                Action::Remove => {
                    lens_box.retain(|lens| lens.label != label);
                }
                Action::FocalLen(focal_len) => {
                    if let Some(lens) = lens_box.iter_mut().find(|lens| lens.label == label) {
                        lens.focal_len = focal_len;
                    } else {
                        lens_box.push(Lens { label, focal_len });
                    }
                }
            }
        }
    }

    let p2: usize = boxes
        .iter()
        .enumerate()
        .map(|(box_idx, lens_box)| {
            lens_box
                .iter()
                .enumerate()
                .map(|(slot_idx, lens)| (box_idx + 1) * (slot_idx + 1) * (lens.focal_len as usize))
                .sum::<usize>()
        })
        .sum();

    (p1.to_string(), p2.to_string())
}

fn hash(input: &str) -> u8 {
    let input = input.as_bytes();
    input
        .iter()
        .fold(Wrapping(0u8), |acc, &x| (acc + Wrapping(x)) * Wrapping(17))
        .0
}

fn parse_sequence(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(tag(","), is_not(",")).parse(input)
}

fn parse_step(input: &str) -> IResult<&str, (&str, Action)> {
    let (input, label) = alpha1(input)?;
    let (input, action) = alt((parse_remove, parse_focal_len)).parse(input)?;

    Ok((input, (label, action)))
}

fn parse_remove(input: &str) -> IResult<&str, Action> {
    let (input, _) = tag("-").parse(input)?;

    Ok((input, Action::Remove))
}

fn parse_focal_len(input: &str) -> IResult<&str, Action> {
    let (input, (_, focal_len)) = (tag("="), complete::u8).parse(input)?;

    Ok((input, Action::FocalLen(focal_len)))
}
