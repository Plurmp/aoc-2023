use std::{fs::read_to_string, ops::Range};

use itertools::Itertools;
use nom::{
    IResult, Parser,
    bytes::complete::{tag, take_until},
    character::complete::{self, line_ending, space1},
    multi::separated_list1,
    sequence::pair,
};

#[derive(Debug)]
struct Almanac {
    seeds: Vec<i64>,
    maps: Vec<AlmanacMap>,
}

#[derive(Debug)]
struct AlmanacMap {
    entries: Vec<AlmanacEntry>,
}

#[derive(Debug, Clone)]
struct AlmanacEntry {
    source_start: i64,
    dest_start: i64,
    len: i64,
}

#[derive(Debug, Clone, Default)]
struct RangeSet(Vec<Range<i64>>);

impl RangeSet {
    fn union(&self, other: &Self) -> Self {
        match (self.0.is_empty(), other.0.is_empty()) {
            (true, true) => RangeSet::default(),
            (true, false) => other.clone(),
            (false, true) => self.clone(),
            _ => RangeSet(merge_range_sets(self.0.clone(), other.0.clone())),
        }
    }

    fn intersection(&self, other: &Self) -> Self {
        let mut result = vec![];
        for r1 in &self.0 {
            for r2 in &other.0 {
                if r1.start < r2.end && r1.end > r2.start {
                    let start = r1.start.max(r2.start);
                    let end = r1.end.min(r2.end);
                    result.push(start..end);
                }
            }
        }

        RangeSet(result)
    }

    fn difference(&self, other: &Self) -> Self {
        let mut result = self.0.clone();
        for other_range in &other.0 {
            let mut new_result = vec![];
            for range in &result {
                if range.start >= other_range.end || range.end <= other_range.start {
                    new_result.push(range.clone());
                } else {
                    if range.start < other_range.start {
                        new_result.push(range.start..other_range.start);
                    }
                    if range.end > other_range.end {
                        new_result.push(other_range.end..range.end);
                    }
                }
            }
            result = new_result;
        }

        RangeSet(result)
    }

    fn get_first(&self) -> Option<i64> {
        self.0.first().map(|r| r.start)
    }
}

fn merge_range_sets(a: Vec<Range<i64>>, b: Vec<Range<i64>>) -> Vec<Range<i64>> {
    if a.is_empty() && b.is_empty() {
        return vec![];
    }

    let mut combination = a;
    combination.extend(b);
    combination.sort_by(|a, b| a.start.cmp(&b.start));

    let mut result = Vec::new();
    let mut current = combination[0].clone();
    for range in combination[1..].iter() {
        if current.end < range.start {
            result.push(current);
            current = (*range).clone();
        } else {
            current.end = current.end.max(range.end);
        }
    }
    result.push(current);

    result
}

const _EX: &str = r"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

pub fn solve() -> (String, String) {
    // let input = _EX.to_string();
    let input = read_to_string("inputs/input5.txt").expect("input not readable");

    let (_, Almanac { seeds, maps }) = parse_almanac(&input).expect("epic parse fail");

    let mut seeds_p1 = seeds.clone();
    process_seeds(&mut seeds_p1, &maps);

    let p1 = seeds_p1.iter().min().unwrap();

    let seeds_p2: Vec<_> = seeds
        .iter()
        .tuples()
        .map(|(&start, &range)| (start..(start + range)))
        .collect();

    let mut location_ranges = RangeSet(merge_range_sets(vec![], seeds_p2));
    let mut ranges_to_add = RangeSet(vec![]);
    
    #[allow(clippy::single_range_in_vec_init)]
    for map in maps {
        dbg!(&location_ranges);
        for AlmanacEntry {
            dest_start,
            source_start,
            len,
        } in map.entries
        {
            let mapping_difference = dest_start - source_start;
            let mapping_range = RangeSet(vec![source_start..(source_start + len)]);
            let range_diff = mapping_range.intersection(&location_ranges);
            for diff in &range_diff.0 {
                ranges_to_add = ranges_to_add.union(&RangeSet(vec![
                    (diff.start + mapping_difference)..(diff.end + mapping_difference),
                ]))
            }
            location_ranges = location_ranges.difference(&range_diff);
        }
        location_ranges = location_ranges.union(&ranges_to_add);
        ranges_to_add = RangeSet(vec![]);
    }
    location_ranges = location_ranges.union(&ranges_to_add);

    dbg!(&location_ranges);

    let p2 = location_ranges.get_first().unwrap();

    (p1.to_string(), p2.to_string())
}

fn parse_almanac(input: &str) -> IResult<&str, Almanac> {
    let (input, _) = tag("seeds: ")(input)?;
    let (input, seeds) = separated_list1(space1, complete::i64).parse(input)?;
    let (input, _) = pair(line_ending, line_ending).parse(input)?;
    let (input, maps) = separated_list1(pair(line_ending, line_ending), parse_map).parse(input)?;

    Ok((input, Almanac { seeds, maps }))
}

fn parse_map(input: &str) -> IResult<&str, AlmanacMap> {
    let (input, _) = (take_until("map:"), tag("map:"), line_ending).parse(input)?;
    let (input, entries) = separated_list1(line_ending, parse_entry).parse(input)?;

    Ok((input, AlmanacMap { entries }))
}

fn parse_entry(input: &str) -> IResult<&str, AlmanacEntry> {
    let (input, (dest_start, _, source_start, _, len)) =
        (complete::i64, space1, complete::i64, space1, complete::i64).parse(input)?;

    Ok((
        input,
        AlmanacEntry {
            source_start,
            dest_start,
            len,
        },
    ))
}

fn process_seeds(seeds: &mut [i64], maps: &[AlmanacMap]) {
    for map in maps {
        for seed in seeds.iter_mut() {
            for entry in &map.entries {
                let difference = entry.dest_start - entry.source_start;
                if ((entry.source_start)..(entry.source_start + entry.len)).contains(seed) {
                    *seed += difference;
                    break;
                }
            }
        }
    }
}
