use std::io::{stdin, BufRead};

fn main() {
    let input = parse_input(stdin().lock());
    println!("part 1: {}", part_1(&input));
    println!("part 2: {}", part_2(&input));
}

type Input = Vec<Vec<char>>;

fn parse_input(input: impl BufRead) -> Input {
    input
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect()
}

type Direction = (isize, isize);

fn part_1(input: &Input) -> u64 {
    let word = &['X', 'M', 'A', 'S'];
    let mut count_matches = 0;
    for r in 0..input.len() {
        for c in 0..input[r].len() {
            for direction in directions() {
                if search_in_direction(input, (r as isize, c as isize), direction, word) {
                    count_matches += 1;
                }
            }
        }
    }
    count_matches
}

fn directions() -> impl Iterator<Item = Direction> {
    [-1, 0, 1]
        .into_iter()
        .flat_map(|r| [-1, 0, 1].into_iter().map(move |c| (r, c)))
        .filter(|&direction| direction != (0, 0))
}

fn search_in_direction(
    input: &Input,
    (r, c): (isize, isize),
    (r_mod, c_mod): Direction,
    word: &[char],
) -> bool {
    if word.len() == 0 {
        return true;
    }
    if r < 0 || c < 0 || r as usize >= input.len() || c as usize >= input[r as usize].len() {
        return false;
    }
    if input[r as usize][c as usize] != word[0] {
        return false;
    }
    return search_in_direction(input, (r + r_mod, c + c_mod), (r_mod, c_mod), &word[1..]);
}

fn part_2(input: &Input) -> u64 {
    let word = &['M', 'A', 'S'];
    let mut count_matches = count_x_words(input, word);
    let mut input: Input = rotate_matrix(input);
    count_matches += count_x_words(&input, word);
    input = rotate_matrix(&input);
    count_matches += count_x_words(&input, word);
    input = rotate_matrix(&input);
    count_matches += count_x_words(&input, word);
    return count_matches;
}

fn rotate_matrix(input: &Input) -> Input {
    let mut output: Vec<Vec<char>> = input.clone();
    for r in 0..input.len() {
        for c in 0..input[r].len() {
            output[r][c] = input[c][input[r].len() - r - 1]
        }
    }
    output
}

fn count_x_words(input: &Input, word: &[char]) -> u64 {
    let mut count_matches = 0;

    for r in 0..input.len() {
        for c in 0..input[r].len() {
            if is_x_word_at_pos(input, (r, c), word) {
                count_matches += 1;
            }
        }
    }
    count_matches
}

fn is_x_word_at_pos(input: &Input, (r, c): (usize, usize), word: &[char]) -> bool {
    if r + word.len() > input.len() || c + word.len() > input[r].len() {
        return false;
    }
    let left_to_right_diagonal: Vec<char> = (0..word.len())
        .map(|delta| (r + delta, c + delta))
        .map(|(r, c)| input[r][c])
        .collect();

    let right_to_left_diagonal: Vec<char> = (0..word.len())
        .map(|delta| (r + delta, c + word.len() - 1 - delta))
        .map(|(r, c)| input[r][c])
        .collect();

    &left_to_right_diagonal == word && &right_to_left_diagonal == word
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn test_part_1_example() {
        let input = parse_input(EXAMPLE.as_bytes());
        assert_eq!(part_1(&input), 18);
    }

    #[test]
    fn test_part_2_example() {
        let input = parse_input(EXAMPLE.as_bytes());
        assert_eq!(part_2(&input), 9);
    }

    #[test]
    fn test_rotate_matrix() {
        let input = vec![
            vec!['1', '2', '3'],
            vec!['4', '5', '6'],
            vec!['7', '8', '9'],
        ];
        let expected_output = vec![
            vec!['3', '6', '9'],
            vec!['2', '5', '8'],
            vec!['1', '4', '7'],
        ];
        assert_eq!(rotate_matrix(&input), expected_output);

        assert_eq!(
            rotate_matrix(&rotate_matrix(&rotate_matrix(&rotate_matrix(&input)))),
            input
        )
    }
}
