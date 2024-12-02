use std::cmp::PartialEq;
use std::fs;

#[derive(PartialEq)]
enum Direction {
    Increasing,
    Decreasing,
}

pub fn part_1(file_path: &str) -> u32 {
    let contents = fs::read_to_string(file_path).expect("Failed to read file.");
    let lines = contents.lines().collect::<Vec<_>>();
    let mut valid_lines = 0;
    for line in lines.iter() {
        let numbers: Vec<_> = line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
        if is_line_valid(&numbers) {
            valid_lines += 1;
        }
    }
    valid_lines
}

pub fn part_2(file_path: &str) -> u32 {
    let contents = fs::read_to_string(file_path).expect("Failed to read file.");
    let lines = contents.lines().collect::<Vec<_>>();
    let mut valid_lines = 0;
    'lines: for line in lines.iter() {
        let numbers: Vec<_> = line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
        if is_line_valid(&numbers) {
            valid_lines += 1;
            continue 'lines;
        }

        for i in 0..numbers.len() {
            let mut numbers_2 = numbers.clone();
            numbers_2.remove(i);
            if is_line_valid(&numbers_2) {
                valid_lines += 1;
                continue 'lines;
            }
        }
    }
    valid_lines
}

fn is_line_valid(numbers: &Vec<i32>) -> bool {
    let mut valid = true;
    let mut direction: Option<Direction> = None;
    for i in 0..numbers.len() - 1 {
        let abs_diff = numbers[i].abs_diff(numbers[i + 1]);
        if abs_diff < 1 || abs_diff > 3 {
            valid = false;
            break;
        }

        let diff = numbers[i] - numbers[i + 1];
        match direction {
            Some(Direction::Increasing) => {
                if diff < 0 {
                    valid = false;
                    break;
                }
            }
            Some(Direction::Decreasing) => {
                if diff > 0 {
                    valid = false;
                    break;
                }}
            None => {
                if diff < 0 {
                    direction = Some(Direction::Decreasing);
                } else if diff > 0 {
                    direction = Some(Direction::Increasing);
                }
            }
        }
    }
    valid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1("src/day_2/example.txt"), 2);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2("src/day_2/example.txt"), 4);
    }
}