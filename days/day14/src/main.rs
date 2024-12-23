use std::{
    collections::HashMap,
    io::{stdin, BufRead},
};

use regex::Regex;

fn main() {
    let input = parse_input(stdin().lock());
    let bounds = (101, 103);
    println!("part 1: {}", part_1(&input, bounds));
    part_2(&input, bounds);
}

type Input = Vec<Robot>;

#[derive(Debug, Clone)]
struct Robot {
    position: (i32, i32),
    velocity: (i32, i32),
}

fn parse_input(input: impl BufRead) -> Input {
    let robot_re = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();
    input
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let (_, [p_x, p_y, v_x, v_y]) = robot_re.captures(&line).unwrap().extract();
            Robot {
                position: (p_x.parse().unwrap(), p_y.parse().unwrap()),
                velocity: (v_x.parse().unwrap(), v_y.parse().unwrap()),
            }
        })
        .collect()
}

fn part_1(input: &Input, (width, height): (i32, i32)) -> usize {
    let seconds = 100;
    let new_robot_positions: Vec<(i32, i32)> = input
        .iter()
        .map(|robot| {
            (
                (robot.position.0 + (robot.velocity.0 * seconds)).rem_euclid(width),
                (robot.position.1 + (robot.velocity.1 * seconds)).rem_euclid(height),
            )
        })
        .collect();
    let quadrant_count = |(top, left, bottom, right): (i32, i32, i32, i32)| {
        new_robot_positions
            .iter()
            .filter(|robot| {
                left <= robot.0 && robot.0 <= right && top <= robot.1 && robot.1 <= bottom
            })
            .count()
    };
    let top_left_count = quadrant_count((0, 0, height / 2 - 1, width / 2 - 1));
    let top_right_count = quadrant_count((0, width / 2 + 1, height / 2 - 1, width));
    let bottom_left_count = quadrant_count((height / 2 + 1, 0, height, width / 2 - 1));
    let bottom_right_count = quadrant_count((height / 2 + 1, width / 2 + 1, height, width));
    top_left_count * top_right_count * bottom_left_count * bottom_right_count
}

fn part_2(input: &Input, (width, height): (i32, i32)) {
    let mut robots: Vec<Robot> = input.clone();
    for seconds in 0..10000 {
        print_robots(&robots, (width, height), seconds);
        robots = robots
            .iter()
            .map(|robot| Robot {
                position: (
                    (robot.position.0 + (robot.velocity.0)).rem_euclid(width),
                    (robot.position.1 + (robot.velocity.1)).rem_euclid(height),
                ),
                velocity: robot.velocity,
            })
            .collect();
    }
}

fn print_robots(robots: &[Robot], (width, height): (i32, i32), seconds: usize) {
    let mut position_counts = HashMap::new();
    for robot in robots {
        let entry = position_counts.entry(robot.position).or_insert(0);
        *entry += 1;
    }
    let mut out = String::new();
    out.push_str(format!("{}[2J", 27 as char).as_str());
    out.push_str(format!("seconds: {seconds}\n").as_str());
    for y in 0..height {
        for x in 0..width {
            out.push_str(
                format!(
                    "{}",
                    position_counts
                        .get(&(x, y))
                        .map(|count| count.to_string())
                        .unwrap_or(".".to_string())
                )
                .as_str(),
            )
        }
        out.push('\n');
    }
    println!("{out}");
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn test_part_1_example() {
        let input = parse_input(EXAMPLE.as_bytes());
        assert_eq!(part_1(&input, (11, 7)), 12);
    }
}
