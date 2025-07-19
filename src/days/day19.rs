use std::{collections::HashMap, fs::read_to_string, ops::Range};

use nom::{
    IResult, Parser,
    bytes::complete::{tag, take_till, take_until},
    character::complete::{self, line_ending, one_of},
    multi::{separated_list0, separated_list1},
    sequence::separated_pair,
};

#[derive(Debug, Clone, Copy)]
struct Part {
    x: u16,
    m: u16,
    a: u16,
    s: u16,
}

#[derive(Debug, Clone)]
struct PartRanges {
    x: Range<u16>,
    m: Range<u16>,
    a: Range<u16>,
    s: Range<u16>,
}

#[derive(Debug, Clone)]
struct Workflow<'a> {
    rules: Vec<Rule<'a>>,
    otherwise: Action<'a>,
}

#[derive(Debug, Clone)]
struct Rule<'a> {
    category: Category,
    range: Range<u16>,
    action: Action<'a>,
}

#[derive(Debug, Clone, Copy)]
enum Category {
    X,
    M,
    A,
    S,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Action<'a> {
    SendTo(&'a str),
    Accept,
    Reject,
}

const _EX: &str = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

pub fn solve() -> (String, String) {
    let input = _EX.to_string();
    // let input = read_to_string("inputs/input19.txt").expect("could not read input");

    let (_, (workflows, parts)) = parse_input(&input).expect("epic parse fail");
    // dbg!(workflows, parts);

    let p1: u32 = parts
        .iter()
        .map(|part| {
            let mut action = Action::SendTo("in");
            'outer: while let Action::SendTo(label) = action {
                let workflow = workflows.get(label).unwrap();
                for rule in &workflow.rules {
                    use Category::{X, M, A, S};
                    match rule.category {
                        X => {
                            if rule.range.contains(&part.x) {
                                action = rule.action;
                                continue 'outer;
                            }
                        }
                        M => {
                            if rule.range.contains(&part.m) {
                                action = rule.action;
                                continue 'outer;
                            }
                        }
                        A => {
                            if rule.range.contains(&part.a) {
                                action = rule.action;
                                continue 'outer;
                            }
                        }
                        S => {
                            if rule.range.contains(&part.s) {
                                action = rule.action;
                                continue 'outer;
                            }
                        }
                    }
                }
                action = workflow.otherwise;
            }

            (action, part)
        })
        .filter(|(action, _)| action == &Action::Accept)
        .map(|(_, part)| {
            [part.x, part.m, part.a, part.s]
                .iter()
                .map(|n| *n as u32)
                .sum::<u32>()
        })
        .sum();

    let accept_workflows = workflows.iter().filter(|(_, workflow)| {
        workflow
            .rules
            .iter()
            .any(|rule| rule.action == Action::Accept)
            || workflow.otherwise == Action::Accept
    });

    let p2 = accept_workflows.flat_map(|(w_label, workflow)| {
        let part_ranges = PartRanges { x: 1..4001, m: 1..4001, a: 1..4001, s: 1..4001 };
        let mut part_ranges_collection = vec![];
        if workflow.otherwise == Action::Accept {
            let range_opposites = workflow
                    .rules
                    .iter()
                    .map(|rule| (rule.category, range_opposites(&rule.range)));
                let new_part_ranges_iter = range_opposites
                    .flat_map(|(category, ranges)|
                        ranges.iter().map(|range| new_part_ranges(&part_ranges, category, range)).collect::<Vec<_>>()
                    );
                
                part_ranges_collection.append(&mut new_part_ranges_iter
                    .flat_map(|new_part_ranges| find_accepted_ranges(&new_part_ranges, &workflows, w_label)).collect::<Vec<_>>());
        }
        for Rule { category, range, action} in &workflow.rules {
            if action == &Action::Accept {
                part_ranges_collection.append(&mut find_accepted_ranges(&new_part_ranges(&part_ranges, *category, range), &workflows, w_label));
            }
        }

        part_ranges_collection
    })
    .map(|PartRanges {x, m, a, s}| 
        [x, m, a, s].iter().map(|range| range.len()).product::<usize>()
    )
    .fold(0u128, |acc, x| acc + (x as u128));

    (p1.to_string(), "".to_string())
}

fn intersect_ranges(a: &Range<u16>, b: &Range<u16>) -> Range<u16> {
    (a.start.max(b.start))..(a.end.max(b.end))
}

fn range_opposites(range: &Range<u16>) -> [Range<u16>; 2] {
    [1..range.start, range.end..4001]
}

