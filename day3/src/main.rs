use std::collections::{HashMap, HashSet};

#[derive(Copy, Clone, Debug)]
struct Number {
    value: u32,
    row: usize,
    col_a: usize,
    col_b: usize,
}

fn adjacent_symbols(number: Number, symbols: &HashSet<(usize, usize)>) -> Vec<(usize, usize)> {
    let Number { value: _, row, col_a, col_b } = number;

    let neighbors = match (row, col_a) {
        (0, 0) => vec![(0, 1), (1, 0), (1, 1)],
        (0, col_a) => {
            let mut v: Vec<(usize, usize)> = (col_a - 1..col_b + 1).map(|j| (1, j)).collect();
            v.push((0, col_a - 1));
            v.push((0, col_b));
            v
        }
        (row, 0) => {
            let mut v: Vec<(usize, usize)> = (0..col_b + 1).map(|j| (row - 1, j)).collect();
            v.extend((0..col_b + 1).map(|j| (row + 1, j)));
            v.push((row, col_b));
            v
        }
        (row, col_a) => {
            let mut v: Vec<(usize, usize)> = (col_a - 1..col_b + 1).map(|j| (row - 1, j)).collect();
            v.extend((col_a - 1..col_b + 1).map(|j| (row + 1, j)));
            v.push((row, col_a - 1));
            v.push((row, col_b));
            v
        }
    };

    let mut result = Vec::new();

    for neighbor in neighbors {
        if symbols.contains(&neighbor) {
            result.push(neighbor);
        }
    }

    result
}

fn main() {
    let input = include_str!("../input.txt");

    let mut nums: Vec<Number> = Vec::new();
    let mut symbols: HashSet<(usize, usize)> = HashSet::new();
    let mut asterisks: HashSet<(usize, usize)> = HashSet::new();

    for (i, line) in input.lines().enumerate() {
        let mut num_start = None;
        let mut num: u32 = 0;

        for (j, c) in line.chars().enumerate() {
            if c == '*' {
                asterisks.insert((i, j));
            }

            match (c == '.', c.to_digit(10), num_start) {
                (false, Some(d), Some(_)) => num = 10 * num + d,
                (false, Some(d), None) => {
                    num_start = Some(j);
                    num = d;
                }
                (false, None, Some(col_a)) => {
                    // Symbol and end of number
                    symbols.insert((i, j));
                    nums.push(Number { value: num, row: i, col_a, col_b: j });
                    num_start = None;
                }
                (false, None, None) => {
                    // Symbol, not end of number
                    symbols.insert((i, j));
                }
                (true, _, Some(col_a)) => {
                    nums.push(Number { value: num, row: i, col_a, col_b: j });
                    num_start = None;
                }
                _ => {}
            }
        }

        if let Some(col_a) = num_start {
            nums.push(Number { value: num, row: i, col_a, col_b: line.len() })
        }
    }

    let mut symb_neighbors: HashMap<(usize, usize), Vec<Number>> = HashMap::new();
    let mut result1: u32 = 0;

    for num in nums {
        let adj_symbs = adjacent_symbols(num, &symbols);

        if !adj_symbs.is_empty() {
            result1 += num.value;
        }

        for symb in adj_symbs {
            symb_neighbors.entry(symb).or_default().push(num);
        }
    }

    let mut result2: u32 = 0;
    for coord in asterisks {
        let nums = symb_neighbors.get(&coord).unwrap();
        if nums.len() == 2 {
            result2 += nums[0].value * nums[1].value;
        }
    }

    println!("Part 1: {result1}");
    println!("Part 2: {result2}");
}
