use crate::days::Day;
use bitvec::order::Msb0;
use bitvec::prelude::{BitView, Lsb0};
use bitvec::slice::BitSlice;
use bitvec::vec::BitVec;
use regex::Regex;
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::io::Read;

#[derive(Debug)]
struct Machine<I: Iterator<Item = Instruction>> {
    inner: I,
    curr_mask: HashMap<usize, bool>,
    memory: HashMap<u64, u64>,
}

impl<I> Machine<I>
where
    I: Iterator<Item = Instruction>,
{
    fn new(i: I) -> Self {
        Self {
            inner: i,
            curr_mask: HashMap::new(),
            memory: HashMap::new(),
        }
    }

    fn finish(mut self) -> HashMap<u64, u64> {
        while self.next().is_some() {}
        self.memory
    }
}

impl<'a, I> Iterator for Machine<I>
where
    I: Iterator<Item = Instruction>,
{
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.next()? {
            Instruction::Assignment(index, mut value) => {
                let bits = value.view_bits_mut::<Lsb0>();
                for (index, overwrite) in &self.curr_mask {
                    bits.set(*index, *overwrite);
                }
                self.memory.insert(index, value);
            }
            Instruction::Declaration(mask) => self.curr_mask = mask,
        }
        Some(())
    }
}

#[derive(Debug)]
struct Machine2<I: Iterator<Item = Instruction2>> {
    inner: I,
    curr_mask: Vec<MaskValues>,
    memory: HashMap<u64, u64>,
}

impl<I> Machine2<I>
where
    I: Iterator<Item = Instruction2>,
{
    fn new(i: I) -> Self {
        Self {
            inner: i,
            curr_mask: Vec::new(),
            memory: HashMap::new(),
        }
    }

    fn finish(mut self) -> HashMap<u64, u64> {
        while self.next().is_some() {}
        self.memory
    }
}

impl<'a, I> Iterator for Machine2<I>
where
    I: Iterator<Item = Instruction2>,
{
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.next()? {
            Instruction2::Assignment(index, value) => {
                let mut indicies = vec![index];
                for (i, mask_value) in self.curr_mask.iter().enumerate() {
                    match mask_value {
                        MaskValues::Unchanged => {}
                        MaskValues::Overwrite1 => {
                            for v in &mut indicies {
                                v.view_bits_mut::<Lsb0>().set(i, true);
                            }
                        }
                        MaskValues::Floating => {
                            let mut vec = Vec::new();
                            for v in &mut indicies {
                                v.view_bits_mut::<Lsb0>().set(i, true);
                                let mut v2 = v.clone();
                                v2.view_bits_mut::<Lsb0>().set(i, false);
                                vec.push(v2);
                            }
                            indicies = indicies.into_iter().chain(vec.into_iter()).collect();
                        }
                    }
                }
                for index in indicies {
                    self.memory.insert(index, value);
                }
            }
            Instruction2::Declaration(mask) => self.curr_mask = mask,
        }
        Some(())
    }
}

#[derive(Debug, Clone)]
enum Instruction {
    Declaration(HashMap<usize, bool>),
    Assignment(u64, u64),
}

#[derive(Debug, Clone)]
enum MaskValues {
    Unchanged,
    Overwrite1,
    Floating,
}

impl MaskValues {
    fn parse(c: char) -> Option<Self> {
        match c {
            '1' => Some(Self::Overwrite1),
            '0' => Some(Self::Unchanged),
            'X' => Some(Self::Floating),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
enum Instruction2 {
    Declaration(Vec<MaskValues>),
    Assignment(u64, u64),
}

pub struct Day14;

impl Day14 {
    fn parse(&self) -> Vec<Instruction> {
        let declaration_regex = Regex::new(r"mask\s=\s((1|0|X){36})").unwrap();
        let assignment_regex = Regex::new(r"mem\[(\d+)\]\s=\s(\d+)").unwrap();
        self.input()
            .split("\r\n")
            .filter(|x| !x.is_empty())
            .map(|x| {
                let caps = declaration_regex.captures(x);
                if caps.is_some() {
                    let caps = caps.unwrap();
                    let mut chars = caps.get(1).unwrap().as_str().chars().collect::<Vec<char>>();
                    chars.reverse();
                    Instruction::Declaration(
                        chars
                            .into_iter()
                            .enumerate()
                            .filter(|(_, x)| *x != 'X')
                            .map(|(i, x)| (i, x == '1'))
                            .collect::<HashMap<usize, bool>>(),
                    )
                } else {
                    let caps = assignment_regex.captures(x).unwrap();
                    Instruction::Assignment(
                        caps.get(1).unwrap().as_str().parse().unwrap(),
                        caps.get(2).unwrap().as_str().parse().unwrap(),
                    )
                }
            })
            .collect::<Vec<Instruction>>()
    }
    fn parse2(&self) -> Vec<Instruction2> {
        let declaration_regex = Regex::new(r"mask\s=\s((1|0|X){36})").unwrap();
        let assignment_regex = Regex::new(r"mem\[(\d+)\]\s=\s(\d+)").unwrap();
        self.input()
            .split("\r\n")
            .filter(|x| !x.is_empty())
            .map(|x| {
                let caps = declaration_regex.captures(x);
                if caps.is_some() {
                    let caps = caps.unwrap();
                    let mut chars = caps.get(1).unwrap().as_str().chars().collect::<Vec<char>>();
                    chars.reverse();
                    Instruction2::Declaration(
                        chars
                            .into_iter()
                            .map(|x| MaskValues::parse(x).unwrap())
                            .collect::<Vec<MaskValues>>(),
                    )
                } else {
                    let caps = assignment_regex.captures(x).unwrap();
                    Instruction2::Assignment(
                        caps.get(1).unwrap().as_str().parse().unwrap(),
                        caps.get(2).unwrap().as_str().parse().unwrap(),
                    )
                }
            })
            .collect::<Vec<Instruction2>>()
    }
}

impl Day<u128> for Day14 {
    fn part1(&self) -> u128 {
        let instructions = self.parse();
        let machine = Machine::new(instructions.into_iter());
        let memory = machine.finish();
        memory
            .into_iter()
            .map(|(_, i)| i as u128)
            .fold(0, |acc, x| acc + x)
    }

    fn part2(&self) -> u128 {
        let instructions = self.parse2();
        let machine = Machine2::new(instructions.into_iter());
        let memory = machine.finish();
        memory
            .into_iter()
            .map(|(_, i)| i as u128)
            .fold(0, |acc, x| acc + x)
    }

    fn input(&self) -> &str {
        include_str!("../inputs/14")
    }
}
