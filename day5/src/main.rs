use nom::{
    IResult,
    bytes::complete::tag,
    character::complete::{alpha1, digit1, newline, space0, space1},
    combinator::{map_res, recognize},
    multi::{many1, separated_list1},
    sequence::{delimited, preceded, pair, separated_pair, terminated, tuple}
};

fn number(input: &str) -> IResult<&str, u32> {
    map_res(digit1, |s| u32::from_str_radix(s, 10))(input)
}

fn eol(input: &str) -> IResult<&str, &str> {
    recognize(pair(space0, newline))
}

fn seeds_line(input: &str) -> IResult<&str, Vec<u32>> {
    preceded(tag("seeds: "), many1(number))(input)
}

struct MapRange {
    dest_range_start: u32,
    source_range_start: u32,
    range_length: u32
}

fn map_range(input: &str) -> IResult<&str, MapRange> {
    let p = tuple((number, delimited(space1, number, space1), number));

    map_res(
        p,
        | (a, b, c) | {
            let res = MapRange {
                dest_range_start: a,
                source_range_start: b,
                range_length: c
            };
            Ok(res)
        }
    )(input)
}

struct Mapping {
    start_name: &'static str,
    end_name: &'static str,
    mapping: Vec<MapRange>
}

fn map_name_line(input: &str) -> IResult<&str, (&str, &str)> {
    terminated(separated_pair(alpha1, tag("-to-"), alpha1), tag(" map:"))(input)
}

fn mapping(input: &str) -> IResult<&str, Mapping> {
    let p = separated_pair(map_name_line, eol, separated_list1(eol, map_range));

    map_res(
        p,
        | ((start_name, end_name), mapping) | {
            Ok(Mapping { start_name, end_name, mapping })
        }
    )(input)
}

fn full_input(input: &str) -> IResult<&str, (Vec<u32>, Vec<Mapping>)> {
    separated_pair(
        seeds_line,
        many1(eol),
        separated_list1(many1(eol), mapping)
    )(input)
}

fn main() {
    let input = include_str!("../test.txt");;
}
