mod day_1;
mod day_2;
mod day_3;
mod day_4;

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
}
