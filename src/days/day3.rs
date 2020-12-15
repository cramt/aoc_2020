use crate::days::Day;

pub struct Day3;

impl Day3 {
    fn day3_crawl(&self, value: &Box<[Box<[bool]>]>, mut slope: (usize, usize)) -> usize {
        let height = value.len();
        let width = value[0].len();
        let mut curr_pos = (0usize, 0usize);
        let mut trees = 0usize;
        loop {
            if value[curr_pos.0][curr_pos.1] {
                trees += 1;
            }
            curr_pos.0 += slope.0;
            curr_pos.1 = (curr_pos.1 + slope.1) % width;
            if curr_pos.0 >= height {
                break;
            }
        }
        trees
    }

    fn parse(&self) -> Box<[Box<[bool]>]> {
        self.input()
            .split_ascii_whitespace()
            .map(|x| {
                x.chars()
                    .map(|x| x == '#')
                    .collect::<Vec<bool>>()
                    .into_boxed_slice()
            })
            .collect::<Vec<Box<[bool]>>>()
            .into_boxed_slice()
    }
}

impl Day<usize> for Day3 {
    fn part1(&self) -> usize {
        let value = self.parse();
        self.day3_crawl(&value, (1, 3))
    }

    fn part2(&self) -> usize {
        let value = self.parse();
        [(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)]
            .to_vec()
            .into_iter()
            .map(|x| self.day3_crawl(&value, x))
            .fold(None, |acc, x| Some(acc.unwrap_or(1) * x))
            .unwrap()
    }

    fn input(&self) -> &str {
        include_str!("../inputs/3")
    }
}
