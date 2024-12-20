use std::{
    collections::{HashMap, HashSet},
    io::{stdin, BufRead},
};

fn main() {
    let input = parse_input(stdin().lock());
    println!("part 1: {}", part_1(&input));
    println!("part 2: {}", part_2(&input));
}

type Position = (isize, isize);

#[derive(Debug, Clone)]
struct Bounds {
    top: isize,
    bottom: isize,
    left: isize,
    right: isize,
}

#[derive(Debug)]
struct Node {
    value: char,
    position: Position,
}

#[derive(Debug)]
struct AntennaMap {
    bounds: Bounds,
    nodes: Vec<Node>,
}

fn parse_input(input: impl BufRead) -> AntennaMap {
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();
    if grid.len() == 0 || grid[0].len() == 0 {
        panic!("invalid grid")
    };
    let bounds = Bounds {
        bottom: 0,
        left: 0,
        top: grid.len() as isize - 1,
        right: grid[0].len() as isize - 1,
    };
    let mut nodes = Vec::new();
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            let value = grid[r][c];
            if value == '.' {
                continue;
            }
            let x = c as isize;
            let y = bounds.top - r as isize;
            nodes.push(Node {
                value,
                position: (x, y),
            });
        }
    }
    AntennaMap { bounds, nodes }
}

fn part_1(input: &AntennaMap) -> usize {
    let mut grouped_nodes: HashMap<char, Vec<Position>> = HashMap::new();
    for node in input.nodes.iter() {
        grouped_nodes
            .entry(node.value)
            .or_insert_with(Vec::new)
            .push(node.position);
    }
    let next_anti_node = |a, b| anti_nodes(a, b).skip(1).take(1);
    let anti_nodes: HashSet<_> = grouped_nodes
        .values()
        .flat_map(|group| {
            in_order_pairs(group)
                .flat_map(|(a, b)| next_anti_node(*a, *b).chain(next_anti_node(*b, *a)))
        })
        .filter(|anit_node| is_in_bound(&input.bounds, *anit_node))
        .collect();

    anti_nodes.len()
}

fn part_2(input: &AntennaMap) -> usize {
    let mut grouped_nodes: HashMap<char, Vec<Position>> = HashMap::new();
    for node in input.nodes.iter() {
        grouped_nodes
            .entry(node.value)
            .or_insert_with(Vec::new)
            .push(node.position);
    }
    let anti_nodes: HashSet<_> = grouped_nodes
        .values()
        .flat_map(|group| {
            in_order_pairs(group).flat_map(|(a, b)| {
                in_bounds_anti_nodes(*a, *b, &input.bounds).chain(in_bounds_anti_nodes(
                    *b,
                    *a,
                    &input.bounds,
                ))
            })
        })
        .collect();
    anti_nodes.len()
}

fn in_order_pairs<T>(items: &[T]) -> impl Iterator<Item = (&T, &T)> {
    (0..items.len())
        .flat_map(|first| ((first + 1)..items.len()).map(move |second| (first, second)))
        .map(|(first, second)| (&items[first], &items[second]))
}

fn in_bounds_anti_nodes(
    a: Position,
    b: Position,
    bounds: &Bounds,
) -> impl Iterator<Item = Position> {
    let bounds = bounds.clone();
    anti_nodes(a, b).take_while(move |node| is_in_bound(&bounds, *node))
}

fn anti_nodes(a: Position, b: Position) -> impl Iterator<Item = Position> {
    let run = b.0 - a.0;
    let rise = b.1 - a.1;
    let mut current = b;
    (0..).map(move |_| {
        let result = current.clone();
        current = (current.0 + run, current.1 + rise);
        result
    })
}

fn is_in_bound(bounds: &Bounds, (x, y): Position) -> bool {
    bounds.left <= x && x <= bounds.right && bounds.bottom <= y && y <= bounds.top
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn test_part_1_example() {
        let input = parse_input(EXAMPLE.as_bytes());
        assert_eq!(part_1(&input), 14);
    }

    #[test]
    fn test_part_2_example() {
        let input = parse_input(EXAMPLE.as_bytes());
        assert_eq!(part_2(&input), 34);
    }

    #[test]
    fn test_in_order_pairs() {
        let items = [1, 2, 3, 4];
        let expected = vec![(1, 2), (1, 3), (1, 4), (2, 3), (2, 4), (3, 4)];
        let actual: Vec<_> = in_order_pairs(&items).map(|(a, b)| (*a, *b)).collect();
        assert_eq!(actual, expected);
    }
}
