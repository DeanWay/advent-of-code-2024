use std::io::{stdin, BufRead};

fn main() {
    let input = parse_input(stdin().lock());
    println!("part 1: {}", part_1(&input));
    println!("part 2: {}", part_2(&input));
}

type Input = Vec<u8>;

fn parse_input(mut input: impl BufRead) -> Input {
    let mut s = String::new();
    input.read_to_string(&mut s).unwrap();
    s.chars().map(|c| c.to_digit(10).unwrap() as u8).collect()
}

fn expand_input(input: &Input) -> Vec<Option<u128>> {
    let mut array = Vec::new();
    let mut is_empty_block = false;
    let mut value: u128 = 0;
    for digit in input {
        for _ in 0..*digit {
            array.push(if is_empty_block { None } else { Some(value) });
        }
        if !is_empty_block {
            value += 1;
        }
        is_empty_block = !is_empty_block
    }
    array
}

fn part_1(input: &Input) -> u128 {
    let mut memory = expand_input(input);

    let mut front = 0;
    let mut back = memory.len() - 1;
    while front < back {
        while memory[front].is_some() {
            front += 1;
        }
        if front >= back {
            break;
        }
        while memory[back].is_none() {
            back -= 1;
        }
        if front >= back {
            break;
        }
        memory[front] = memory[back];
        memory[back] = None;
    }
    memory
        .into_iter()
        .filter_map(|value| value)
        .enumerate()
        .map(|(i, value)| i as u128 * value)
        .sum()
}

fn part_2(input: &Input) -> u128 {
    let mut memory = expand_input(input);

    let mut block_to_move_end = memory.len() - 1;
    while block_to_move_end > 0 {
        while memory[block_to_move_end].is_none() {
            block_to_move_end -= 1;
        }
        if block_to_move_end <= 0 {
            break;
        }
        let mut block_to_move_start = block_to_move_end;
        while block_to_move_start > 0
            && memory[block_to_move_start - 1] == memory[block_to_move_end]
        {
            block_to_move_start -= 1;
        }
        let block_to_move_size = block_to_move_end - block_to_move_start + 1;
        let mut front = 0;
        loop {
            let Some((mut empty_block_start, empty_block_end)) =
                find_next_empty_block(&memory, front)
            else {
                break;
            };
            if empty_block_start > block_to_move_start {
                break;
            }
            let empty_block_size = empty_block_end - empty_block_start + 1;
            if block_to_move_size > empty_block_size {
                front = empty_block_end + 1;
                continue;
            }
            for _ in 0..block_to_move_size {
                memory[empty_block_start] = memory[block_to_move_start];
                memory[block_to_move_start] = None;
                empty_block_start += 1;
                block_to_move_start += 1;
            }
            break;
        }
        block_to_move_end = block_to_move_start.max(1) - 1;
    }
    memory
        .into_iter()
        .enumerate()
        .filter(|(_, value)| value.is_some())
        .map(|(i, value)| i as u128 * value.unwrap())
        .sum()
}

fn find_next_empty_block(memory: &[Option<u128>], mut front: usize) -> Option<(usize, usize)> {
    while front < memory.len() && memory[front].is_some() {
        front += 1;
    }
    if front >= memory.len() {
        return None;
    }
    let mut empty_block_end = front;
    while empty_block_end + 1 < memory.len() && memory[empty_block_end + 1].is_none() {
        empty_block_end += 1;
    }
    Some((front, empty_block_end))
}

#[allow(unused)]
fn print_mem(memory: &[Option<u128>]) {
    for item in memory {
        match item {
            None => print!("."),
            Some(value) => print!("{value}"),
        }
    }
    println!()
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn test_part_1_example() {
        let input = parse_input(EXAMPLE.as_bytes());
        assert_eq!(part_1(&input), 1928);
    }

    #[test]
    fn test_part_2_example() {
        let input = parse_input(EXAMPLE.as_bytes());
        assert_eq!(part_2(&input), 2858);
    }
}
