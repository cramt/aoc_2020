use crate::days::Day;
use std::collections::VecDeque;
use combinations::Combinations;

#[derive(Debug)]
struct ContinuesSubsets<T> {
    start: usize,
    end: usize,
    v: Vec<T>,
}

impl<V> ContinuesSubsets<V> {
    fn new<T: Iterator<Item=V>>(v: T) -> Self {
        Self {
            start: 0,
            end: 2,
            v: v.collect(),
        }
    }
}

impl<T> Iterator for ContinuesSubsets<T>
    where T: Clone {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut re = Vec::new();
        for i in self.start..self.end {
            re.push(self.v.get(i)?.clone())
        }
        self.end += 1;
        if self.end == self.v.len() {
            self.start += 1;
            self.end = self.start + 2;
        }
        Some(re)
    }
}


#[derive(Debug)]
struct XmasRunner {
    loaded: VecDeque<u128>,
    unloaded: VecDeque<u128>,
    finished: Vec<u128>
}

impl XmasRunner {
    fn new<T: Iterator<Item=u128>>(load_size: usize, values: T) -> Self {
        let mut loaded = VecDeque::with_capacity(load_size);
        let mut unloaded = values.collect::<VecDeque<u128>>();
        for _ in 0..load_size {
            loaded.push_back(unloaded.pop_front().unwrap());
        }
        Self {
            loaded,
            unloaded,
            finished: Vec::new()
        }
    }

    fn get_loaded_comb(&self) -> impl Iterator<Item=(u128, u128)> {
        Combinations::new(self.loaded.clone().into_iter().collect(), 2)
            .map(|x| (x.get(0).unwrap().clone(), x.get(1).unwrap().clone()))
    }

    fn finish(self) -> impl Iterator<Item = u128>{
        self.finished.into_iter().chain(self.loaded.into_iter())
    }
}

impl Iterator for XmasRunner {
    type Item = (u128, bool);

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.unloaded.pop_front()?;
        let validity = self.get_loaded_comb().find(|(a, b)| *a + *b == value);
        self.finished.push(self.loaded.pop_front().unwrap());
        self.loaded.push_back(value);
        Some((value, validity.is_some()))
    }
}

pub struct Day9;

impl Day9 {
    fn parse(&self) -> Vec<u128> {
        self.input().split_ascii_whitespace().map(|x| x.parse::<u128>().unwrap()).collect::<Vec<u128>>()
    }
}

impl Day<u128> for Day9 {
    fn part1(&self) -> u128 {
        let mut runner = XmasRunner::new(25, self.parse().into_iter());
        runner.find(|(number, valid)| !*valid).unwrap().0
    }

    fn part2(&self) -> u128 {
        let mut runner = XmasRunner::new(25, self.parse().into_iter());
        let n = runner.find(|(number, valid)| !*valid).unwrap().0;
        let mut found = ContinuesSubsets::new(runner.finish()).find(|x| x.into_iter().fold(0, |acc, x| acc + x) == n).unwrap();
        found.sort();
        found.last().unwrap() + found.first().unwrap()
    }

    fn input(&self) -> &str {
        include_str!("../inputs/9")
    }
}
