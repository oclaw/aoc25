use std::collections::HashSet;

use crate::day;


pub struct Day {
}

fn solve(input: &str, max_parts: Option<u32>) -> Result<String, Box<dyn std::error::Error>> {
    type BoxedError = Box<dyn std::error::Error>;

    let mut result: u64 = 0;

    input.split(",").into_iter().try_for_each(|line| -> Result<(), BoxedError> {

        let parts = line.split("-").collect::<Vec<&str>>();
        if parts.len() != 2 {
            return Err("invalid line format".into());
        }

        let start = parts[0].parse::<u64>()?;
        let end = parts[1].parse::<u64>()?;

        let mut set = HashSet::<u64>::new();
        let mut cur = start;
        while cur <= end {

            let mut prev_pattern: u64= 0;
            for pattern_len in 1..= (parts[1].len() as u64) / 2 {
                let step = 10_u64.pow(pattern_len as u32);
                let mut acc = cur % step;
                let mut left = cur / step;
                let pattern = acc;

                if pattern == prev_pattern {
                    continue;
                }
                prev_pattern = pattern;

                let mut parts_count = 1;
                while left != 0 &&  parts_count < max_parts.unwrap_or(u32::MAX) {
                    let part = left % step;
                    if part != pattern {
                        break;
                    }
                    acc = acc * step + part;
                    left /= step;
                    parts_count += 1;
                }


                if left == 0 && acc > 10 && !set.contains(&acc) {
                    // println!("Found pattern value: {}", acc);
                    result += acc as u64;
                    set.insert(acc);
                }
            }                

            cur += 1;
        }

        Ok(())
        
    })?;

    Ok(result.to_string())
}


impl day::Day for Day {
    fn part1(&self, input: &str) -> Result<String, Box<dyn std::error::Error>> {
        solve(input, Option::from(2))
    }

    fn part2(&self, input: &str) -> Result<String, Box<dyn std::error::Error>> {
        solve(input, Option::None)
    }
}