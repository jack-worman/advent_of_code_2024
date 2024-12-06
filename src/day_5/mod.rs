use std::cmp::Ordering;
use std::fs;

pub fn part_1(file_path: &str) -> u32 {
    let contents = fs::read_to_string(file_path).expect("Failed to read file.");

    let (rules, updates) = parse_input(contents);

    let mut valid_update_middles_sum: u32 = 0;
    for update in &updates {
        let sorted_update = sort_update(update, &rules);
        if sorted_update.eq(update) {
            valid_update_middles_sum += sorted_update[sorted_update.len() / 2] as u32;
        }
    }

    valid_update_middles_sum
}

pub fn part_2(file_path: &str) -> u32 {
    let contents = fs::read_to_string(file_path).expect("Failed to read file.");

    let (rules, updates) = parse_input(contents);

    let mut invalid_update_middles_sum: u32 = 0;
    for update in &updates {
        let sorted_update = sort_update(update, &rules);
        if !sorted_update.eq(update) {
            invalid_update_middles_sum += sorted_update[sorted_update.len() / 2] as u32;
        }
    }

    invalid_update_middles_sum
}

fn parse_input(contents: String) -> (Vec<(u8, u8)>, Vec<Vec<u8>>)
{
    let lines = contents.lines().collect::<Vec<&str>>();

    let mut after_blank_line = false;
    let mut rules: Vec<(u8, u8)> = Vec::new();
    let mut updates: Vec<Vec<u8>> = Vec::new();
    for line in lines {
        if line == "" {
            after_blank_line = true;
            continue;
        }

        if after_blank_line {
            let numbers = line.split(",").map(|x| x.parse::<u8>().unwrap()).collect::<Vec<_>>();
            updates.push(numbers);
        } else {
            let numbers = line.split("|").map(|x| x.parse::<u8>().unwrap()).collect::<Vec<_>>();
            rules.push((numbers[0], numbers[1]));
        }
    }
    (rules, updates)
}

fn sort_update(update: &Vec<u8>, rules: &Vec<(u8, u8)>) -> Vec<u8>
{
    let mut sorted_update = update.clone();
    sorted_update.sort_by(|x1, x2| {
        for rule in rules {
            if *rule == (*x1, *x2) {
                return Ordering::Less;
            }
            if *rule == (*x2, *x1) {
                return Ordering::Greater;
            }
        }
        Ordering::Equal
    });
    sorted_update
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1("src/day_5/example.txt"), 143);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2("src/day_5/example.txt"), 123);
    }
}
