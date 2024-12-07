use std::fs;

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;

fn main() {
    println!("Day 1, Part 1: {}", day_1::part_1("src/day_1/input.txt"));
    println!("Day 1, Part 2: {}", day_1::part_2("src/day_1/input.txt"));
    println!();
    println!("Day 2, Part 1: {}", day_2::part_1("src/day_2/input.txt"));
    println!("Day 2, Part 2: {}", day_2::part_2("src/day_2/input.txt"));
    println!();
    println!("Day 3, Part 1: {}", day_3::part_1("src/day_3/input.txt"));
    println!("Day 3, Part 2: {}", day_3::part_2("src/day_3/input.txt"));
    println!();
    println!("Day 4, Part 1: {}", day_4::part_1("src/day_4/input.txt"));
    println!("Day 4, Part 2: {}", day_4::part_2("src/day_4/input.txt"));
    println!();
    println!("Day 5, Part 1: {}", day_5::part_1("src/day_5/input.txt"));
    println!("Day 5, Part 2: {}", day_5::part_2("src/day_5/input.txt"));
    println!();
    println!("Day 6, Part 1: {}", day_6::part_1("src/day_6/input.txt"));
    println!("Day 6, Part 2: {}", day_6::part_2("src/day_6/input.txt"));
    println!();
    println!("Day 7, Part 1: {}", day_7::part_1(read_file("src/day_7/input.txt")));
    println!("Day 7, Part 2: {}", day_7::part_2(read_file("src/day_7/input.txt")));
}

pub fn read_file(file_path: &str) -> String {
    fs::read_to_string(file_path).expect(format!("Failed to read file: {}", file_path).as_str())
}
