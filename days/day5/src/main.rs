use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    io::{stdin, BufRead},
};

fn main() {
    let input = parse_input(stdin().lock());
    println!("part 1: {}", part_1(&input));
    println!("part 2: {}", part_2(&input));
}

#[derive(Default)]
struct Input {
    rules: HashMap<u32, HashSet<u32>>,
    lines: Vec<Vec<u32>>,
}

fn parse_input(input: impl BufRead) -> Input {
    let mut parsed = Input::default();
    input.lines().map(|line| line.unwrap()).for_each(|line| {
        if line.is_empty() {
            return;
        }
        if line.contains("|") {
            let (before, after) = line.split_once('|').unwrap();
            let (before, after) = (before.parse().unwrap(), after.parse().unwrap());
            let entry = parsed.rules.entry(before).or_insert(HashSet::new());
            entry.insert(after);
        } else {
            let nums = line.split(',').map(|num| num.parse().unwrap()).collect();
            parsed.lines.push(nums);
        }
    });
    parsed
}

fn part_1(input: &Input) -> u64 {
    input
        .lines
        .iter()
        .filter(|line| is_valid_line(line, &input.rules))
        .map(|valid_line| valid_line[valid_line.len() / 2] as u64)
        .sum()
}

fn is_valid_line(line: &[u32], rules: &HashMap<u32, HashSet<u32>>) -> bool {
    let mut cannot_be_before = HashSet::<u32>::new();
    for num in line.iter().rev() {
        if cannot_be_before.contains(num) {
            return false;
        }
        if let Some(after_set) = rules.get(num) {
            cannot_be_before.extend(after_set);
        }
    }
    return true;
}

fn part_2(input: &Input) -> u64 {
    input
        .lines
        .iter()
        .filter(|line| !is_valid_line(line, &input.rules))
        .map(|line| {
            let mut line = line.clone();
            line.sort_by(|a, b| {
                let after_a = &input.rules.get(a);
                let after_b = &input.rules.get(b);
                if after_a.map(|after| after.contains(b)).unwrap_or(false) {
                    Ordering::Less
                } else if after_b.map(|after| after.contains(a)).unwrap_or(false) {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            });
            line
        })
        .map(|valid_line| valid_line[valid_line.len() / 2] as u64)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn test_part_1_example() {
        let input = parse_input(EXAMPLE.as_bytes());
        assert_eq!(part_1(&input), 143);
    }

    #[test]
    fn test_part_2_example() {
        let input = parse_input(EXAMPLE.as_bytes());
        assert_eq!(part_2(&input), 123);
    }
}
