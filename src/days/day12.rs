use crate::days::Day;
use regex::Regex;
use std::panic::resume_unwind;

#[derive(Debug, Clone)]
enum Instruction {
    North(isize),
    South(isize),
    East(isize),
    West(isize),
    Left(isize),
    Right(isize),
    Forward(isize),
}

impl Instruction {
    fn parse<S: AsRef<str>>(s: S) -> Option<Self> {
        let regex = Regex::new(r"(N|S|W|E|L|R|F)(\d+)").unwrap();
        let caps = regex.captures(s.as_ref())?;
        let dir = caps.get(1)?.as_str();
        let value = caps.get(2)?.as_str().parse::<isize>().unwrap();
        match dir {
            "N" => Some(Self::North(value)),
            "S" => Some(Self::South(value)),
            "E" => Some(Self::East(value)),
            "W" => Some(Self::West(value)),
            "L" => Some(Self::Left(value)),
            "R" => Some(Self::Right(value)),
            "F" => Some(Self::Forward(value)),
            _ => None
        }
    }
}

#[derive(Debug)]
struct ShipRunner<I: Iterator<Item=Instruction>> {
    inner: I,
    dir: (isize, isize),
    pos: (isize, isize),
}

impl<I> ShipRunner<I>
    where I: Iterator<Item=Instruction> {
    fn new(t: I) -> Self {
        Self {
            inner: t,
            dir: (0, 1),
            pos: (0, 0),
        }
    }

    fn do_instruction(&mut self, inst: Instruction) {
        match inst {
            Instruction::North(v) => self.pos.0 += v,
            Instruction::South(v) => self.pos.0 -= v,
            Instruction::East(v) => self.pos.1 += v,
            Instruction::West(v) => self.pos.1 -= v,
            Instruction::Left(v) => {
                for _ in 0..(v / 90) {
                    self.dir = (self.dir.1, self.dir.0 * -1)
                }
            }
            Instruction::Right(v) => {
                for _ in 0..(v / 90) {
                    self.dir = (self.dir.1 * -1, self.dir.0)
                }
            }
            Instruction::Forward(v) => self.pos = (self.pos.0 + (self.dir.0 * v), self.pos.1 + (self.dir.1 * v))
        }
    }
}

impl<I> Iterator for ShipRunner<I>
    where I: Iterator<Item=Instruction> {
    type Item = (isize, isize);

    fn next(&mut self) -> Option<Self::Item> {
        let inst = self.inner.next()?;
        self.do_instruction(inst);
        Some(self.pos.clone())
    }
}

#[derive(Debug)]
struct ShipRunner2<I: Iterator<Item=Instruction>> {
    inner: I,
    waypoing_pos: (isize, isize),
    ship_pos: (isize, isize),
}

impl<I> ShipRunner2<I>
    where I: Iterator<Item=Instruction> {
    fn new(t: I) -> Self {
        Self {
            inner: t,
            waypoing_pos: (1, 10),
            ship_pos: (0, 0),
        }
    }

    fn do_instruction(&mut self, inst: Instruction) {
        match inst {
            Instruction::North(v) => self.waypoing_pos.0 += v,
            Instruction::South(v) => self.waypoing_pos.0 -= v,
            Instruction::East(v) => self.waypoing_pos.1 += v,
            Instruction::West(v) => self.waypoing_pos.1 -= v,
            Instruction::Left(v) => {
                for _ in 0..(v / 90) {
                    self.waypoing_pos = (self.waypoing_pos.1, self.waypoing_pos.0 * -1)
                }
            }
            Instruction::Right(v) => {
                for _ in 0..(v / 90) {
                    self.waypoing_pos = (self.waypoing_pos.1 * -1, self.waypoing_pos.0)
                }
            }
            Instruction::Forward(v) => self.ship_pos = (self.ship_pos.0 + (self.waypoing_pos.0 * v), self.ship_pos.1 + (self.waypoing_pos.1 * v))
        }
    }
}

impl<I> Iterator for ShipRunner2<I>
    where I: Iterator<Item=Instruction> {
    type Item = (isize, isize);

    fn next(&mut self) -> Option<Self::Item> {
        let inst = self.inner.next()?;
        self.do_instruction(inst);
        Some(self.ship_pos.clone())
    }
}

pub struct Day12;

impl Day12 {
    fn parse(&self) -> Vec<Instruction> {
        self.input()
            .split_ascii_whitespace()
            .filter(|x| !x.is_empty())
            .map(|x| Instruction::parse(x).unwrap())
            .collect()
    }
}

impl Day<isize> for Day12 {
    fn part1(&self) -> isize {
        let instructions = self.parse();
        let runner = ShipRunner::new(instructions.into_iter());
        let pos = runner.last().unwrap();
        pos.0.abs() + pos.1.abs()
    }

    fn part2(&self) -> isize {
        let instructions = self.parse();
        let runner = ShipRunner2::new(instructions.into_iter());
        let pos = runner.last().unwrap();
        pos.0.abs() + pos.1.abs()
    }

    fn input(&self) -> &str {
        include_str!("../inputs/12")
    }
}
