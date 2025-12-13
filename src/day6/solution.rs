use crate::day;


pub struct Day {
}

impl day::Day for Day {
    fn part1(&self, input: &str) -> Result<String, Box<dyn std::error::Error>> {
        
        // all lines except the last contain numbers
        let numbers = input
            .lines()
            .take_while(|line| line.chars().skip_while(|c| c.is_whitespace()).next().unwrap().is_digit(10))
            .map(|line| line.split_whitespace().map(|num| num.parse::<u64>().unwrap()).collect::<Vec<u64>>())
            .collect::<Vec<Vec<u64>>>();

        // last line contains the operations
        let operations = input.lines().rev().next().unwrap()
            .split_whitespace()
            .map(|op| op.chars().next().unwrap())
            .collect::<Vec<char>>();

        let mut result: u64 = 0;

        for column in 0..numbers[0].len() {
            let op = operations[column];
            let mut column_result = numbers[0][column];

            for row in 1..numbers.len() {
                let num = numbers[row][column];
                // println!("Applying operation {} to {} and {}", op, column_result, num);
                match op {
                    '+' => {
                        column_result += num;
                    }
                    '*' => {
                        column_result *= num;
                    }
                    _ => panic!("unknown operation"),
                }
            }
            result += column_result;
            // println!("Column {} result: {}", column + 1, column_result);
        }

        Ok(result.to_string())

    }

    fn part2(&self, input: &str) -> Result<String, Box<dyn std::error::Error>> {

        let mut chars = input
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        chars = chars[0..chars.len()-1].to_vec();

        // last line contains the operations
        let operations = input.lines().rev().next().unwrap()
            .split_whitespace()
            .map(|op| op.chars().next().unwrap())
            .collect::<Vec<char>>();

        let mut result: u64 = 0;

        let mut problem_sum: u64;
        match operations[operations.len() - 1] {
            '+' => problem_sum = 0,
            '*' => problem_sum = 1,
            _ => panic!("unknown operation"),
        }

        let mut op_idx = operations.len() - 1; 

        for column in (0..chars[0].len()).rev() {
        
            let mut new_problem_started = false;
            let mut row_idx:  usize = 0;
            while row_idx < chars.len() && chars[row_idx][column].is_whitespace(){
                row_idx += 1;
            }
            
            if row_idx >= chars.len() {
                new_problem_started = true;
            }

            if new_problem_started {
                result += problem_sum;
                op_idx -= 1;
                match operations[op_idx] {
                    '+' => problem_sum = 0,
                    '*' => problem_sum = 1,
                    _ => panic!("unknown operation"),
                }
                continue; // to the next column
            }
 
            let mut val: u64 = 0;

            let mut i = chars.len() as i32 - 1;
            let mut ten_power = 0;
            while i >= 0 {
                let idx = i as usize;
                if !chars[idx][column].is_whitespace() {
                    let new_digit= chars[idx][column].to_digit(10).unwrap() as u64;
                    val = new_digit * 10_u64.pow(ten_power) + val;
                    ten_power += 1;
                }
                i -= 1;
            }

            let op = operations[op_idx];
            match op {
                '+' => problem_sum = problem_sum + val, 
                '*' => problem_sum = problem_sum * val,
                _ => panic!("unknown operation"),                    
            }
        }

        result += problem_sum;


        Ok(result.to_string())


    }
}