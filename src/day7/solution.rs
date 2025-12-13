use crate::day;


pub struct Day {
}

const BEAM_START: char = 'S';
const BEAM_MOVE: char = '|';
const FREE_SPACE: char = '.';
const SPLITTER: char = '^';

impl day::Day for Day {
    fn part1(&self, input: &str) -> Result<String, Box<dyn std::error::Error>> {
        let mut chars = input.lines().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

        let mut total_splits = 0;
        for row in 1..chars.len() {
            for col in 0..chars[row].len() {
                let upper_cell = chars[row - 1][col];

                if upper_cell != BEAM_START && upper_cell != BEAM_MOVE {
                    continue; 
                }

                let current_cell = chars[row][col];

                match current_cell {
                    BEAM_START => {},
                    BEAM_MOVE => {},
                    FREE_SPACE => {
                        chars[row][col] = BEAM_MOVE;
                    },
                    SPLITTER => {
                        total_splits += 1;
                        chars[row][col-1] = BEAM_MOVE;
                        chars[row][col+1] = BEAM_MOVE;
                    },
                    _ => panic!("unexpected cell"),
                }
            }

        }

        Ok(total_splits.to_string())
    }


    fn part2(&self, input: &str) -> Result<String, Box<dyn std::error::Error>> {
        // return part2_naive(input);

        let mut visits = Vec::<Vec<u64>>::new();
        let mut chars = input.lines().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

        for i in 0..chars.len() {
            visits.push(Vec::<u64>::new());
            for _ in 0..chars[i].len() {
                visits[i].push(0);
            }
        }

        let mut beam_start_col = 0;
        for col in 0..chars[0].len() {
            if chars[0][col] == BEAM_START {
                beam_start_col = col;
                break;
            }
        }
        visits[0][beam_start_col] = 1;
 
        for row in 1..chars.len() {
            for col in 0..chars[row].len() {
                let upper_cell = chars[row - 1][col];

                if upper_cell != BEAM_START && upper_cell != BEAM_MOVE {
                    continue; 
                }

                let upper_cell_visits = visits[row - 1][col] as u64;
                let current_cell = chars[row][col];

                match current_cell {
                    BEAM_START => {},
                    BEAM_MOVE => {
                        chars[row][col] = BEAM_MOVE;
                        visits[row][col] += upper_cell_visits;
                    },
                    FREE_SPACE => {
                        chars[row][col] = BEAM_MOVE;
                        visits[row][col] += upper_cell_visits;
                    },
                    SPLITTER => {
                        chars[row][col-1] = BEAM_MOVE;
                        visits[row][col-1] += upper_cell_visits;


                        chars[row][col+1] = BEAM_MOVE;
                        visits[row][col+1] += upper_cell_visits;
                    },
                    _ => panic!("unexpected cell"),
                }
            }      
        }

        let result = visits.last().unwrap().iter().sum::<u64>();

        Ok(result.to_string())
    }


}

// Naive DFS solution for part 2 for verification
fn part2_naive(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let chars = input.lines().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let mut visits = Vec::<Vec<u64>>::new();
    
    for i in 0..chars.len() {
        visits.push(Vec::<u64>::new());
        for _ in 0..chars[i].len() {
            visits[i].push(0);

        }
    }

    fn particle_move(map: &Vec<Vec<char>>, visits: &mut Vec<Vec<u64>>, pos: (usize, usize), finished_paths: &mut u32) {
        if pos.0 >= (map.len() - 1) {
            *finished_paths += 1;
            return;
        }
        visits[pos.0][pos.1] += 1;
        let next_row = pos.0 + 1;
        match map[next_row][pos.1] {
            FREE_SPACE => particle_move(map, visits, (next_row, pos.1), finished_paths),
            SPLITTER => {
                particle_move(map, visits, (next_row, pos.1 - 1), finished_paths);
                particle_move(map, visits, (next_row, pos.1 + 1), finished_paths);
            }
            _ => panic!("unexpected cell during particle move"),
        }
    }

    let mut beam_start_col = 0;
    for col in 0..chars[0].len() {
        if chars[0][col] == BEAM_START {
            beam_start_col = col;
            break;
        }
    }

    let mut total_finished_paths: u32 = 0;
    particle_move(&chars, &mut visits,(0, beam_start_col), &mut total_finished_paths);

    for i in 0..visits.len() {
        for j in 0..visits[i].len() {
            print!("{:2} ", visits[i][j]);
        }
        println!();
    }

    Ok(total_finished_paths.to_string())
}