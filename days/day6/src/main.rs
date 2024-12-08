use std::{
    collections::HashSet,
    io::{stdin, BufRead},
};

fn main() {
    let input = parse_input(stdin().lock());
    println!("part 1: {}", part_1(&input));
    println!("part 2: {}", part_2(&input));
}

type Input = Vec<Vec<char>>;

type Position = (isize, isize);
type Direction = (isize, isize);

#[derive(Clone, Hash, PartialEq, Eq)]
struct Guard {
    position: Position,
    direction: Direction,
}

#[derive(Clone)]
struct Map {
    initial_guard: Guard,
    width: usize,
    height: usize,
    walls: HashSet<Position>,
}

impl Map {
    fn is_in_bounds(&self, (row, col): Position) -> bool {
        row >= 0 && col >= 0 && row < self.height as isize && col < self.width as isize
    }

    fn is_wall(&self, position: Position) -> bool {
        self.walls.contains(&position)
    }
}

#[derive(Clone)]
struct GameState {
    map: Map,
    guard_path: Vec<Guard>,
    prev_guard_states: HashSet<Guard>,
}

impl GameState {
    fn new(map: Map) -> Self {
        Self {
            prev_guard_states: HashSet::new(),
            guard_path: vec![map.initial_guard.clone()],
            map,
        }
    }

    fn next(mut self) -> Self {
        if self.is_game_over() {
            return self.clone();
        }
        self.prev_guard_states.insert(self.current_guard().clone());
        let next_guard = self.next_guard();
        self.guard_path.push(next_guard);
        GameState {
            map: self.map,
            guard_path: self.guard_path,
            prev_guard_states: self.prev_guard_states,
        }
    }

    fn next_guard(&self) -> Guard {
        let current_guard = self.current_guard();
        let mut direction = current_guard.direction;
        for _attempt in 0..4 {
            let next_pos = apply_movement(current_guard.position, direction);
            if !self.map.is_wall(next_pos) {
                return Guard {
                    position: next_pos,
                    direction,
                };
            }
            direction = turn_right(direction)
        }
        panic!("unable to move");
    }

    fn current_guard(&self) -> &Guard {
        self.guard_path.last().unwrap()
    }

    fn is_game_over(&self) -> bool {
        let current_guard_position = self.current_guard().position;
        !self.map.is_in_bounds(current_guard_position)
    }

    fn is_in_loop(&self) -> bool {
        self.prev_guard_states.contains(self.current_guard())
    }

    fn guard_visited_positions(&self) -> HashSet<Position> {
        HashSet::from_iter(
            self.guard_path[0..self.guard_path.len() - 1]
                .iter()
                .map(|guard| guard.position)
                .filter(|position| self.map.is_in_bounds(*position)),
        )
    }

    #[allow(unused)]
    fn print(&self) {
        let guard = self.current_guard();
        for row in 0..self.map.height {
            for col in 0..self.map.width {
                let pos = (row as isize, col as isize);
                if self.map.is_wall(pos) {
                    print!("#")
                } else if guard.position == pos {
                    match guard.direction {
                        (-1, 0) => print!("^"),
                        (0, 1) => print!(">"),
                        (1, 0) => print!("v"),
                        (0, -1) => print!("<"),
                        _ => print!("?"),
                    }
                } else {
                    print!(".")
                }
            }
            println!();
        }
        println!();
    }
}

fn apply_movement(position: Position, direction: Direction) -> Position {
    (position.0 + direction.0, position.1 + direction.1)
}

fn turn_right((r_delta, c_delta): Direction) -> Direction {
    (c_delta, r_delta * -1)
}

fn parse_input(input: impl BufRead) -> Input {
    input
        .lines()
        .map(|line| line.unwrap())
        .map(|line| line.chars().collect())
        .collect()
}

fn input_to_map(input: &Input) -> Map {
    let height = input.len();
    let width = input[0].len();
    let mut guard_pos = None;
    let mut walls = HashSet::new();
    for row in 0..height {
        for col in 0..width {
            let value = input[row][col];
            let pos = (row as isize, col as isize);
            if value == '^' {
                guard_pos = Some(pos);
            }
            if value == '#' {
                walls.insert(pos);
            }
        }
    }
    Map {
        initial_guard: Guard {
            position: guard_pos.unwrap(),
            direction: (-1, 0),
        },
        width,
        height,
        walls,
    }
}

fn part_1(input: &Input) -> usize {
    let map = input_to_map(input);
    let mut game_state = GameState::new(map);
    while !game_state.is_game_over() {
        game_state = game_state.next()
    }
    game_state.guard_visited_positions().len()
}

fn part_2(input: &Input) -> usize {
    let mut possible_obstacle_positions: HashSet<Position> = HashSet::new();
    let map = input_to_map(input);
    let mut game_state = GameState::new(map);
    while !game_state.is_game_over() {
        game_state = game_state.next()
    }
    for possible_obstacle_postion in game_state
        .guard_visited_positions()
        .into_iter()
        .filter(|pos| *pos != game_state.map.initial_guard.position)
    {
        let mut possible_game_map = game_state.map.clone();
        possible_game_map.walls.insert(possible_obstacle_postion);
        let mut possible_game = GameState::new(possible_game_map);
        while !possible_game.is_game_over() {
            if possible_game.is_in_loop() {
                possible_obstacle_positions.insert(possible_obstacle_postion);
                break;
            }
            possible_game = possible_game.next()
        }
    }
    possible_obstacle_positions.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn test_part_1_example() {
        let input = parse_input(EXAMPLE.as_bytes());
        assert_eq!(part_1(&input), 41);
    }

    #[test]
    fn test_part_2_example() {
        let input = parse_input(EXAMPLE.as_bytes());
        assert_eq!(part_2(&input), 6);
    }
}
