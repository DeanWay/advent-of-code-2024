use std::{
    collections::{HashSet, VecDeque},
    io::{stdin, BufRead},
};

fn main() {
    let input = parse_input(stdin().lock());
    println!("part 1: {}", part_1(&input));
    println!("part 2: {}", part_2(&input));
}

type Input = Vec<Vec<char>>;
type Position = (isize, isize);

struct Shape {
    positions: HashSet<Position>,
    perimeter: i64,
}

impl Shape {
    fn area(&self) -> i64 {
        self.positions.len() as i64
    }
    fn number_of_sides(&self) -> i64 {
        let top = self.positions.iter().map(|(r, _)| *r).min().unwrap();
        let bottom = self.positions.iter().map(|(r, _)| *r).max().unwrap();
        let left = self.positions.iter().map(|(_, c)| *c).min().unwrap();
        let right = self.positions.iter().map(|(_, c)| *c).max().unwrap();

        let mut vertical_sides = 0;
        for col in left..=right {
            let mut last_had_left_line = false;
            let mut last_had_right_line = false;
            for row in top..=bottom {
                if !self.positions.contains(&(row, col)) {
                    last_had_left_line = false;
                    last_had_right_line = false;
                    continue;
                }
                let has_left_line = !self.positions.contains(&(row, col - 1));
                let has_right_line = !self.positions.contains(&(row, col + 1));
                if has_left_line && !last_had_left_line {
                    vertical_sides += 1;
                }
                if has_right_line && !last_had_right_line {
                    vertical_sides += 1;
                }
                last_had_left_line = has_left_line;
                last_had_right_line = has_right_line;
            }
        }

        let mut horizontal_sides = 0;
        for row in top..=bottom {
            let mut last_had_top_line = false;
            let mut last_had_bottom_line = false;
            for col in left..=right {
                if !self.positions.contains(&(row, col)) {
                    last_had_top_line = false;
                    last_had_bottom_line = false;
                    continue;
                }
                let has_top_line = !self.positions.contains(&(row - 1, col));
                let has_bottom_line = !self.positions.contains(&(row + 1, col));
                if has_top_line && !last_had_top_line {
                    horizontal_sides += 1;
                }
                if has_bottom_line && !last_had_bottom_line {
                    horizontal_sides += 1;
                }
                last_had_top_line = has_top_line;
                last_had_bottom_line = has_bottom_line;
            }
        }

        vertical_sides + horizontal_sides
    }
}

fn parse_input(input: impl BufRead) -> Input {
    input
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect()
}

fn part_1(input: &Input) -> i64 {
    let mut cost = 0;
    let mut seen = HashSet::new();
    for r in 0..input.len() {
        for c in 0..input[0].len() {
            let position = (r as isize, c as isize);
            if seen.contains(&position) {
                continue;
            }
            if let Some(shape) = build_shape(input, position) {
                cost += shape.area() * shape.perimeter;
                seen.extend(shape.positions);
            }
        }
    }
    cost
}

fn part_2(input: &Input) -> i64 {
    let mut cost = 0;
    let mut seen = HashSet::new();
    for r in 0..input.len() {
        for c in 0..input[0].len() {
            let position = (r as isize, c as isize);
            if seen.contains(&position) {
                continue;
            }
            if let Some(shape) = build_shape(input, position) {
                cost += shape.area() * shape.number_of_sides();
                seen.extend(shape.positions);
            }
        }
    }
    cost
}

fn build_shape(input: &Input, position: Position) -> Option<Shape> {
    if !is_in_bound(input, position) {
        return None;
    }
    let mut positions: HashSet<Position> = HashSet::new();
    let mut perimeter = 0;
    let value = input[position.0 as usize][position.1 as usize];
    let mut queue = VecDeque::new();
    queue.push_front(position);
    while let Some(current) = queue.pop_front() {
        if positions.contains(&current) {
            continue;
        }
        perimeter += 4;
        neighbors(current)
            .filter(|n| is_in_bound(input, *n))
            .filter(|&(r, c)| input[r as usize][c as usize] == value)
            .for_each(|n| {
                perimeter -= 1;
                queue.push_front(n);
            });
        positions.insert(current);
    }
    Some(Shape {
        positions,
        perimeter,
    })
}

fn is_in_bound(input: &Input, (row, col): Position) -> bool {
    0 <= row && row < input.len() as isize && 0 <= col && col < input[0].len() as isize
}

fn neighbors((row, col): Position) -> impl Iterator<Item = Position> {
    [
        (row + 1, col),
        (row, col + 1),
        (row - 1, col),
        (row, col - 1),
    ]
    .into_iter()
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE1: &str = include_str!("../example.txt");
    const EXAMPLE2: &str = include_str!("../example2.txt");
    const EXAMPLE3: &str = include_str!("../example3.txt");
    const EXAMPLE4: &str = include_str!("../example4.txt");

    #[test]
    fn test_part_1_example_1() {
        let input = parse_input(EXAMPLE1.as_bytes());
        assert_eq!(part_1(&input), 140);
    }
    #[test]
    fn test_part_1_example_2() {
        let input = parse_input(EXAMPLE2.as_bytes());
        assert_eq!(part_1(&input), 1930);
    }

    #[test]
    fn test_part_2_example_1() {
        let input = parse_input(EXAMPLE1.as_bytes());
        assert_eq!(part_2(&input), 80);
    }

    #[test]
    fn test_part_2_example_2() {
        let input = parse_input(EXAMPLE2.as_bytes());
        assert_eq!(part_2(&input), 1206);
    }

    #[test]
    fn test_part_2_example_3() {
        let input = parse_input(EXAMPLE3.as_bytes());
        assert_eq!(part_2(&input), 236);
    }

    #[test]
    fn test_part_2_example_4() {
        let input = parse_input(EXAMPLE4.as_bytes());
        assert_eq!(part_2(&input), 368);
    }
}
