use std::{
    collections::{HashSet, VecDeque},
    io::{stdin, BufRead},
};

fn main() {
    let input = parse_input(stdin().lock());
    println!("part 1: {}", part_1(&input));
    println!("part 2: {}", part_2(&input));
}

type Input = Vec<Vec<i32>>;

fn parse_input(input: impl BufRead) -> Input {
    input
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect()
}

fn part_1(input: &Input) -> usize {
    let mut total = 0;
    for row in 0..input.len() {
        for col in 0..input[0].len() {
            if input[row][col] == 0 {
                total += trail_score(input, (row as isize, col as isize));
            }
        }
    }
    total
}

fn part_2(input: &Input) -> usize {
    let mut total = 0;
    for row in 0..input.len() {
        for col in 0..input[0].len() {
            if input[row][col] == 0 {
                total += trail_raiting(input, (row as isize, col as isize));
            }
        }
    }
    total
}

fn is_in_bound(input: &Input, (row, col): (isize, isize)) -> bool {
    0 <= row && row < input.len() as isize && 0 <= col && col < input[0].len() as isize
}

fn neighbors((row, col): (isize, isize)) -> impl Iterator<Item = (isize, isize)> {
    [
        (row + 1, col),
        (row, col + 1),
        (row - 1, col),
        (row, col - 1),
    ]
    .into_iter()
}

fn trail_score(input: &Input, starting_at: (isize, isize)) -> usize {
    let trail_ends: HashSet<_> = trails(input, starting_at)
        .into_iter()
        .map(|trail| trail[trail.len() - 1])
        .collect();
    trail_ends.len()
}

fn trail_raiting(input: &Input, starting_at: (isize, isize)) -> usize {
    trails(input, starting_at).len()
}

fn trails(input: &Input, starting_at: (isize, isize)) -> Vec<Vec<(isize, isize)>> {
    if !is_in_bound(input, starting_at) {
        return Vec::new();
    }
    if input[starting_at.0 as usize][starting_at.1 as usize] != 0 {
        return Vec::new();
    }
    let mut trails = VecDeque::new();
    trails.push_front(vec![starting_at]);
    let mut complete_trails = Vec::new();
    while let Some(current_trail) = trails.pop_back() {
        let current = current_trail.last().unwrap();
        let current_value = input[current.0 as usize][current.1 as usize];
        if current_value == 9 {
            complete_trails.push(current_trail);
            continue;
        }
        neighbors(*current)
            .filter(|neighbor| is_in_bound(input, *neighbor))
            .filter(|neighbor| {
                let neighbor_value = input[neighbor.0 as usize][neighbor.1 as usize];
                current_value + 1 == neighbor_value
            })
            .for_each(|neighbor| {
                let mut next_trail = current_trail.clone();
                next_trail.push(neighbor);
                trails.push_back(next_trail)
            })
    }
    complete_trails
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn test_part_1_example() {
        let input = parse_input(EXAMPLE.as_bytes());
        assert_eq!(part_1(&input), 36);
    }

    #[test]
    fn test_part_2_example() {
        let input = parse_input(EXAMPLE.as_bytes());
        assert_eq!(part_2(&input), 81);
    }
}
