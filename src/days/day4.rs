use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{self, line_ending, space1},
    multi::separated_list1,
    sequence::separated_pair,
};
use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

#[derive(Debug)]
struct LottoCard {
    winning_numbers: HashSet<u32>,
    card_numbers: HashSet<u32>,
}

type LottoDeck = HashMap<usize, LottoCard>;

const _EX: &str = r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

pub fn solve() -> (String, String) {
    // let input = _EX.to_string();
    let input = read_to_string("inputs/input4.txt").expect("input not readable");
    let (_, cards) = parse_deck(&input).expect("parsing fail");

    let p1: u32 = cards
        .values()
        .map(|card| &card.winning_numbers & &card.card_numbers)
        .filter(|intersection| !intersection.is_empty())
        .map(|intersection| 2u32.pow(intersection.len() as u32 - 1))
        .sum();

    let mut card_counts = vec![1usize; cards.len()];
    card_counts.insert(0, 0);

    for id in 1..card_counts.len() {
        let won_cards = process_card(id, &cards);
        for i in 1..=won_cards {
            card_counts[id + i] += card_counts[id];
        }
    }

    let p2: usize = card_counts.iter().sum();

    (p1.to_string(), p2.to_string())
}

fn parse_deck(input: &str) -> IResult<&str, LottoDeck> {
    let (_, ids_and_cards) = separated_list1(line_ending, parse_card).parse(input)?;
    let deck: LottoDeck = ids_and_cards.into_iter().collect();

    Ok((input, deck))
}

fn parse_card(input: &str) -> IResult<&str, (usize, LottoCard)> {
    let (input, _) = (tag("Card"), space1).parse(input)?;
    let (input, card_id) = complete::usize(input)?;
    let (input, _) = (tag(":"), space1).parse(input)?;
    let (input, (winning_numbers, card_numbers)) = separated_pair(
        separated_list1(space1, complete::u32),
        (tag(" |"), space1),
        separated_list1(space1, complete::u32),
    )
    .parse(input)?;

    let winning_numbers: HashSet<_> = winning_numbers.into_iter().collect();
    let card_numbers: HashSet<_> = card_numbers.into_iter().collect();

    Ok((
        input,
        (
            card_id,
            LottoCard {
                winning_numbers,
                card_numbers,
            },
        ),
    ))
}

fn process_card(id: usize, deck: &LottoDeck) -> usize {
    if let Some(card) = deck.get(&id) {
        (&card.winning_numbers & &card.card_numbers).len()
    } else {
        0
    }
}
