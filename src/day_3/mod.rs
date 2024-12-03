use std::fs;
use regex::Regex;

pub fn part_1(file_path: &str) -> i32 {
    let contents = fs::read_to_string(file_path).expect("Failed to read file.");
    let regex = Regex::new(r"mul\((?<left>\d+),(?<right>\d+)\)").unwrap();
    let captures = regex.captures_iter(contents.as_str());
    let mut sum = 0;
    for (_, [left, right]) in captures.map(|c| c.extract()) {
        sum += left.parse::<i32>().unwrap() * right.parse::<i32>().unwrap();
    }
    sum
}

pub fn part_2(file_path: &str) -> i32 {
    let contents = fs::read_to_string(file_path).expect("Failed to read file.");
    // . does not match newlines
    let contents = contents.replace("\n", " ");

    // try iterating over don't() do() and mul()
    let regex_dont_do = Regex::new(r"don't\(\).*?(?:do\(\)|$)").unwrap();
    let contents = regex_dont_do.replace_all(contents.as_str(), " ");

    let regex = Regex::new(r"mul\((?<left>\d{1,3}),(?<right>\d{1,3})\)").unwrap();
    let captures = regex.captures_iter(&*contents);
    let mut sum = 0;
    for (_, [left, right]) in captures.map(|c| c.extract()) {
        sum += left.parse::<i32>().unwrap() * right.parse::<i32>().unwrap();
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1("src/day_3/example.txt"), 161);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2("src/day_3/example_2.txt"), 48);
    }
}