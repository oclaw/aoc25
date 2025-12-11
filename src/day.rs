use std::error::Error;

pub trait Day {
    fn part1(&self, input: &str) -> Result<String, Box<dyn Error>>;
    fn part2(&self, input: &str) -> Result<String, Box<dyn Error>>;
}