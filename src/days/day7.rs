use nom::{
    IResult, Parser,
    bytes::complete::take,
    character::complete::{self, line_ending, space1},
    multi::{count, separated_list1},
};
use std::{collections::HashMap, fs::read_to_string};

#[derive(Clone, Copy)]
struct Play {
    hand: [u8; 5],
    bid: u64,
    hand_type: HandType,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum HandType {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

const _EX: &str = r"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

pub fn solve() -> (String, String) {
    // let input = _EX.to_string();
    let input = read_to_string("inputs/input7.txt").expect("input could not be read");

    let (_, plays) = parse_plays(&input).expect("epic parse fail");
    let mut plays_p1 = plays.clone();
    plays_p1.sort_by(|a, b| {
        if a.hand_type == b.hand_type {
            a.hand.cmp(&b.hand)
        } else {
            a.hand_type.cmp(&b.hand_type)
        }
    });
    let p1: u64 = plays_p1
        .iter()
        .enumerate()
        .map(|(i, Play { bid, .. })| (i + 1) as u64 * bid)
        .sum();

    let mut plays_p2: Vec<_> = plays
        .clone()
        .iter()
        .map(|&Play { hand, bid, .. }| {
            let hand = hand.map(|card| if card == 11 { 0 } else { card });
            let hand_type = get_hand_type(&hand);
            Play {
                hand,
                bid,
                hand_type,
            }
        })
        .collect();
    plays_p2.sort_by(|a, b| {
        if a.hand_type == b.hand_type {
            a.hand.cmp(&b.hand)
        } else {
            a.hand_type.cmp(&b.hand_type)
        }
    });
    let p2: u64 = plays_p2
        .iter()
        .enumerate()
        .map(|(i, Play { bid, .. })| (i + 1) as u64 * bid)
        .sum();

    (p1.to_string(), p2.to_string())
}

fn parse_plays(input: &str) -> IResult<&str, Vec<Play>> {
    separated_list1(line_ending, parse_play).parse(input)
}

fn parse_play(input: &str) -> IResult<&str, Play> {
    let (input, hand) = count(take(1usize), 5).parse(input)?;
    let (input, _) = space1(input)?;
    let (input, bid) = complete::u64(input)?;

    let hand: [u8; 5] = hand
        .into_iter()
        .take(5)
        .map(|s| match s {
            "2" => 2u8,
            "3" => 3,
            "4" => 4,
            "5" => 5,
            "6" => 6,
            "7" => 7,
            "8" => 8,
            "9" => 9,
            "T" => 10,
            "J" => 11,
            "Q" => 12,
            "K" => 13,
            "A" => 14,
            _ => unreachable!(),
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    let hand_type = get_hand_type(&hand);

    Ok((
        input,
        Play {
            hand,
            bid,
            hand_type,
        },
    ))
}

fn get_hand_type(hand: &[u8; 5]) -> HandType {
    let mut card_counts = HashMap::new();
    for card in hand {
        card_counts
            .entry(*card)
            .and_modify(|count| *count += 1)
            .or_insert(1u8);
    }

    let joker_count = card_counts.remove(&0).unwrap_or(0);
    let mut card_counts: Vec<_> = card_counts.values().cloned().collect();
    card_counts.sort();
    if let Some(last_card) = card_counts.last_mut() {
        *last_card += joker_count;
    } else {
        card_counts = vec![5];
    }

    if card_counts == vec![5] {
        HandType::FiveOfAKind
    } else if card_counts == vec![1, 4] {
        HandType::FourOfAKind
    } else if card_counts == vec![2, 3] {
        HandType::FullHouse
    } else if card_counts == vec![1, 1, 3] {
        HandType::ThreeOfAKind
    } else if card_counts == vec![1, 2, 2] {
        HandType::TwoPair
    } else if card_counts == vec![1, 1, 1, 2] {
        HandType::Pair
    } else {
        HandType::HighCard
    }
}
