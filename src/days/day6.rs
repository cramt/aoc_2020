use crate::days::Day;
use std::collections::HashSet;
use std::collections::hash_map::RandomState;

//6th of december, my E day
pub struct Day6;

impl Day6 {
    fn parse(&self) -> Vec<HashSet<char, RandomState>> {
        self.input().split("\r\n\r\n")
            .map(|x| x.split_ascii_whitespace().collect::<String>().chars().collect::<HashSet<char>>())
            .collect::<Vec<HashSet<char>>>()
    }
}

impl Day<usize> for Day6 {
    fn part1(&self) -> usize {
        self.parse().into_iter().map(|x| x.len()).fold(0, |acc, x| acc + x)
    }

    fn part2(&self) -> usize {
        fn count(c: Vec<Vec<char>>) -> usize {
            c.into_iter()
                .map(|x| x.into_iter().collect::<HashSet<char>>())
                .fold(None, |acc, mut x| {
                    match acc {
                        None => Some(x),
                        Some(y) => Some(y.into_iter().filter(|z| x.contains(z)).collect())
                    }
                }).unwrap().len()
        }
        self.input().split("\r\n\r\n")
            .filter(|x| !x.is_empty())
            .map(|x| {
                count(x.split("\r\n").filter(|x| !x.is_empty()).map(|x| x.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>())
            }).fold(0, |acc, x| acc + x)
    }

    fn input(&self) -> &str {
        include_str!("../inputs/6")
    }
}
