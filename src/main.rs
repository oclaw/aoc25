mod day;
mod day1;

use std::env;


fn main() {
    let days: Vec<Box<dyn day::Day>> = vec![
        Box::new(day1::solution::Day {}),
    ];

    let args: Vec<String> = env::args().into_iter().skip(1).collect();
    
    match args.len() {
        1 => {
            match args[0].parse::<u32>() {
                Ok(day_num) if day_num > 0 && (day_num as usize) <= days.len() => {
                    let day = &days[(day_num - 1) as usize];

                    const EXAMPLE_TYPE: &str = "example";
                    const TASK_TYPE: &str = "task";

                    solve(day.as_ref(), format!("tasks/day{}", day_num).as_str(), EXAMPLE_TYPE).expect("failed to solve example");
                    solve(day.as_ref(), format!("tasks/day{}", day_num).as_str(), TASK_TYPE).expect("failed to solve task");

                }
                _ => {
                    println!("Invalid day number");
                    std::process::exit(1);
                }
            }
        }
        _ => {
            println!("Usage: {} <day>", args[0]);
            std::process::exit(1);
        }
    }
}

fn solve(day: &dyn day::Day, path: &str, task_type: &str) -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(format!("{}/{}.txt", path, task_type))?;

    println!("Part 1 {} Solution: {}", task_type, day.part1(&input)?);
    println!("Part 2 {} Solution: {}", task_type, day.part2(&input)?);

    Ok(())
}
