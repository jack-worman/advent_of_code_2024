use std::fs;

pub fn part_1(file_path: &str) -> i32 {
    let contents = fs::read_to_string(file_path).expect("Failed to read file.");
    let lines = contents.lines().collect::<Vec<&str>>();
    let mut chars = Vec::new();
    for line in lines {
        chars.push(line.chars().collect::<Vec<char>>())
    }

    let mut directions: Vec<(i32, i32)> = Vec::new();
    for i in -1..=1 {
        for j in -1..=1 {
            directions.push((i, j))
        }
    }

    let mut found_xmas_count = 0;
    for row in 0..chars.len() {
        for col in 0..chars[0].len() {
            for direction in &directions {
                let mut found_xmas = true;

                let xmas_chars = vec!['X', 'M', 'A', 'S'];
                for i in 0..4 {
                    let check_row = (row as i32) + (direction.0 * i);
                    if check_row < 0 || check_row >= chars.len() as i32 {
                        found_xmas = false;
                        break;
                    }

                    let check_col = (col as i32) + (direction.1 * i);
                    if check_col < 0 || check_col >= chars[0].len() as i32 {
                        found_xmas = false;
                        break;
                    }

                    if chars[check_row as usize][check_col as usize] != xmas_chars[i as usize] {
                        found_xmas = false;
                    }
                }
                if found_xmas {
                    found_xmas_count += 1;
                }
            }
        }
    }

    found_xmas_count
}

pub fn part_2(file_path: &str) -> i32 {
    let contents = fs::read_to_string(file_path).expect("Failed to read file.");
    let lines = contents.lines().collect::<Vec<&str>>();
    let mut chars = Vec::new();
    for line in lines {
        chars.push(line.chars().collect::<Vec<char>>())
    }

    let mut directions: Vec<(i32, i32)> = Vec::new();
    for i in -1..=1 {
        for j in -1..=1 {
            directions.push((i, j))
        }
    }

    let mut found_crosses = 0;
    for row in 1..chars.len() - 1 {
        for col in 1..chars[0].len() - 1 {
            if chars[row][col] != 'A' {
                continue;
            }
            if
                (
                    (chars[row - 1][col - 1] == 'M' && chars[row + 1][col + 1] == 'S')
                    || (chars[row - 1][col - 1] == 'S' && chars[row + 1][col + 1] == 'M')
                )
                && (
                    (chars[row - 1][col + 1] == 'M' && chars[row + 1][col - 1] == 'S')
                    || (chars[row - 1][col + 1] == 'S' && chars[row + 1][col - 1] == 'M')
                )
            {
                found_crosses += 1;
            }
        }
    }

    found_crosses
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1("src/day_4/example.txt"), 18);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2("src/day_4/example.txt"), 9);
    }
}
