use std::io::{stdin, BufRead};

use regex::Regex;

fn main() {
    let input = parse_input(stdin().lock());
    println!("part 1: {}", part_1(&input));
    println!("part 2: {}", part_2(&input));
}

type Input = String;

fn parse_input(mut input: impl BufRead) -> Input {
    let mut s = String::new();
    input.read_to_string(&mut s).unwrap();
    s
}

fn part_1(input: &Input) -> u64 {
    let regex = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    regex
        .captures_iter(input)
        .map(|cap| cap.extract())
        .map(|(_, [x, y])| x.parse::<u64>().unwrap() * y.parse::<u64>().unwrap())
        .sum()
}

fn part_2(input: &Input) -> u64 {
    let regex = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)|don't\(\)|do\(\)").unwrap();
    let capture_regex = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    let mut enabled = true;
    let mut sum = 0;
    for m in regex.find_iter(input).map(|m| m.as_str()) {
        if m == "don't()" {
            enabled = false
        } else if m == "do()" {
            enabled = true
        } else if enabled {
            let (_, [x, y]) = capture_regex.captures(m).unwrap().extract();
            sum += x.parse::<u64>().unwrap() * y.parse::<u64>().unwrap()
        }
    }
    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("../example.txt");
    const EXAMPLE2: &str = include_str!("../example2.txt");

    #[test]
    fn test_part_1_example() {
        let input = parse_input(EXAMPLE.as_bytes());
        assert_eq!(part_1(&input), 161);
    }

    #[test]
    fn test_part_2_example() {
        let input = parse_input(EXAMPLE2.as_bytes());
        assert_eq!(part_2(&input), 48);
    }
}
