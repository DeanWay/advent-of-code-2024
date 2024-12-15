use std::sync::{OnceLock, RwLock};
use std::{
    collections::HashMap,
    io::{stdin, BufRead},
};

fn main() {
    let input = parse_input(stdin().lock());
    println!("part 1: {}", part_1(&input));
    println!("part 2: {}", part_2(&input));
}

type Input = Vec<u128>;

fn parse_input(mut input: impl BufRead) -> Input {
    let mut s = String::new();
    input.read_to_string(&mut s).unwrap();
    s.split(' ').map(|val| val.parse().unwrap()).collect()
}

fn part_1(input: &Input) -> u128 {
    count_stones_after_n_blinks(input, 25)
}

fn part_2(input: &Input) -> u128 {
    count_stones_after_n_blinks(input, 75)
}

fn count_stones_after_n_blinks(stones: &[u128], blinks: usize) -> u128 {
    stones.iter().map(|stone| blinks_dfs(*stone, blinks)).sum()
}

static BLINKS_DFS_MEMO: OnceLock<RwLock<HashMap<(u128, usize), u128>>> = OnceLock::new();

fn blinks_dfs_memo() -> &'static RwLock<HashMap<(u128, usize), u128>> {
    BLINKS_DFS_MEMO.get_or_init(|| RwLock::new(HashMap::new()))
}

fn blinks_dfs(stone: u128, to_go: usize) -> u128 {
    let memo = blinks_dfs_memo();
    {
        let read_memo = memo.read().unwrap();
        if let Some(result) = read_memo.get(&(stone, to_go)) {
            return *result;
        }
    }
    let result = {
        if to_go == 0 {
            1
        } else if stone == 0 {
            blinks_dfs(1, to_go - 1)
        } else if count_digits(stone) % 2 == 0 {
            let (left, right) = split_digits(stone);
            blinks_dfs(left, to_go - 1) + blinks_dfs(right, to_go - 1)
        } else {
            blinks_dfs(stone * 2024, to_go - 1)
        }
    };
    let mut write_memo = memo.write().unwrap();
    write_memo.insert((stone, to_go), result);
    result
}

fn count_digits(num: u128) -> u64 {
    (num as f64).log10().floor() as u64 + 1
}

fn split_digits(num: u128) -> (u128, u128) {
    let len = count_digits(num);
    let half_len = len / 2;
    let divisor = 10u128.pow(half_len as u32);
    (num / divisor, num % divisor)
}

#[cfg(test)]
mod tests {

    use super::*;
    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn test_part_1_example() {
        let input = parse_input(EXAMPLE.as_bytes());
        assert_eq!(part_1(&input), 55312);
    }

    #[test]
    fn test_part_2_example() {
        let input = parse_input(EXAMPLE.as_bytes());
        assert_eq!(part_2(&input), 0);
    }

    #[test]
    fn test_count_digits() {
        assert_eq!(count_digits(0), 1);
        assert_eq!(count_digits(1), 1);
        assert_eq!(count_digits(125), 3);
        assert_eq!(count_digits(253000), 6);
        assert_eq!(count_digits(999), 3);
    }

    #[test]
    fn test_split_digits() {
        assert_eq!(split_digits(11), (1, 1));
        assert_eq!(split_digits(2024), (20, 24));
        assert_eq!(split_digits(2402), (24, 2));
        assert_eq!(split_digits(253000), (253, 000));
    }
}
