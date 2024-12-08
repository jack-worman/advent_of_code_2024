use std::collections::HashMap;

pub fn part_1(input: String) -> u32 {
    let mut map = Vec::new();
    let lines = input.lines();
    for line in lines {
        map.push(line.chars().collect::<Vec<_>>())
    }

    let mut antenna_sets: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    for row in 0..map.len() {
        for col in 0..map[row].len() {
            let cell = map[row][col];
            if cell == '.' {
                continue;
            }
            match antenna_sets.get_mut(&cell) {
                Some(antennas) => {
                    antennas.push((row, col));
                },
                None => {
                    antenna_sets.insert(cell, vec![(row, col)]);
                },
            };
        }
    }

    let mut antinodes: Vec<(i32, i32)> = Vec::new();
    for (_, antennas) in antenna_sets {
        for i in 0..(antennas.len() - 1) {
            for j in (i + 1)..antennas.len() {
                let a = (antennas[i].0 as i32, antennas[i].1 as i32);
                let b = (antennas[j].0 as i32, antennas[j].1 as i32);
                let rise: i32 = b.0 - a.0;
                let run: i32 = b.1 - a.1;

                let node_1 = (a.0 - rise, a.1 - run);
                if node_1.0 >= 0 && node_1.0 < map.len() as i32 && node_1.1 >= 0 && node_1.1 < map[0].len() as i32 {
                    antinodes.push(node_1);
                }

                let node_2 = (b.0 + rise, b.1 + run);
                if node_2.0 >= 0 && node_2.0 < map.len() as i32 && node_2.1 >= 0 && node_2.1 < map[0].len() as i32 {
                    antinodes.push(node_2);
                }
            }
        }
    }
    antinodes.sort();
    antinodes.dedup();

    antinodes.len() as u32
}

pub fn part_2(input: String) -> u32 {
    let mut map = Vec::new();
    let lines = input.lines();
    for line in lines {
        map.push(line.chars().collect::<Vec<_>>())
    }

    let mut antenna_sets: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    for row in 0..map.len() {
        for col in 0..map[row].len() {
            let cell = map[row][col];
            if cell == '.' {
                continue;
            }
            match antenna_sets.get_mut(&cell) {
                Some(antennas) => {
                    antennas.push((row, col));
                },
                None => {
                    antenna_sets.insert(cell, vec![(row, col)]);
                },
            };
        }
    }

    let mut antinodes: Vec<(i32, i32)> = Vec::new();
    for (_, antennas) in antenna_sets {
        for i in 0..(antennas.len() - 1) {
            for j in (i + 1)..antennas.len() {
                let a = (antennas[i].0 as i32, antennas[i].1 as i32);
                antinodes.push(a);
                let b = (antennas[j].0 as i32, antennas[j].1 as i32);
                antinodes.push(b);
                let rise: i32 = b.0 - a.0;
                let run: i32 = b.1 - a.1;

                let mut count = 1;
                loop {
                    let node_1 = (a.0 - (rise * count), a.1 - (run * count));
                    if node_1.0 >= 0 && node_1.0 < map.len() as i32 && node_1.1 >= 0 && node_1.1 < map[0].len() as i32 {
                        antinodes.push(node_1);
                        count += 1;
                    } else {
                        break;
                    }
                }

                let mut count = 1;
                loop {
                    let node_2 = (b.0 + (rise * count), b.1 + (run * count));
                    if node_2.0 >= 0 && node_2.0 < map.len() as i32 && node_2.1 >= 0 && node_2.1 < map[0].len() as i32 {
                        antinodes.push(node_2);
                        count += 1;
                    } else {
                        break;
                    }
                }
            }
        }
    }
    antinodes.sort();
    antinodes.dedup();

    antinodes.len() as u32
}

#[cfg(test)]
mod tests {
    use crate::*;
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(read_file("src/day_8/example.txt")), 14);
        assert_eq!(part_1(read_file("src/day_8/input.txt")), 301);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(read_file("src/day_8/example.txt")), 34);
        assert_eq!(part_2(read_file("src/day_8/input.txt")), 1019);
    }
}
