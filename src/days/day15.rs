use crate::days::Day;
use std::collections::{VecDeque, HashMap};

struct Runner {
    inner: HashMap<usize, usize>,
    current_index: usize,
    current_value: usize,
}

impl Runner {
    fn new(inner: Vec<usize>) -> Self {
        let mut inner = inner.into_iter().enumerate().collect::<Vec<(usize, usize)>>();
        let (index, value) = inner.pop().unwrap();
        let inner = inner.into_iter().map(|(i, x)| (x, i)).collect::<HashMap<usize, usize>>();
        Self {
            inner,
            current_index: index,
            current_value: value,
        }
    }
}

impl Iterator for Runner {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let new_number = self.inner.get(&self.current_value).map(|x| self.current_index - *x).unwrap_or(0);
        self.inner.insert(self.current_value, self.current_index);
        self.current_value = new_number;
        self.current_index += 1;
        Some(new_number)
    }
}

pub struct Day15;

impl Day15 {
    fn run(&self, times: usize) -> usize {
        let input = self.input().split(',').map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();
        let size = input.len() + 1;
        let mut runner = Runner::new(input);
        runner.nth(times - size).unwrap()
    }
}

impl Day<usize> for Day15 {
    fn part1(&self) -> usize {
        self.run(2020)
    }

    fn part2(&self) -> usize {
        self.run(30000000)
    }

    fn input(&self) -> &str {
        "14,1,17,0,3,20"
    }
}
