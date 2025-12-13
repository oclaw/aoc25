use std::error::Error;

pub trait Day {
    fn part1(&self, input: &str) -> Result<String, Box<dyn Error>>;
    fn part2(&self, input: &str) -> Result<String, Box<dyn Error>>;
}



// template for new day solution files:

// use crate::day;


// pub struct Day {
// }

// impl day::Day for Day {
//     fn part1(&self, input: &str) -> Result<String, Box<dyn std::error::Error>> {
//         Ok(String::from("not implemented"))
//     }

//     fn part2(&self, input: &str) -> Result<String, Box<dyn std::error::Error>> {
//         Ok(String::from("not implemented"))
//     }
// }