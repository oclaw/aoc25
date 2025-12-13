use crate::day;


pub struct Day {
}

const PAPER_ROLL: char = '@';
const EMPTY_SPACE: char = '.';
// const REACHABLE_SPACE: char = 'x';

fn get_adjacent_positions(i: usize, j: usize) -> [(i32, i32); 8] {
    [
        (i as i32 - 1, j as i32),     // up
        (i as i32 - 1, j as i32 - 1), // up, left
        (i as i32 - 1, j as i32 + 1), // up, right
        (i as i32 + 1, j as i32),     // down
        (i as i32 + 1, j as i32 - 1), // down, left
        (i as i32 + 1, j as i32 + 1), // down, right
        (i as i32, j as i32 - 1),     // left
        (i as i32, j as i32 + 1),     // right
    ]
}

impl day::Day for Day {

    fn part1(&self, input: &str) -> Result<String, Box<dyn std::error::Error>> {
        let grid = input.split("\n").collect::<Vec<&str>>();
        let mut result: u32 = 0;
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {

                let _cell = grid[i].chars().nth(j).unwrap();
                if _cell != PAPER_ROLL {
                    continue;
                }

                let mut adjacent_rolls: u32 = 0;
                let adjacent_positions = get_adjacent_positions(i, j);

                adjacent_positions.iter().for_each(|pos| {
                    if pos.0 < 0 || pos.0 >= grid.len() as i32 {
                        return;
                    }
                    if pos.1 < 0 || pos.1 >= grid[pos.0 as usize].len() as i32 {
                        return;
                    }
                    let adjacent_cell = grid[pos.0 as usize].chars().nth(pos.1 as usize).unwrap();
                    if adjacent_cell == PAPER_ROLL {
                        adjacent_rolls += 1;
                    }
                });

                if adjacent_rolls < 4 {
                    result += 1;
                }
            }
        }
        
        Ok(result.to_string())
    }


    fn part2(&self, input: &str) -> Result<String, Box<dyn std::error::Error>> {
        
        let grid = input.split("\n").collect::<Vec<&str>>();
        
        fn compute_weight(grid: &Vec<&str>, i: usize, j: usize) -> i32 {
            let mut weight: i32 = 0;
            let adjacent_positions = get_adjacent_positions(i, j);

            adjacent_positions.iter().for_each(|pos| {
                if pos.0 < 0 || pos.0 >= grid.len() as i32 {
                    return;
                }
                if pos.1 < 0 || pos.1 >= grid[pos.0 as usize].len() as i32 {
                    return;
                }
                let adjacent_cell = grid[pos.0 as usize].chars().nth(pos.1 as usize).unwrap();
                if adjacent_cell == PAPER_ROLL {
                    weight += 1;
                }
            });

            weight
        }

        let mut weights: Vec<Vec<i32>> = grid.iter().enumerate().map(|(i, line)| {
            line.chars().enumerate().map(|(j, ch)|   {
                match ch {
                    PAPER_ROLL => compute_weight(&grid, i, j),
                    EMPTY_SPACE => -1,
                    _ => panic!("invalid cell character"),
                }
            }).collect::<Vec<i32>>()
        }).collect::<Vec<Vec<i32>>>();

        fn remove(weights: &mut Vec<Vec<i32>>, i: usize, j: usize) {

            weights[i][j] = -1;

            let adjacent_positions = get_adjacent_positions(i, j);

            adjacent_positions.iter().for_each(|pos| {
                if pos.0 < 0 || pos.0 >= weights.len() as i32 {
                    return;
                }
                if pos.1 < 0 || pos.1 >= weights[pos.0 as usize].len() as i32 {
                    return;
                }
                if weights[pos.0 as usize][pos.1 as usize] > 0 {
                    weights[pos.0 as usize][pos.1 as usize] -= 1;
                }
            });
        }


        let mut result = 0;
        loop {
            let mut removed = 0;
            for i in 0..grid.len() {
                for j in 0..grid[i].len() {
                    let cell = weights[i][j];
                    if cell < 0 {
                        continue;
                    }

                    if weights[i][j] < 4 {
                        remove(&mut weights, i, j);
                        removed += 1;
                    }
                }
            }
            result += removed;
            if removed == 0 {
                break;
            }
        }

        Ok(result.to_string())
    }
}