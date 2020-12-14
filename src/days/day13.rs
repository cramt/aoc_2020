use crate::days::Day;
use std::ops::RangeFrom;
use std::cmp::min;

pub struct Day13;

impl Day13 {
    fn parse(&self) -> (usize, Vec<usize>) {
        let cut = self.input().split_ascii_whitespace().collect::<Vec<&str>>();
        let a = cut.get(0).unwrap().parse::<usize>().unwrap();
        let b = cut.get(1).unwrap().split(',').filter_map(|x| x.parse::<usize>().ok()).collect::<Vec<usize>>();
        (a, b)
    }
    fn parse2(&self) -> (usize, Vec<(usize, usize)>) {
        let cut = self.input().split_ascii_whitespace().collect::<Vec<&str>>();
        let a = cut.get(0).unwrap().parse::<usize>().unwrap();
        let b = cut.get(1).unwrap().split(',').enumerate().filter_map(|(i, x)| Some((i, x.parse::<usize>().ok()?))).collect::<Vec<(usize, usize)>>();
        (a, b)
    }
}

impl Day<usize> for Day13 {
    fn part1(&self) -> usize {
        let (time, buses) = self.parse();
        for i in time.. {
            for bus in &buses {
                if i % bus == 0 {
                    return (i - time) * bus;
                }
            }
        }
        0
    }

    fn part2(&self) -> usize {
        let (_, buses) = self.parse2();
        let mut curr = 0usize;
        let mut step_size = 1usize;
        for (offset, bus_id) in buses.iter() {
            let timestamp = (curr..).step_by(step_size).find(|timestamp| (timestamp + offset) % bus_id == 0).unwrap();
            curr = timestamp;
            step_size *= bus_id;
        }
        curr
    }

    fn input(&self) -> &str {
        include_str!("../inputs/13")
    }
}
