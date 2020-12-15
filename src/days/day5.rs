use crate::days::Day;
use std::collections::{HashMap, HashSet, VecDeque};
use std::iter::Map;

#[derive(Debug, Clone)]
struct CuttableRange {
    start: usize,
    end: usize,
}

impl CuttableRange {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    fn cut(&self) -> (Self, Self) {
        let cut = ((self.end + self.start) / 2);
        (Self::new(self.start, cut), Self::new(cut + 1, self.end))
    }

    fn finish(&self) -> usize {
        if self.start == self.end {
            self.start
        } else {
            panic!()
        }
    }
}

pub struct Day5;

impl Day5 {
    fn parse_identifiers(&self) -> Vec<(&str, &str)> {
        self.input()
            .split_ascii_whitespace()
            .filter(|x| *x != "")
            .map(|x| {
                let row_identifiers = &x[0..7];
                let column_identifiers = &x[7..];
                return (row_identifiers, column_identifiers);
            })
            .collect::<Vec<(&str, &str)>>()
    }

    fn parse_positions(&self) -> Vec<(usize, usize, usize)> {
        fn convert(chars: Vec<char>, up: char, down: char, starting_range: CuttableRange) -> usize {
            chars
                .into_iter()
                .fold(starting_range, |acc, x| {
                    let cut = acc.cut();
                    match x {
                        k if (k == up) => cut.1,
                        k if (k == down) => cut.0,
                        _ => panic!(),
                    }
                })
                .finish()
        }
        self.parse_identifiers()
            .into_iter()
            .map(|(row, column)| {
                (
                    row.chars().collect::<Vec<char>>(),
                    column.chars().collect::<Vec<char>>(),
                )
            })
            .map(|(row, column)| {
                (
                    convert(row, 'B', 'F', CuttableRange::new(0, 127)),
                    convert(column, 'R', 'L', CuttableRange::new(0, 7)),
                )
            })
            .map(|(row, column)| (row, column, (row * 8) + column))
            .collect::<Vec<(usize, usize, usize)>>()
    }

    fn split_range(&self) {}
}

impl Day<usize> for Day5 {
    fn part1(&self) -> usize {
        let mut ids = self
            .parse_positions()
            .into_iter()
            .map(|(_, _, id)| id)
            .collect::<Vec<usize>>();
        ids.sort_by(|b, a| a.cmp(b));
        return *ids.get(0).unwrap();
    }

    fn part2(&self) -> usize {
        let (row, columns) = self
            .parse_positions()
            .into_iter()
            .fold(
                HashMap::new(),
                |mut acc: HashMap<usize, Vec<usize>>, (row, column, _)| {
                    if !acc.contains_key(&row) {
                        acc.insert(row, Vec::new());
                    }
                    acc.get_mut(&row).unwrap().push(column);
                    acc
                },
            )
            .into_iter()
            .find(|(row, columns)| columns.len() == 7)
            .unwrap();
        let column = columns
            .into_iter()
            .fold(
                (0..=7).into_iter().collect::<HashSet<usize>>(),
                |mut acc, x| {
                    acc.remove(&x);
                    acc
                },
            )
            .into_iter()
            .find(|_| true)
            .unwrap();
        (row * 8) + column
    }

    fn input(&self) -> &str {
        include_str!("../inputs/5")
    }
}
