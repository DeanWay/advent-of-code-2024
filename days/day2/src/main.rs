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
    reports: Vec<Vec<u32>>,
}

fn parse_input(input: impl BufRead) -> Input {
    let reports = input
        .lines()
        .map(|line| line.unwrap())
        .map(|line| line.split(" ").map(|num| num.parse().unwrap()).collect())
        .collect();
    Input { reports }
}

fn solution_1(input: &Input) -> u32 {
    input
        .reports
        .iter()
        .map(|report| is_safe(report))
        .map(|b| b as u32)
        .sum()
}

fn is_safe(report: &[u32]) -> bool {
    let mut prev_sign = None;
    for diff in report
        .windows(2)
        .map(|window| window[0] as i32 - window[1] as i32)
    {
        let sign = diff.signum();
        if sign == 0 {
            return false;
        }
        if prev_sign.map(|prev| prev != sign).unwrap_or(false) {
            return false;
        }
        if diff.abs() > 3 {
            return false;
        }
        prev_sign = Some(sign)
    }
    return true;
}

fn solution_2(input: &Input) -> u32 {
    input
        .reports
        .iter()
        .map(|report| is_safe_with_problem_dampener(report))
        .map(|b| b as u32)
        .sum()
}

fn is_safe_with_problem_dampener(report: &[u32]) -> bool {
    (0..report.len())
        .map(|remove_i| {
            let mut report: Vec<u32> = report.iter().cloned().collect();
            report.remove(remove_i);
            report
        })
        .any(|report| is_safe(&report))
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn test_solution_1_example() {
        let input = parse_input(EXAMPLE.as_bytes());
        assert_eq!(solution_1(&input), 2);
    }

    #[test]
    fn test_solution_2_example() {
        let input = parse_input(EXAMPLE.as_bytes());
        assert_eq!(solution_2(&input), 4);
    }
}
