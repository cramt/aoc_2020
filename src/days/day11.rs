use crate::days::Day;
use combinations::Combinations;
use regex::internal::Input;

#[derive(Debug, Clone, Eq, PartialEq)]
enum Seat {
    Occupied,
    Unoccupied,
    Floor,
}

impl Seat {
    fn parse(c: char) -> Option<Self> {
        match c {
            '#' => Some(Self::Occupied),
            'L' => Some(Self::Unoccupied),
            '.' => Some(Self::Floor),
            _ => None,
        }
    }
}

impl ToString for Seat {
    fn to_string(&self) -> String {
        match self {
            Self::Occupied => "#",
            Self::Unoccupied => "L",
            Self::Floor => ".",
        }
        .to_string()
    }
}

#[derive(Debug)]
struct SeatGrid {
    seats: Box<[Box<[Seat]>]>,
    part: bool,
}

impl SeatGrid {
    fn parse<T: AsRef<str>>(s: T) -> Self {
        let seats = s
            .as_ref()
            .split_ascii_whitespace()
            .map(|x| {
                x.chars()
                    .map(|c| Seat::parse(c).unwrap())
                    .collect::<Vec<Seat>>()
                    .into_boxed_slice()
            })
            .collect::<Vec<Box<[Seat]>>>()
            .into_boxed_slice();
        Self { seats, part: false }
    }

    fn get_location_adjacent(&self, location: (usize, usize)) -> Vec<(usize, usize)> {
        let dirs = vec![
            (-1isize, -1isize),
            (-1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
            (1, 0),
            (1, -1),
            (0, -1),
        ];
        if self.part {
            dirs.into_iter()
                .filter_map(|x| {
                    let mut placement = (location.0 as isize, location.1 as isize);
                    loop {
                        placement.0 += x.0;
                        placement.1 += x.1;
                        if placement.0 < 0
                            || placement.1 < 0
                            || placement.0 >= self.seats.len() as isize
                            || placement.1 >= (&self.seats[placement.0 as usize]).len() as isize
                        {
                            return None;
                        }
                        if self.seats[placement.0 as usize][placement.1 as usize].clone()
                            != Seat::Floor
                        {
                            break;
                        }
                    }
                    Some(placement)
                })
                .map(|(x, y)| (x as usize, y as usize))
                .collect()
        } else {
            dirs.into_iter()
                .map(|x| (x.0 + location.0 as isize, x.1 + location.1 as isize))
                .filter(|(x, y)| *x >= 0 && *y >= 0)
                .map(|(x, y)| (x as usize, y as usize))
                .filter(|(x, y)| *x < self.seats.len() && *y < (&self.seats[*x]).len())
                .collect()
        }
    }

    fn get_adjacent_seats(&self, location: (usize, usize)) -> Vec<&Seat> {
        self.get_location_adjacent(location)
            .into_iter()
            .map(|(x, y)| &self.seats[x][y])
            .collect()
    }

    fn change(&mut self) -> usize {
        let mut new = self.seats.clone();
        let mut changes = 0;
        for i in 0..new.len() {
            for j in 0..new[i].len() {
                if new[i][j] == Seat::Floor {
                    continue;
                }
                let seats = self.get_adjacent_seats((i, j));
                let occupied = seats.iter().filter(|x| ***x == Seat::Occupied).count();
                let max = if self.part { 5 } else { 4 };
                if occupied == 0 {
                    if new[i][j] != Seat::Occupied {
                        new[i][j] = Seat::Occupied;
                        changes += 1;
                    }
                } else if occupied >= max {
                    if new[i][j] != Seat::Unoccupied {
                        new[i][j] = Seat::Unoccupied;
                        changes += 1;
                    }
                }
            }
        }
        self.seats = new;
        changes
    }

    fn finish(mut self) -> Box<[Box<[Seat]>]> {
        while self.next().is_some() {}
        self.seats
    }
}

impl ToString for SeatGrid {
    fn to_string(&self) -> String {
        self.seats
            .iter()
            .map(|x| x.iter().map(|y| y.to_string()).collect::<String>())
            .collect::<Vec<String>>()
            .join("\r\n")
    }
}

impl Iterator for SeatGrid {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let c = self.change();
        if c == 0 {
            None
        } else {
            Some(c)
        }
    }
}

pub struct Day11;

impl Day11 {
    fn parse(&self) -> SeatGrid {
        SeatGrid::parse(self.input())
    }
}

impl Day<usize> for Day11 {
    fn part1(&self) -> usize {
        self.parse()
            .finish()
            .to_vec()
            .into_iter()
            .map(|x| {
                x.to_vec()
                    .into_iter()
                    .map(|x| (x == Seat::Occupied) as usize)
                    .fold(0, |acc, x| acc + x)
            })
            .fold(0, |acc, x| acc + x)
    }

    fn part2(&self) -> usize {
        let mut grid = self.parse();
        grid.part = true;
        grid.finish()
            .to_vec()
            .into_iter()
            .map(|x| {
                x.to_vec()
                    .into_iter()
                    .map(|x| (x == Seat::Occupied) as usize)
                    .fold(0, |acc, x| acc + x)
            })
            .fold(0, |acc, x| acc + x)
    }

    fn input(&self) -> &str {
        include_str!("../inputs/11")
    }
}
