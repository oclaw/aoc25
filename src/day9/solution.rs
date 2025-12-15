use crate::day;

pub struct Day {
}

#[derive(Clone, Debug)]
struct Point {
    col: i32,
    row: i32,
}

impl day::Day for Day {
    fn part1(&self, input: &str) -> Result<String, Box<dyn std::error::Error>> {
        let mut points = input.lines()
            .map(|line| {
                let mut parts = line.split(',');
                Point {
                    col: parts.next().unwrap().trim().parse::<i32>().unwrap(),
                    row: parts.next().unwrap().trim().parse::<i32>().unwrap(),
                }
            })
            .collect::<Vec<Point>>();

        points.sort_by(|a, b| {            
            if a.row == b.row {
                a.col.cmp(&b.col)
            } else {
                a.row.cmp(&b.row)
            }
        });

        let mut max_rectangle_area: i64 = 0;

        for i in 0..points.len() {
            for j in i+1..points.len() {
                let p1 = &points[i];
                let p2 = &points[j];

                if p1.col != p2.col && p1.row != p2.row {
                    let side_1 = (p2.col - p1.col + 1).abs();
                    let side_2 = (p2.row - p1.row + 1).abs();
                    let area: i64 = side_1 as i64 * side_2 as i64;
                    if area > max_rectangle_area {
                        // println!("New max rectangle: {:?}, {:?}, area: {}, sides: ({}, {})", p1, p2, area, side_1, side_2);

                        max_rectangle_area = area;
                    }
                }
            }
        }

        Ok(max_rectangle_area.to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Box<dyn std::error::Error>> {
        Ok(String::from("not implemented"))
    }
}