use std::{
    collections::HashMap,
    hash::Hash,
    io::{stdin, BufRead},
};

fn main() {
    let input = parse_input(stdin().lock());
    println!("solution 1: {}", solution_1(&input));
    println!("solution 2: {}", solution_2(&input));
}

#[derive(Clone)]
struct Input {
    left: Vec<u32>,
    right: Vec<u32>,
}

fn parse_input(input: impl BufRead) -> Input {
    input
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            let (left, right) = line.split_once("   ").unwrap();
            (left.parse().unwrap(), right.parse().unwrap())
        })
        .fold(
            Input {
                left: Vec::new(),
                right: Vec::new(),
            },
            |mut input, row| {
                input.left.push(row.0);
                input.right.push(row.1);
                input
            },
        )
}

fn solution_1(input: &Input) -> u32 {
    let Input {
        mut left,
        mut right,
    } = input.clone();
    left.sort();
    right.sort();
    left.into_iter()
        .zip(right.into_iter())
        .map(|(l, r)| ((l as i32) - (r as i32)).abs() as u32)
        .sum()
}

fn solution_2(input: &Input) -> u32 {
    let Input { left, right } = input;
    let right_counts = count_elems(right.iter());
    left.iter()
        .map(|elem| right_counts.get(elem).unwrap_or(&0) * elem)
        .sum()
}

fn count_elems<T: Eq + Hash + Clone>(iter: impl Iterator<Item = T>) -> HashMap<T, u32> {
    let mut counts = HashMap::new();
    for elem in iter {
        let entry = counts.entry(elem.clone()).or_insert(0);
        *entry += 1;
    }
    counts
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn test_solution_1_example() {
        let input = parse_input(EXAMPLE.as_bytes());
        assert_eq!(solution_1(&input), 11);
    }

    #[test]
    fn test_solution_2_example() {
        let input = parse_input(EXAMPLE.as_bytes());
        assert_eq!(solution_2(&input), 31);
    }
}