fn find_accepted_ranges(part_ranges: &PartRanges, workflows: &HashMap<&str, Workflow>, label: &str) -> Vec<PartRanges> {
    if label == "in" {
        return vec![part_ranges.clone()];
    }

    let applicable_workflows = workflows
        .iter()
        .filter(|(_, workflow)| workflow
            .rules
            .iter()
            .any(|rule| rule.action == Action::SendTo(label))
            || workflow.otherwise == Action::SendTo(label)
        );

    applicable_workflows
        .flat_map(|(w_label, workflow)| {
            let mut part_ranges_collection = vec![];
            if workflow.otherwise == Action::SendTo(label) {
                let range_opposites = workflow
                    .rules
                    .iter()
                    .map(|rule| (rule.category, range_opposites(&rule.range)));
                let new_part_ranges_iter = range_opposites
                    .flat_map(|(category, ranges)|
                        ranges.iter().map(|range| new_part_ranges(part_ranges, category, range)).collect::<Vec<_>>()
                    );
                
                part_ranges_collection.append(&mut new_part_ranges_iter
                    .flat_map(|new_part_ranges| find_accepted_ranges(&new_part_ranges, workflows, w_label)).collect::<Vec<_>>());
            }
            for Rule { category, range, action} in &workflow.rules {
                if action == &Action::SendTo(label) {
                    part_ranges_collection.append(&mut find_accepted_ranges(&new_part_ranges(part_ranges, *category, range), workflows, w_label));
                }
            }

            part_ranges_collection
        })
        .collect()
}

fn new_part_ranges(part_ranges: &PartRanges, category: Category, range: &Range<u16>) -> PartRanges {
    use Category::{X, M, A, S};
    match category {
        X => PartRanges {
            x: intersect_ranges(&part_ranges.x, range),
            ..part_ranges.clone()
        },
        M => PartRanges {
            m: intersect_ranges(&part_ranges.m, range),
            ..part_ranges.clone()
        },
        A => PartRanges {
            a: intersect_ranges(&part_ranges.a, range),
            ..part_ranges.clone()
        },
        S => PartRanges {
            s: intersect_ranges(&part_ranges.s, range),
            ..part_ranges.clone()
        },
    }
}

fn parse_input(input: &str) -> IResult<&str, (HashMap<&str, Workflow>, Vec<Part>)> {
    separated_pair(
        parse_workflows,
        (line_ending, line_ending),
        separated_list1(line_ending, parse_part),
    )
    .parse(input)
}

fn parse_workflows(input: &str) -> IResult<&str, HashMap<&str, Workflow>> {
    let (input, labels_and_workflows) =
        separated_list1(line_ending, parse_workflow).parse(input)?;

    Ok((input, labels_and_workflows.into_iter().collect()))
}

fn parse_workflow(input: &str) -> IResult<&str, (&str, Workflow)> {
    let (input, name) = take_until("{")(input)?;
    let (input, _) = tag("{")(input)?;
    let (input, rules) = separated_list0(tag(","), parse_rule).parse(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, otherwise) = parse_action(input)?;
    let (input, _) = tag("}")(input)?;

    Ok((input, (name, Workflow { rules, otherwise })))
}

fn parse_rule(input: &str) -> IResult<&str, Rule> {
    let (input, category) = one_of("xmas")(input)?;
    let (input, comparison_sign) = one_of("><")(input)?;
    let (input, num) = complete::u16(input)?;

    use Category::{X, M, A, S};
    let category = match category {
        'x' => X,
        'm' => M,
        'a' => A,
        's' => S,
        _ => unreachable!(),
    };

    let range: Range<u16> = match comparison_sign {
        '>' => num..4001,
        '<' => 1..num,
        _ => unreachable!(),
    };

    let (input, _) = tag(":")(input)?;
    let (input, action) = parse_action(input)?;

    Ok((
        input,
        Rule {
            category,
            range,
            action,
        },
    ))
}

fn parse_action(input: &str) -> IResult<&str, Action> {
    let (input, label) = take_till(|c: char| !c.is_alphabetic()).parse(input)?;

    Ok((
        input,
        match label {
            "A" => Action::Accept,
            "R" => Action::Reject,
            s => Action::SendTo(s),
        },
    ))
}

fn parse_part(input: &str) -> IResult<&str, Part> {
    let (input, _) = tag("{")(input)?;
    let (input, (x, _, m, _, a, _, s)) = (
        parse_rating,
        tag(","),
        parse_rating,
        tag(","),
        parse_rating,
        tag(","),
        parse_rating,
    )
        .parse(input)?;
    let (input, _) = tag("}")(input)?;

    Ok((input, Part { x, m, a, s }))
}

fn parse_rating(input: &str) -> IResult<&str, u16> {
    let (input, _) = (one_of("xmas"), tag("=")).parse(input)?;

    complete::u16(input)
}
