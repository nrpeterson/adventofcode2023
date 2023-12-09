use std::collections::HashSet;
use nom::{
    IResult,
    bytes::complete::tag,
    character::complete::{digit1, space1},
    combinator::{map_res},
    multi::separated_list1,
    sequence::{pair, preceded, tuple}
};

fn number(input: &str) -> IResult<&str, u32> {
    map_res(digit1, |s| u32::from_str_radix(s, 10))(input)
}

fn numbers(input: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(space1, number)(input)
}

fn input_line(input: &str) -> IResult<&str, (Vec<u32>, Vec<u32>)> {
    pair(
        preceded(
            tuple((tag("Card"), space1, number, tag(":"), space1)),
            numbers
        ),
        preceded(
            tuple((space1, tag("|"), space1)),
            numbers
        )
    )(input)
}

fn main() {
    let input = include_str!("../input.txt");
    let cards: Vec<(HashSet<u32>, HashSet<u32>)> = input.lines()
        .map(|line| input_line(line).unwrap().1)
        .map(|(winning, chosen)| {
            (winning.into_iter().collect(), chosen.into_iter().collect())
        })
        .collect();

    let matches: Vec<usize> = cards.iter()
        .map(|(a, b) | {
            let isect: HashSet<&u32> = (*a).intersection(b).collect();
            isect.len()
        })
        .collect();

    let result1: usize = matches.iter()
        .map(|n| {
            if *n == 0 {
                0
            }
            else {
                2usize.pow((n - 1) as u32)
            }
        })
        .sum();

    let mut card_counts: Vec<usize> = vec![1; cards.len()];

    for i in 0..card_counts.len() {
        let count = card_counts[i];
        let ms = matches[i];

        for j in (i+1)..(i+1+ms) {
            card_counts[j] += count;
        }
    }

    let result2: usize = card_counts.iter().sum();

    println!("Part 1: {result1}");
    println!("Part 2: {result2}");
}
