use std::collections::HashMap;

mod input;

const DIRECTION: [(i32, i32); 8] = [(-1, 0), (-1, 1), (0, 1), (1, 1), (1, 0), (1, -1), (0, -1), (-1, -1)];
const XMAS: &str = "XMAS";

fn main() {
    let data = input::INPUT;
    let hashmap = make_hashmap(data);

    let xmas = part1(&hashmap, XMAS);
    let x_mas = part2(&hashmap);
    println!("XMAS: {xmas}");
    println!("X-MAS: {x_mas}");
}

// create a hashmap that maps grid position to char
fn make_hashmap(input: &str) -> HashMap::<(i32, i32), char> {
    let mut hashmap = HashMap::<(i32, i32), char>::new();
    for (i, row) in input.lines().enumerate() {
        for (j, col) in row.trim().chars().enumerate() {
            hashmap.insert((i as i32, j as i32), col);
        }
    }
    hashmap
}

// return count of words found in a grid in word search game
fn part1(grid: &HashMap<(i32, i32), char>, word: &str) -> i32 {
    let first_char = word.chars().nth(0).unwrap();
    let mut count = 0;

    for (position, letter) in grid.iter() {
        if *letter == first_char {
            for direction in DIRECTION.iter() {
                let mut current_pos = *position;
                let mut word_found = String::new();

                for _ in 0..word.len() {
                    if let Some(&c) = grid.get(&(current_pos.0, current_pos.1)) {
                        word_found.push(c);
                        current_pos = (current_pos.0 + direction.0, current_pos.1 + direction.1);
                    } else {
                        break;
                    }
                }

                if word_found == word {
                    count += 1;
                }
            }
        }
    }
    count
}

// count how many X-MAS is found in the grid
fn part2(grid: &HashMap<(i32, i32), char>) -> i32 {
    let mut count = 0;

    for (position, letter) in grid.iter() {
        if *letter == 'A' {
            let patterns = [
                [((-1, -1), 'M'), ((1, 1), 'S'), ((-1, 1), 'M'), ((1, -1), 'S')],
                [((-1, -1), 'M'), ((1, 1), 'S'), ((1, -1), 'M'), ((-1, 1), 'S')],
                [((-1, 1), 'M'), ((1, -1), 'S'), ((1, 1), 'M'), ((-1, -1), 'S')],
                [((1, 1), 'M'), ((-1, -1), 'S'), ((1, -1), 'M'), ((-1, 1), 'S')],
            ];


            for pattern in patterns.iter() {
                if pattern.iter().all(|&(offset, letter)| {
                    grid.get(&(position.0 + offset.0, position.1 + offset.1)) == Some(&letter)
                }) {
                    count += 1;
                    break;
                }
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "MMMSXXMASM
                       MSAMXMSMSA
                       AMXSXMAAMM
                       MSAMASMSMX
                       XMASAMXAMM
                       XXAMMXXAMA
                       SMSMSASXSS
                       SAXAMASAAA
                       MAMMMXMMMM
                       MXMXAXMASX";
        let hashmap = make_hashmap(input);
        assert_eq!(part1(&hashmap, XMAS), 18);
    }

    #[test]
    fn test_part2() {
        let input = "MMMSXXMASM
                       MSAMXMSMSA
                       AMXSXMAAMM
                       MSAMASMSMX
                       XMASAMXAMM
                       XXAMMXXAMA
                       SMSMSASXSS
                       SAXAMASAAA
                       MAMMMXMMMM
                       MXMXAXMASX";
        let hashmap = make_hashmap(input);
        assert_eq!(part2(&hashmap), 9);
    }
}