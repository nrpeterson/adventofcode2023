use std::cmp::max;
use nom::{
    IResult,
    Finish,
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, space1},
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::{pair, preceded, separated_pair, terminated}
};

enum Balls {
    Red(u32),
    Blue(u32),
    Green(u32)
}

struct Counts {
    red: u32,
    green: u32,
    blue: u32
}

fn number(input: &str) -> IResult<&str, u32> {
    map_res(digit1, str::parse)(input)
}

fn red_balls(input: &str) -> IResult<&str, Balls> {
    map(terminated(number, pair(space1, tag("red"))), Balls::Red)(input)
}

fn green_balls(input: &str) -> IResult<&str, Balls> {
    map(terminated(number, pair(space1, tag("green"))), Balls::Green)(input)
}

fn blue_balls(input: &str) -> IResult<&str, Balls> {
    map(terminated(number, pair(space1, tag("blue"))), Balls::Blue)(input)
}

fn balls(input: &str) -> IResult<&str, Balls> {
    alt((red_balls, green_balls, blue_balls))(input)
}

fn round(input: &str) -> IResult<&str, Vec<Balls>> {
    separated_list1(tag(", "), balls)(input)
}

fn game_num(input: &str) -> IResult<&str, u32> {
    preceded(tag("Game "), number)(input)
}

fn game(input: &str) -> IResult<&str, (u32, Vec<Vec<Balls>>)> {
    separated_pair(game_num, tag(": "), separated_list1(tag("; "), round))(input)
}

fn balls_good(balls: &Balls) -> bool {
    match *balls {
        Balls::Red(n) => n <= 12,
        Balls::Green(n) => n <= 13,
        Balls::Blue(n) => n <= 14
    }
}

fn round_is_bad(round: &Vec<Balls>) -> bool {
    round.iter().any(balls_bad)
}

fn game_is_bad(rounds: &Vec<Vec<Balls>>) -> bool {
    rounds.iter().any(round_is_bad)
}

fn min_balls(rounds: &Vec<Vec<Balls>>) -> Counts {
    let mut counts = Counts { red: 0, blue: 0, green: 0 };

    for round in rounds {
        for ball in round {
            match *ball {
                Balls::Red(n) => counts.red = max(counts.red, n),
                Balls::Green(n) => counts.green = max(counts.green, n),
                Balls::Blue(n) => counts.blue = max(counts.blue, n)
            }
        }
    }

    counts
}

fn main() {
    let input = include_str!("../input.txt");

    let result1: u32 = input.lines()
        .map(|line| game(line).finish().unwrap().1)
        .filter_map( |(game_num, rounds)| {
                if game_is_bad(&rounds) { None }
                else { Some(game_num) }
            })
        .sum();

    let result2: u32 = input.lines()
        .map(|line| game(line).finish().unwrap().1)
        .map(|(_, rounds)| min_balls(&rounds))
        .map( |Counts { red: r, green: g, blue: b }| r * g * b )
        .sum();

    println!("Part 1: {result1}");
    println!("Part 2: {result2}");
}
