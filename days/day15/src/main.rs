use std::{
    collections::{HashMap, HashSet, VecDeque},
    io::{stdin, BufRead},
};

use uuid::Uuid;

fn main() {
    let input = parse_input(stdin().lock());
    println!("part 1: {}", part_1(&input));
    println!("part 2: {}", part_2(&input));
}

type Position = (isize, isize);
type Direction = (isize, isize);
type Id = u128;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum ObjectType {
    Robot,
    Box,
    Wall,
}

#[derive(Clone, Copy, Debug)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

impl Move {
    fn to_direction(&self) -> Direction {
        match self {
            Self::Up => (-1, 0),
            Self::Down => (1, 0),
            Self::Right => (0, 1),
            Self::Left => (0, -1),
        }
    }
}

type InputMap = Vec<Vec<Option<ObjectType>>>;

struct Input {
    map: InputMap,
    moves: Vec<Move>,
}

struct GameState {
    bounds: (usize, usize),
    object_types: HashMap<Id, ObjectType>,
    object_positions: HashMap<Id, Vec<Position>>,
    position_to_objects: HashMap<Position, Id>,
}

impl GameState {
    #[allow(unused)]
    fn print(&self) {
        let (rows, cols) = self.bounds;
        for r in 0..rows {
            let r = r as isize;
            for c in 0..cols {
                let c = c as isize;
                let obj_type = self.get_position_type((r, c));
                match obj_type {
                    None => print!("."),
                    Some(ObjectType::Robot) => print!("@"),
                    Some(ObjectType::Wall) => print!("#"),
                    Some(ObjectType::Box) => print!("O"),
                }
            }
            println!();
        }
    }
    fn get_robot_id(&self) -> u128 {
        self.object_types
            .iter()
            .find(|&(_, v)| *v == ObjectType::Robot)
            .map(|(k, _)| *k)
            .unwrap()
    }

    fn get_robot_position(&self) -> Position {
        *self
            .object_positions
            .get(&self.get_robot_id())
            .unwrap()
            .iter()
            .next()
            .unwrap()
    }

    fn get_position_type(&self, position: Position) -> Option<ObjectType> {
        let pos_object_id = self.position_to_objects.get(&position);
        pos_object_id
            .and_then(|id| self.object_types.get(id))
            .map(|obj_type| *obj_type)
    }

    fn apply_moves(&mut self, moves: &[Move]) {
        // self.print();
        for robot_move in moves.iter() {
            self.apply_move(robot_move);
            // print!("{}[2J", 27 as char);
            // println!("move: {:?}", robot_move);
            // self.print();
            // println!();
            // sleep(Duration::from_millis(500));
        }
    }

    fn move_object(&mut self, object_id: u128, direction: Direction) {
        let object_positions = self.object_positions.get(&object_id).unwrap();
        let mut new_positions = Vec::new();
        for position in object_positions.iter() {
            self.position_to_objects.remove(&position);
            let move_to_pos = apply_movement(*position, direction);
            new_positions.push(move_to_pos);
        }
        for position in new_positions.iter() {
            self.position_to_objects.insert(*position, object_id);
        }
        self.object_positions.insert(object_id, new_positions);
    }

    fn apply_move(&mut self, robot_move: &Move) {
        let robot_id = self.get_robot_id();
        let robot_position = self.get_robot_position();
        let direction = robot_move.to_direction();
        let move_to_pos = apply_movement(robot_position, direction);

        let mut boxes_to_move = Vec::new();
        let mut colliding_positions = VecDeque::new();
        colliding_positions.push_back(move_to_pos);
        let mut checked = HashSet::new();
        while let Some(colliding_pos) = colliding_positions.pop_front() {
            let Some(colliding_object_id) = self.position_to_objects.get(&colliding_pos) else {
                continue;
            };
            if checked.contains(colliding_object_id) {
                checked.remove(colliding_object_id);
                continue;
            }
            checked.insert(colliding_object_id);
            let colliding_object_type = self.get_position_type(colliding_pos);
            if matches!(colliding_object_type, Some(ObjectType::Wall)) {
                // can't move into a wall
                return;
            }
            boxes_to_move.push(*colliding_object_id);
            for position in self
                .object_positions
                .get(colliding_object_id)
                .unwrap()
                .iter()
            {
                colliding_positions.push_back(apply_movement(*position, direction));
            }
        }
        for box_id in boxes_to_move.into_iter().rev() {
            self.move_object(box_id, direction);
        }
        self.move_object(robot_id, direction);
    }

    fn sum_of_box_gps_coordinates(&self) -> i128 {
        self.object_types
            .iter()
            .filter(|&(_, obj_type)| *obj_type == ObjectType::Box)
            .map(|(id, _)| {
                self.object_positions
                    .get(id)
                    .unwrap()
                    .iter()
                    .next()
                    .unwrap()
            })
            .map(|&(r, c)| r as i128 * 100 + c as i128)
            .sum()
    }
}

