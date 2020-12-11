use crate::days::Day;
use std::collections::{HashSet, HashMap};
use std::num::Wrapping;

struct Node {
    children: Vec<Node>,
    data: usize,
}

impl Node {
    pub fn new() -> Node {
        Node {
            children: vec!(),
            data: 0,
        }
    }

    pub fn expand(&mut self) {
        self.children = vec!(Node::new(), Node::new());
    }

    pub fn is_leaf(&self) -> bool {
        self.children.len() == 0
    }

    fn expand_leaf_and_inc(&mut self) {
        if self.is_leaf() {
            self.expand();
        } else {
            let index = 0;
            self.children[index].expand_leaf_and_inc();
        }
        self.data += 1
    }
}

struct PairsInclusive<T> {
    index: usize,
    value: Vec<T>,
}

impl<T> PairsInclusive<T> {
    fn new<S: Iterator<Item=T>>(s: S) -> Self {
        Self {
            value: s.collect(),
            index: 1,
        }
    }
}

impl<T> Iterator for PairsInclusive<T>
    where T: Clone {
    type Item = (T, T);

    fn next(&mut self) -> Option<Self::Item> {
        let re = Some((self.value.get(self.index - 1)?.clone(), self.value.get(self.index)?.clone()));
        self.index += 1;
        re
    }
}


pub struct Day10;

impl Day10 {
    fn parse(&self) -> Vec<usize> {
        let mut re =
            vec![0].into_iter().chain(self.input().split_ascii_whitespace()
                .filter(|x| !x.is_empty())
                .map(|x| x.parse::<usize>().unwrap()))
                .collect::<Vec<usize>>();
        re.sort();
        let last = re.last().unwrap() + 3;
        re.push(last);
        re
    }

    fn part2_oneliner(self) -> usize {
        let list = self.parse();
        let max = list.last().unwrap().clone();
        (1..=max).filter(|x| list.contains(x))
            .fold(
                vec![(0, 1)].into_iter().collect::<HashMap<usize, usize>>(),
                |mut acc, i| {
                    acc.insert(i, (1..=3).into_iter().map(|x| acc.get(&(Wrapping(i) - Wrapping(x)).0).unwrap_or(&0)).fold(0usize, |acc, x| acc + x));
                    acc
                }).get(&max).unwrap().clone()
    }
}

impl Day<usize> for Day10 {
    fn part1(&self) -> usize {
        let list = self.parse();
        let diffs = PairsInclusive::new(list.iter()).map(|(a, b)| b - a).collect::<Vec<usize>>();
        let ones = diffs.iter().filter(|x| **x == 1).count();
        let threes = diffs.iter().filter(|x| **x == 3).count();
        ones * threes
    }

    fn part2(&self) -> usize {
        let list = self.parse();
        let filter = list.clone().into_iter().collect::<HashSet<usize>>();
        let mut acc = HashMap::new();
        let max = list.last().unwrap().clone();
        acc.insert(0, 1);
        for i in 1..=max {
            if filter.contains(&i) {
                acc.insert(i, (1..=3).into_iter().map(|x| acc.get(&(Wrapping(i) - Wrapping(x)).0).unwrap_or(&0)).fold(0usize, |acc, x| acc + x));
            }
        }
        acc.get(&max).unwrap().clone()
    }



    fn input(&self) -> &str {
        include_str!("../inputs/10")
    }
}
