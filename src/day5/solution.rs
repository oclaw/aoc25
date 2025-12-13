use crate::day;

pub struct Day {
}

impl day::Day for Day {
    fn part1(&self, input: &str) -> Result<String, Box<dyn std::error::Error>> {
        let ranges = input
            .lines()
            .take_while(|line| !line.is_empty())
            .map(|line| {
                let parts = line.split('-').map(|part| part.parse::<u64>().unwrap()).collect::<Vec<u64>>();
                return (parts[0], parts[1]);
            })
            .collect::<Vec<(u64, u64)>>();

        let values = input.lines()
            .skip_while(|line| !line.is_empty())
            .skip(1)
            .map(|line| line.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();

        let mut fresh_ids_count = 0;
        values.iter().for_each(|value| {
            for range in &ranges {
                if *value >= range.0 && *value <= range.1 {
                    fresh_ids_count += 1;
                    return;
                }
            }
        });

        Ok(fresh_ids_count.to_string())


    }

    fn part2(&self, input: &str) -> Result<String, Box<dyn std::error::Error>> {


       let mut ranges = input
            .lines()
            .take_while(|line| !line.is_empty())
            .map(|line| {
                let parts = line.split('-').map(|part| part.parse::<u64>().unwrap()).collect::<Vec<u64>>();
                return Range{
                    start: parts[0], end: parts[1]};
            })
            .collect::<Vec<Range>>();


        loop {
            let mut merged_ranges: Vec<Range> = Vec::new();
            let mut erased_ranges = std::iter::repeat(false).take(ranges.len()).collect::<Vec<bool>>();

            ranges.iter().enumerate().for_each(|(idx, range)| {
                if erased_ranges[idx] {
                    return;
                }

                let mut has_merged = false;

                ranges.iter().enumerate().skip(idx+1).for_each(|(jdx, other_range)| {
                    if erased_ranges[jdx] {
                        return;
                    }

                    if ranges_overlap(range, other_range) {
                        let new_range = Range {
                            start: std::cmp::min(range.start, other_range.start),
                            end: std::cmp::max(range.end, other_range.end),
                        };

                        merged_ranges.push(new_range);
                        has_merged = true;
                        erased_ranges[jdx] = true;
                    }
                });

                if !has_merged {
                    merged_ranges.push(Range { start: range.start, end: range.end });
                } 

            });

            if merged_ranges.len() == ranges.len() {
                break;
            }
            std::mem::swap(&mut ranges, &mut merged_ranges);
            
        }

        let result: u64 = ranges.iter().map(|range| range.end - range.start + 1).sum();
        Ok(result.to_string())
    }
}


struct Range {
    start: u64,
    end: u64,
}

impl std::fmt::Debug for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]", self.start, self.end)
    }
}

fn ranges_overlap(r1: &Range, r2: &Range) -> bool {
    return r1.start <= r2.end && r2.start <= r1.end;
}
