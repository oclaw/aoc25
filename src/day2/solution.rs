use crate::day;


pub struct Day {
}

impl day::Day for Day {
    fn part1(&self, input: &str) -> Result<String, Box<dyn std::error::Error>> {
        

        type BoxedError = Box<dyn std::error::Error>;

        let result: u64 = 0;

        input.split(",").into_iter().try_for_each(|line| -> Result<(), BoxedError> {

            let parts = line.split("-").collect::<Vec<&str>>();
            if parts.len() != 2 {
                return Err("invalid line format".into());
            }

            let start = parts[0].parse::<i32>()?;
            let end = parts[1].parse::<i32>()?;

            let start_chars = parts[0];

            let mut pattern_len = 1;
            while pattern_len <=  {
                pattern = &start_chars[...pattern_len];
                
            }

            Ok(())
            
        })?;

        Ok(String::from("not implemented"))
    }

    fn part2(&self, input: &str) -> Result<String, Box<dyn std::error::Error>> {
        Ok(String::from("not implemented"))
    }
}