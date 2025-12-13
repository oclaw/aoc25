use crate::day;


pub struct Day {
}

impl day::Day for Day {
    fn part1(&self, input: &str) -> Result<String, Box<dyn std::error::Error>> {

        let mut result: u64 = 0;
        input.split("\n").into_iter().for_each(|line| {
            let mut max_joltage: u32 = 0; 

            let mut offset = 0;
            while offset < line.len() - 1 {
                let first_digit = line.chars().nth(offset).unwrap().to_digit(10).unwrap();
                for look_ahead in offset+1..line.len() {
                    let next_digit = line.chars().nth(look_ahead).unwrap().to_digit(10).unwrap();
                    let joltage = first_digit * 10 + next_digit;
                    if joltage > max_joltage {
                        max_joltage = joltage;
                    }
                 }
                offset += 1;
            }
            // println!("Max joltage in line '{}' is {}", line, max_joltage);
            
            result += max_joltage as u64;
        });


        Ok(String::from(result.to_string()))
    }

    fn part2(&self, input: &str) -> Result<String, Box<dyn std::error::Error>> {

        const JOLTAGE_LEN: u32  = 12;

        fn compute_joltage(digits: &str, start: u32, depth: u32, acc: u64) -> u64 {
            
            if depth == JOLTAGE_LEN {
                return acc;
            }

            let rest_to_collect = JOLTAGE_LEN - depth;


            let mut next_max_digit = 0;
            let max_next_digit_index = digits.len() as u32 - rest_to_collect + 1;

            for look_ahead in  start..max_next_digit_index {
                let digit = digits.chars().nth(look_ahead as usize).unwrap().to_digit(10).unwrap();
                if digit > next_max_digit {
                    next_max_digit = digit;
                }
            }

            let mut max_joltage: u64 = 0;
            for idx in start..digits.len() as u32 {
                let digit = digits.chars().nth(idx as usize).unwrap().to_digit(10).unwrap();
                if digit != next_max_digit {
                    continue;
                }

                let joltage_starting_from_this = compute_joltage(digits, idx + 1, depth + 1, acc * 10 + digit as u64);
                max_joltage = std::cmp::max(max_joltage, joltage_starting_from_this);
            }

            return max_joltage;

        }

        let mut result: u64 = 0;
        input.split("\n").into_iter().for_each(|line| {
            let jolage = compute_joltage(line, 0, 0, 0);
            // println!("Max joltage in line '{}' is {}", line, jolage);
            result += jolage;
        });


        Ok(String::from(result.to_string()))

    }
}