fn game_state_from_input(map: &InputMap) -> GameState {
    let mut object_types = HashMap::new();
    let mut object_positions = HashMap::new();
    let mut position_to_objects = HashMap::new();
    let mut insert_object = |object_type: ObjectType, pos: Position| {
        let object_id = Uuid::new_v4().as_u128();
        object_types.insert(object_id, object_type);
        object_positions.insert(object_id, vec![pos]);
        position_to_objects.insert(pos, object_id);
    };
    for r in 0..map.len() {
        for c in 0..map[0].len() {
            match &map[r][c] {
                None => {}
                Some(ObjectType::Box) => insert_object(ObjectType::Box, (r as isize, c as isize)),
                Some(ObjectType::Wall) => insert_object(ObjectType::Wall, (r as isize, c as isize)),
                Some(ObjectType::Robot) => {
                    insert_object(ObjectType::Robot, (r as isize, c as isize))
                }
            };
        }
    }
    GameState {
        object_types,
        object_positions,
        position_to_objects,
        bounds: (map.len(), map[0].len()),
    }
}

fn double_size_game_state_from_input(map: &InputMap) -> GameState {
    let mut object_types = HashMap::new();
    let mut object_positions = HashMap::new();
    let mut position_to_objects = HashMap::new();
    let mut insert_object = |object_type: ObjectType, positions: Vec<Position>| {
        let object_id = Uuid::new_v4().as_u128();
        object_types.insert(object_id, object_type);
        for position in positions.iter() {
            position_to_objects.insert(*position, object_id);
        }
        object_positions.insert(object_id, positions);
    };
    for r in 0..map.len() {
        for c in 0..map[0].len() {
            let value = &map[r][c];
            let (r, c) = (r as isize, c as isize);
            match value {
                None => {}
                Some(ObjectType::Box) => {
                    insert_object(ObjectType::Box, vec![(r, c * 2), (r, c * 2 + 1)])
                }
                Some(ObjectType::Wall) => {
                    insert_object(ObjectType::Wall, vec![(r, c * 2), (r, c * 2 + 1)])
                }
                Some(ObjectType::Robot) => insert_object(ObjectType::Robot, vec![(r, c * 2)]),
            };
        }
    }
    GameState {
        object_types,
        object_positions,
        position_to_objects,
        bounds: (map.len(), map[0].len() * 2),
    }
}

fn parse_input(mut input: impl BufRead) -> Input {
    let mut s = String::new();
    input.read_to_string(&mut s).unwrap();
    let (map, moves) = s.split_once("\n\n").unwrap();
    let map: Vec<Vec<Option<ObjectType>>> = map
        .lines()
        .enumerate()
        .map(|(_, line)| {
            line.chars()
                .enumerate()
                .map(|(_, char)| match char {
                    '.' => None,
                    '#' => Some(ObjectType::Wall),
                    'O' => Some(ObjectType::Box),
                    '@' => Some(ObjectType::Robot),
                    char => panic!("unexpected character {char}"),
                })
                .collect()
        })
        .collect();
    let moves = moves
        .chars()
        .filter_map(|c| match c {
            '^' => Some(Move::Up),
            'v' => Some(Move::Down),
            '>' => Some(Move::Right),
            '<' => Some(Move::Left),
            '\n' => None,
            c => panic!("unexpected character {c}"),
        })
        .collect();
    Input { map, moves }
}

fn part_1(input: &Input) -> i128 {
    let mut game_state = game_state_from_input(&input.map);
    game_state.apply_moves(&input.moves);
    game_state.sum_of_box_gps_coordinates()
}

fn part_2(input: &Input) -> i128 {
    let mut game_state = double_size_game_state_from_input(&input.map);
    game_state.apply_moves(&input.moves);
    game_state.sum_of_box_gps_coordinates()
}

fn apply_movement(position: Position, direction: Direction) -> Position {
    (position.0 + direction.0, position.1 + direction.1)
}

#[cfg(test)]
mod tests {
    use super::*;
    const SMALL_EXAMPLE: &str = include_str!("../example_small.txt");
    const LARGE_EXAMPLE: &str = include_str!("../example_large.txt");

    #[test]
    fn test_part_1_small_example() {
        let input = parse_input(SMALL_EXAMPLE.as_bytes());
        assert_eq!(part_1(&input), 2028);
    }

    #[test]
    fn test_part_1_example() {
        let input = parse_input(LARGE_EXAMPLE.as_bytes());
        assert_eq!(part_1(&input), 10092);
    }

    #[test]
    fn test_part_2_large_example() {
        let input = parse_input(LARGE_EXAMPLE.as_bytes());
        assert_eq!(part_2(&input), 9021);
    }
}
