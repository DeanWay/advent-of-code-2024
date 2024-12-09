use std::io::{stdin, BufRead};

fn main() {
    let input = parse_input(stdin().lock());
    println!("part 1: {}", part_1(&input));
    println!("part 2: {}", part_2(&input));
}

struct Equation {
    total: u64,
    operands: Vec<u64>,
}

type Input = Vec<Equation>;

fn parse_input(input: impl BufRead) -> Input {
    input
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            let (total, operands) = line.split_once(":").unwrap();
            let operands = operands
                .split(' ')
                .filter(|v| v.len() > 0)
                .map(|v| v.parse().unwrap())
                .collect();
            Equation {
                total: total.parse().unwrap(),
                operands,
            }
        })
        .collect()
}

fn part_1(input: &Input) -> u64 {
    fn is_possibly_equal(desired_total: u64, total_so_far: u64, operands: &[u64]) -> bool {
        if operands.is_empty() {
            return total_so_far == desired_total;
        }
        return is_possibly_equal(desired_total, total_so_far + operands[0], &operands[1..])
            || is_possibly_equal(desired_total, total_so_far * operands[0], &operands[1..]);
    }
    input
        .iter()
        .filter(|equation| {
            is_possibly_equal(
                equation.total,
                equation.operands[0],
                &equation.operands[1..],
            )
        })
        .map(|equation| equation.total as u64)
        .sum()
}

fn concat(a: u64, b: u64) -> u64 {
    a * 10u64.pow((b as f64).log10().floor() as u32 + 1) + b
}

fn part_2(input: &Input) -> u64 {
    fn is_possibly_equal(desired_total: u64, total_so_far: u64, operands: &[u64]) -> bool {
        if operands.is_empty() {
            return total_so_far == desired_total;
        }
        return is_possibly_equal(desired_total, total_so_far + operands[0], &operands[1..])
            || is_possibly_equal(desired_total, total_so_far * operands[0], &operands[1..])
            || is_possibly_equal(
                desired_total,
                concat(total_so_far, operands[0]),
                &operands[1..],
            );
    }
    input
        .iter()
        .filter(|equation| {
            is_possibly_equal(
                equation.total,
                equation.operands[0],
                &equation.operands[1..],
            )
        })
        .map(|equation| equation.total as u64)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn test_part_1_example() {
        let input = parse_input(EXAMPLE.as_bytes());
        assert_eq!(part_1(&input), 3749);
    }

    #[test]
    fn test_part_2_example() {
        let input = parse_input(EXAMPLE.as_bytes());
        assert_eq!(part_2(&input), 11387);
    }

    #[test]
    fn test_concat() {
        assert_eq!(concat(1, 2), 12);
        assert_eq!(concat(9, 9), 99);
        assert_eq!(concat(10, 10), 1010);
        assert_eq!(concat(100, 10), 10010);
        assert_eq!(concat(10, 100), 10100);
        assert_eq!(concat(999, 50), 99950);
        assert_eq!(concat(99, 50), 9950);
    }
}
