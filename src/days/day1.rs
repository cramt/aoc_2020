use crate::days::Day;

pub struct Day1;

impl Day<Option<u64>> for Day1 {
    fn part1(&self) -> Option<u64> {
        let values = self.input().split_ascii_whitespace().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>();
        for x in &values {
            for y in &values {
                if x + y == 2020 {
                    return Some(x * y);
                }
            }
        }
        None
    }

    fn part2(&self) -> Option<u64> {
        let values = self.input().split_ascii_whitespace().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>();
        for x in &values {
            for y in &values {
                for z in &values {
                    if x + y + z == 2020 {
                        return Some(x * y * z);
                    }
                }
            }
        }
        None
    }

    fn input(&self) -> &str {
        include_str!("../inputs/1")
    }
}
