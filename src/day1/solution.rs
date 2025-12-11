
use crate::day;


pub struct Day {
}

impl day::Day for Day {
    fn part1(&self, input: &str) -> Result<String, Box<dyn std::error::Error>> {
        let mut pos: i32 = 50;
        let mut password: i32 = 0;
        const CIRCLE_SIZE:i32 = 100;

        input.split("\n").into_iter().for_each(|line| {
            let direction: &str = &line[..1];
            let mut shift_value: i32 = line[1..].parse().expect("failed to parse number"); // TODO: handle error
            shift_value = shift_value % (CIRCLE_SIZE as i32);
            

            let sign: i32 = match direction {
                "L" => -1,
                "R" => 1,
                _ => panic!("invalid direction") // TODO: handle error
            };

            pos += sign * shift_value;

            if pos < 0 {
                pos += CIRCLE_SIZE;
            } else if pos >= CIRCLE_SIZE {
                pos -= CIRCLE_SIZE;
            }
            if pos == 0 {
                password += 1;
            }
        });

        Ok(String::from(format!("{}", password)))
    }

    fn part2(&self, input: &str) -> Result<String, Box<dyn std::error::Error>> {


        let mut pos: i32 = 50;
        let mut password: i32 = 0;
        const CIRCLE_SIZE:i32 = 100;

        input.split("\n").into_iter().for_each(|line| {
            let direction: &str = &line[..1];
            let mut shift_value: i32 = line[1..].parse().expect("failed to parse number"); // TODO: handle error
            let full_shifts = shift_value / CIRCLE_SIZE;
            shift_value = shift_value % (CIRCLE_SIZE as i32);
            
            let prev_pos = pos;
            let sign: i32 = match direction {
                "L" => -1,
                "R" => 1,
                _ => panic!("invalid direction") // TODO: handle error
            };

            pos += sign * shift_value;

            password += full_shifts;

            if pos >= CIRCLE_SIZE {
                pos -= CIRCLE_SIZE;
            }

            if pos < 0 {
                pos += CIRCLE_SIZE;
            }

            if prev_pos == 0 {
                return; // no more shifts to count
            }

            if pos == 0 {
                password += 1; // arrived at zero
                return;
            }

            let mut crossed_zero: bool = false;
            if sign > 0 {
                crossed_zero = pos < prev_pos;
            } else {
                crossed_zero = pos > prev_pos;
            }
            if crossed_zero {
                password += 1;
            } 
        });

        Ok(String::from(format!("{}", password)))

    }
}