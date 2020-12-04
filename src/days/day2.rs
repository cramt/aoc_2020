use crate::days::Day;
use regex::Regex;
use std::collections::HashMap;

pub struct Day2;

impl Day<usize> for Day2 {
    fn part1(&self) -> usize {
        let r = Regex::new(r"([0-9]+)-([0-9]+)\s([a-z]):\s(.*)").unwrap();
        let values = self.input().split("\n").filter(|x| !x.is_empty()).map(|x| {
            let captures = r.captures(x).unwrap();
            let start_range = captures.get(1).unwrap().as_str().parse::<u32>().unwrap();
            let end_range = captures.get(2).unwrap().as_str().parse::<u32>().unwrap();
            let char = captures.get(3).unwrap().as_str().chars().collect::<Vec<char>>()[0];
            let password = captures.get(4).unwrap().as_str().to_string();
            let range = (start_range..=end_range);
            (range, char, password)
        });
        values.into_iter().filter(|(range, char, password)| {
            let char_map = password.chars().fold(HashMap::new(), |mut hm, c| {
                hm.insert(c, hm.get(&c).unwrap_or(&0) + 1);
                hm
            });
            let char_count = char_map.get(char).unwrap_or(&0);
            range.contains(char_count)
        }).count()
    }

    fn part2(&self) -> usize {
        let r = Regex::new(r"([0-9]+)-([0-9]+)\s([a-z]):\s(.*)").unwrap();
        let values = self.input().split("\n").filter(|x| !x.is_empty()).map(|x| {
            let captures = r.captures(x).unwrap();
            let index1 = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let index2 = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();
            let char = captures.get(3).unwrap().as_str().chars().collect::<Vec<char>>()[0];
            let password = captures.get(4).unwrap().as_str().chars().collect::<Vec<char>>().into_boxed_slice();
            (index1, index2, char, password)
        });
        values.into_iter().filter(|(index1, index2, char, password)| {
            (&password[*index1 - 1] == char) != (&password[*index2 - 1] == char)
        }).count()
    }

    fn input(&self) -> &str {
        include_str!("../inputs/2")
    }
}
