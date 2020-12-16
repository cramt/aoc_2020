use crate::days::Day;
use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Debug;
use std::ops::Range;

#[derive(Debug)]
struct GreaterRange<Idx>
where
    Idx: PartialOrd<Idx>,
{
    ranges: Vec<Range<Idx>>,
}

impl<Idx> GreaterRange<Idx>
where
    Idx: PartialOrd<Idx>,
    Idx: Debug,
{
    fn new(ranges: Vec<Range<Idx>>) -> Self {
        Self { ranges }
    }

    fn contains(&self, item: &Idx) -> bool {
        self.ranges.iter().any(|x| x.contains(item))
    }
}

pub struct Day16;

impl Day16 {
    fn parse(
        &self,
    ) -> (
        Vec<(String, GreaterRange<usize>)>,
        Vec<usize>,
        Vec<Vec<usize>>,
    ) {
        let rule_regex = Regex::new(r"(.+):\s(\d+-\d+)\sor\s(\d+-\d+)").unwrap();
        fn int_list_parse<S: AsRef<str>>(s: S) -> Vec<usize> {
            s.as_ref()
                .split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        }
        let inputs = self.input().split("\r\n\r\n").collect::<Vec<&str>>();
        let rules = inputs
            .get(0)
            .unwrap()
            .lines()
            .map(|x| {
                let caps = rule_regex.captures(x).unwrap();
                let name = caps.get(1).unwrap().as_str().to_string();
                let first_range = caps
                    .get(2)
                    .unwrap()
                    .as_str()
                    .split('-')
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>();
                let second_range = caps
                    .get(3)
                    .unwrap()
                    .as_str()
                    .split('-')
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>();
                let first_range =
                    (first_range.get(0).unwrap().clone())..(first_range.get(1).unwrap() + 1);
                let second_range =
                    (second_range.get(0).unwrap().clone())..(second_range.get(1).unwrap() + 1);
                let range = GreaterRange::new(vec![first_range, second_range]);
                (name, range)
            })
            .collect::<Vec<(String, GreaterRange<usize>)>>();
        let your_ticket = int_list_parse(
            inputs
                .get(1)
                .unwrap()
                .lines()
                .collect::<Vec<&str>>()
                .last()
                .unwrap(),
        );
        let mut other_tickets = inputs
            .get(2)
            .unwrap()
            .lines()
            .filter(|x| !x.is_empty())
            .collect::<VecDeque<&str>>();
        other_tickets.pop_front();
        let other_tickets = other_tickets
            .into_iter()
            .map(int_list_parse)
            .collect::<Vec<Vec<usize>>>();
        (rules, your_ticket, other_tickets)
    }
}

impl Day<usize> for Day16 {
    fn part1(&self) -> usize {
        let (rules, your_ticket, others_tickets) = self.parse();
        let ranges = rules
            .iter()
            .map(|(_, x)| x)
            .collect::<Vec<&GreaterRange<usize>>>();
        others_tickets
            .into_iter()
            .map(|x| {
                x.into_iter()
                    .filter(|value| {
                        !ranges
                            .iter()
                            .map(|y| y.contains(value))
                            .fold(false, |a, b| a || b)
                    })
                    .fold(0, |a, b| a + b)
            })
            .fold(0, |a, b| a + b)
    }

    fn part2(&self) -> usize {
        let (rules, your_ticket, others_tickets) = self.parse();
        let ranges = rules
            .iter()
            .map(|(_, x)| x)
            .collect::<Vec<&GreaterRange<usize>>>();
        let filtered = others_tickets
            .into_iter()
            .filter(|x| {
                x.into_iter().all(|value| {
                    ranges
                        .iter()
                        .map(|y| y.contains(value))
                        .fold(false, |a, b| a || b)
                })
            })
            .collect::<Vec<Vec<usize>>>();
        let mut rule_index_map = rules
            .iter()
            .map(|(x, _)| {
                (
                    x.to_string(),
                    (0..(filtered.get(0).unwrap().len()))
                        .into_iter()
                        .collect::<HashSet<usize>>(),
                )
            })
            .collect::<HashMap<String, HashSet<usize>>>();
        for x in filtered {
            for (i, y) in x.into_iter().enumerate() {
                for (rule_name, rule) in &rules {
                    if !rule.contains(&y) {
                        rule_index_map.get_mut(rule_name).unwrap().remove(&i);
                    }
                }
            }
        }
        loop {
            if rule_index_map.values().all(|x| x.len() == 1) {
                break;
            }
            let vec = rule_index_map
                .iter()
                .filter(|(_, x)| x.len() == 1)
                .map(|(_, x)| x.into_iter().find(|_| true).unwrap().clone())
                .collect::<Vec<usize>>();
            for n in vec {
                for val in rule_index_map.values_mut() {
                    if val.len() != 1 {
                        val.remove(&n);
                    }
                }
            }
        }
        let rules = rule_index_map
            .into_iter()
            .map(|(name, value)| (name, value.into_iter().find(|_| true).unwrap()))
            .collect::<Vec<(String, usize)>>();
        rules
            .iter()
            .filter(|(name, _)| name.starts_with("departure"))
            .map(|(_, x)| x)
            .map(|x| your_ticket.get(x.clone()).unwrap())
            .fold(1, |a, b| a * b)
    }

    fn input(&self) -> &str {
        include_str!("../inputs/16")
    }
}
