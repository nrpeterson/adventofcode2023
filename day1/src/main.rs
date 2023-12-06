fn main() {
    let input = include_str!("../input.txt");
    let result1: u32 = input.lines().map(get_value_pt1).sum();
    println!("Part 1: {result1}");

    let result2: u32 = input.lines().map(get_value_pt2).sum();
    println!("Part 2: {result2}");
}

fn get_value_pt1(line: &str) -> u32 {
    let mut result: u32 = 0;

    for c in line.chars() {
        if c.is_ascii_digit() {
            result += 10 * c.to_digit(10).unwrap();
            break;
        }
    }

    for c in line.chars().rev() {
        if c.is_ascii_digit() {
            result += c.to_digit(10).unwrap();
            break;
        }
    }

    result
}

fn get_value_pt2(line: &str) -> u32 {
    let options: [(&str, u32); 19] = [
        ("0", 0),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9)
    ];

    let mut result: u32 = 0;

    let mut init = line;
    'outer: loop {
        for (s, val) in options {
            if init.starts_with(s) {
                result += 10 * val;
                break 'outer;
            }
        }
        init = &init[1..];
    }

    let mut tail = line;
    'outer: loop {
        for (s, val) in options {
            if tail.ends_with(s) {
                result += val;
                break 'outer;
            }
        }
        tail = &tail[0..tail.len() - 1];
    }

    result
}
