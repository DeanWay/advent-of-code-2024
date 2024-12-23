use std::io::{stdin, BufRead};

use regex::Regex;

fn main() {
    let input = parse_input(stdin().lock());
    println!("part 1: {}", part_1(&input));
    println!("part 2: {}", part_2(&input));
}

type Input = Vec<ClawMachine>;

#[derive(Debug)]
struct ClawMachine {
    a_button: (u128, u128),
    b_button: (u128, u128),
    prize: (u128, u128),
}

fn parse_input(mut input: impl BufRead) -> Input {
    let button_a_re = Regex::new(r"Button A: X\+([0-9]+), Y\+([0-9]+)").unwrap();
    let button_b_re = Regex::new(r"Button B: X\+([0-9]+), Y\+([0-9]+)").unwrap();
    let prize_re = Regex::new(r"Prize: X=([0-9]+), Y=([0-9]+)").unwrap();

    let mut s = String::new();
    input.read_to_string(&mut s).unwrap();
    let sections = s.split("\n\n");
    sections
        .map(|secion| {
            let mut lines = secion.split("\n");
            let button_a_line = lines.next().unwrap();
            let button_b_line = lines.next().unwrap();
            let prize_line = lines.next().unwrap();
            let (_, [a_x, a_y]) = button_a_re.captures(button_a_line).unwrap().extract();
            let (_, [b_x, b_y]) = button_b_re.captures(button_b_line).unwrap().extract();
            let (_, [prize_x, prize_y]) = prize_re.captures(prize_line).unwrap().extract();
            ClawMachine {
                a_button: (a_x.parse().unwrap(), a_y.parse().unwrap()),
                b_button: (b_x.parse().unwrap(), b_y.parse().unwrap()),
                prize: (prize_x.parse().unwrap(), prize_y.parse().unwrap()),
            }
        })
        .collect()
}

fn part_1(input: &Input) -> u128 {
    input.iter().filter_map(tokens_needed_to_win).sum()
}

fn part_2(input: &Input) -> u128 {
    input
        .iter()
        .map(
            |ClawMachine {
                 a_button,
                 b_button,
                 prize,
             }| ClawMachine {
                a_button: *a_button,
                b_button: *b_button,
                prize: (prize.0 + 10000000000000, prize.1 + 10000000000000),
            },
        )
        .filter_map(|machine| tokens_needed_to_win(&machine))
        .sum()
}

fn tokens_needed_to_win(machine: &ClawMachine) -> Option<u128> {
    let a_slope = machine.a_button.1 as f64 / machine.a_button.0 as f64;
    let b_slope = machine.b_button.1 as f64 / machine.b_button.0 as f64;
    let max_a_presses =
        (machine.prize.0 / machine.a_button.0).min(machine.prize.1 / machine.a_button.1);
    let can_be_reached_with_bs = |distance_remaining: (u128, u128)| -> bool {
        distance_remaining.0 % machine.b_button.0 == 0
            && distance_remaining.1 % machine.b_button.1 == 0
            && distance_remaining.0 / machine.b_button.0
                == distance_remaining.1 / machine.b_button.1
    };
    let slope_to_prize = |position: (u128, u128)| -> f64 {
        let run = machine.prize.0 - position.0;
        let rise = machine.prize.1 - position.1;
        rise as f64 / run as f64
    };
    if can_be_reached_with_bs(machine.prize) {
        return Some((machine.prize.0 / machine.b_button.0) as u128);
    }
    let mut low = 0;
    let mut high = max_a_presses;
    while low <= high {
        let a_presses = (high - low) / 2 + low;
        let position = (
            machine.a_button.0 * a_presses,
            machine.a_button.1 * a_presses,
        );
        if position == machine.prize {
            return Some(a_presses * 3);
        }
        let distance_remaining = (machine.prize.0 - position.0, machine.prize.1 - position.1);
        if can_be_reached_with_bs(distance_remaining) {
            return Some(a_presses * 3 + (distance_remaining.0 / machine.b_button.0) as u128);
        }
        let remaining_slope = slope_to_prize(position);
        if (a_slope < b_slope && remaining_slope > b_slope)
            || (a_slope > b_slope && remaining_slope < b_slope)
        {
            if a_presses == 0 {
                break;
            }
            high = a_presses - 1;
        } else {
            low = a_presses + 1;
        }
    }
    return None;
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn test_part_1_example() {
        let input = parse_input(EXAMPLE.as_bytes());
        assert_eq!(part_1(&input), 480);
    }

    #[test]
    fn test_part_2_example() {
        let input = parse_input(EXAMPLE.as_bytes());
        assert_eq!(part_2(&input), 875318608908);
    }
}
