use std::fs;
use std::iter::zip;

pub fn part_1(file_path: &str) -> u32 {
    let contents = fs::read_to_string(file_path).expect("Failed to read file.");
    let lines = contents.lines().collect::<Vec<_>>();
    let mut left_list = Vec::new();
    let mut right_list = Vec::new();
    for line in lines {
        let line_parts = line.split_whitespace().collect::<Vec<_>>();
        left_list.push(line_parts[0].parse::<u32>().unwrap());
        right_list.push(line_parts[1].parse::<u32>().unwrap());
    }
    left_list.sort();
    right_list.sort();
    let sorted_pairs = zip(left_list, right_list);
    let mut distance = 0;
    for pair in sorted_pairs {
        distance += pair.0.abs_diff(pair.1);
    }
    distance
}

pub fn part_2(file_path: &str) -> u32 {
    let contents = fs::read_to_string(file_path).expect("Failed to read file.");
    let lines = contents.lines().collect::<Vec<_>>();
    let mut left_list = Vec::new();
    let mut right_list = Vec::new();
    for line in lines {
        let line_parts = line.split_whitespace().collect::<Vec<_>>();
        left_list.push(line_parts[0].parse::<u32>().unwrap());
        right_list.push(line_parts[1].parse::<u32>().unwrap());
    }
    let mut similarity = 0;
    for left in left_list {
        let mut count = 0;
        for right in &right_list {
            if left == *right {
                count += 1;
            }
        }
        similarity += left * count;
    }
    similarity
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1("src/day_1/example.txt"), 11);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2("src/day_1/example.txt"), 31);
    }
}