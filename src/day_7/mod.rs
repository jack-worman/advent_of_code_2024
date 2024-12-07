use std::thread;

pub fn part_1(input: String) -> u64 {
    let mut total_calibration_result = 0;
    let equations = get_equations(input);
    'equations: for (test_value, numbers) in equations {
        let mut operations = vec!['+'; numbers.len() - 1];
        loop {
            let mut result = numbers[0];
            for i in 0..operations.len() {
                match operations[i] {
                    '+' => result += numbers[i + 1],
                    '*' => result *= numbers[i + 1],
                    _ => panic!("Unexpected operation: {}", operations[i]),
                }
            }
            if result == test_value {
                total_calibration_result += test_value;
                continue 'equations;
            }

            if operations.iter().all(|x| *x == '*') {
                continue 'equations;
            }
            let mut carry = true;
            for i in 0..operations.len() {
                if !carry {
                    break;
                }
                match operations[i] {
                    '+' => {
                        operations[i] = '*';
                        carry = false;
                    },
                    '*' => {
                        operations[i] = '+';
                        carry = true;
                    },
                    _ => panic!("Unexpected operation: {}", operations[i]),
                }
            }
        }
    }
    total_calibration_result
}

pub fn part_2(input: String) -> u64 {
    let equations = get_equations(input);
    let mut handles = Vec::new();
    for (test_value, numbers) in equations {
        let handle = thread::spawn(move || {
            if check_equation(test_value, &numbers) {
                test_value
            } else {
                0
            }
        });
        handles.push(handle);
    }
    let mut total_calibration_result = 0;
    for handle in handles {
        total_calibration_result += handle.join().unwrap();
    }
    total_calibration_result
}

fn get_equations(input: String) -> Vec<(u64, Vec<u64>)> {
    let mut equations = Vec::new();
    let lines = input.lines();
    for line in lines {
        let parts = line.split(": ").collect::<Vec<_>>();
        let test_value = parts[0].parse::<u64>().unwrap();
        let numbers = parts[1].split_whitespace().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<_>>();
        equations.push((test_value, numbers))
    }
    equations
}

fn check_equation(test_value: u64, numbers: &Vec<u64>) -> bool {
    let mut operations = vec!['+'; numbers.len() - 1];
    for _ in 0..3_i32.pow(operations.len().try_into().unwrap()) {
        let mut result = numbers[0];
        for i in 0..operations.len() {
            match operations[i] {
                '+' => result += numbers[i + 1],
                '*' => result *= numbers[i + 1],
                '|' => {
                    let mut left = result.to_string();
                    left.push_str(&numbers[i + 1].to_string());
                    result = left.parse::<u64>().unwrap();
                },
                _ => panic!("Unexpected operation: {}", operations[i]),
            }
        }
        if result == test_value {
            return true;
        }

        let mut carry = true;
        for i in 0..operations.len() {
            if !carry {
                break;
            }
            match operations[i] {
                '+' => {
                    operations[i] = '*';
                    carry = false;
                },
                '*' => {
                    operations[i] = '|';
                    carry = false;
                },
                '|' => {
                    operations[i] = '+';
                    carry = true;
                },
                _ => panic!("Unexpected operation: {}", operations[i]),
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use crate::*;
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(read_file("src/day_7/example.txt")), 3749);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(read_file("src/day_7/example.txt")), 11387);
    }
}