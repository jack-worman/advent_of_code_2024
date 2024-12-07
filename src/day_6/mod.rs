use std::fs;
use std::thread;

#[derive(PartialEq, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

pub fn part_1(file_path: &str) -> u32 {
    let contents = fs::read_to_string(file_path).expect("Failed to read file.");
    let map = parse_input(contents);
    let (walked, _) = run_map(&map);
    walked.len() as u32
}

pub fn part_2(file_path: &str) -> u32 {
    let contents = fs::read_to_string(file_path).expect("Failed to read file.");

    let map = parse_input(contents);
    let (walked, _) = run_map(&map);

    let mut handles = Vec::new();

    for cell in walked {
        if map[cell.0][cell.1] != '.' {
            continue;
        }
        let mut cloned_map = map.clone();
        cloned_map[cell.0][cell.1] = '#';
        let cloned_map = cloned_map;
        let handle = thread::spawn(move || {
            let (_, is_loop) = run_map(&cloned_map);
            is_loop
        });
        handles.push(handle);
    }

    let mut loops = 0;
    for handle in handles {
        if handle.join().unwrap() {
            loops += 1;
        }
    }
    loops
}

fn parse_input(input: String) -> Vec<Vec<char>> {
    let lines = input.lines();
    let mut map = Vec::new();
    lines.for_each(|line| map.push(line.chars().collect()));
    map
}

fn get_guard(map: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    for row in 0..map.len() {
        for col in 0..map[row].len() {
            let cell = map[row][col];
            if cell == '^' || cell == '>' || cell == 'v' || cell == '<' {
                return Some((row, col));
            }
        }
    }
    None
}

fn run_map(map: &Vec<Vec<char>>) -> (Vec<(usize, usize)>, bool) {
    let mut unique_cells = Vec::new();
    let mut walked_cells = Vec::new();
    let mut guard = get_guard(map).unwrap();
    let mut direction = match map[guard.0][guard.1] {
        '^' => Direction::Up,
        '>' => Direction::Right,
        'v' => Direction::Down,
        '<' => Direction::Left,
        _ => panic!("Unexpected direction."),
    };
    walked_cells.push((guard, direction.clone()));

    loop {
        let velocity = match direction {
            Direction::Up => (-1, 0),
            Direction::Right =>  (0, 1),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
        };

        let next_row = guard.0 as i32 + velocity.0;
        let next_col = guard.1 as i32 + velocity.1;
        if next_row < 0 || next_row >= map.len() as i32 || next_col < 0 || next_col >= map[0].len() as i32 {
            return (unique_cells, false);
        }
        let next_row = next_row as usize;
        let next_col = next_col as usize;

        let next_cell = map[next_row][next_col];
        if next_cell == '#' {
            direction = match direction {
                Direction::Up => Direction::Right,
                Direction::Right =>  Direction::Down,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
            };
        } else {
            guard = (next_row, next_col);
            if walked_cells.contains(&(guard, direction.clone())) {
                return (unique_cells, true);
            }
            if !unique_cells.contains(&guard) {
                unique_cells.push(guard);
            }
            walked_cells.push((guard, direction.clone()));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1("src/day_6/example.txt"), 41);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2("src/day_6/example.txt"), 6);
    }
}
