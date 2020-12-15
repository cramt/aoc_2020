use crate::days::Day;
use regex::internal::Inst;
use regex::Regex;
use std::collections::VecDeque;
use std::iter::{Filter, Map};
use std::ops::Deref;
use std::str::{CharIndices, Lines};

#[derive(Debug, Eq, PartialEq, Clone)]
enum InstructionType {
    Nop,
    Jmp,
    Acc,
}

impl InstructionType {
    pub fn parse<T: AsRef<str>>(t: T) -> Option<Self> {
        match t.as_ref() {
            "nop" => Some(Self::Nop),
            "jmp" => Some(Self::Jmp),
            "acc" => Some(Self::Acc),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
struct Instruction {
    instruction_type: InstructionType,
    value: isize,
    visited: bool,
}

impl Instruction {
    fn new(instruction_type: InstructionType, value: isize) -> Self {
        Self {
            instruction_type,
            value,
            visited: false,
        }
    }

    fn visit(&mut self) {
        self.visited = true;
    }

    fn not_visited(&mut self) {
        self.visited = false;
    }

    fn flip(&mut self) {
        self.instruction_type = match self.instruction_type {
            InstructionType::Nop => InstructionType::Jmp,
            InstructionType::Jmp => InstructionType::Nop,
            InstructionType::Acc => InstructionType::Acc,
        };
    }
}

pub struct Day8;

impl Day8 {
    fn parse(&self) -> Box<[Instruction]> {
        let r = Regex::new(r"(jmp|acc|nop)\s([+\-]\d+)").unwrap();
        self.input()
            .lines()
            .filter(|x| !x.is_empty())
            .map(|x| {
                let caps = r.captures(x).unwrap();
                let t = InstructionType::parse(caps.get(1).unwrap().as_str()).unwrap();
                let v = caps.get(2).unwrap().as_str().parse::<isize>().unwrap();
                Instruction::new(t, v)
            })
            .collect::<Vec<Instruction>>()
            .into_boxed_slice()
    }

    fn parse_permutations(&self) -> Vec<Box<[Instruction]>> {
        let initial = self.parse();
        let mut res = vec![];
        for i in 0..initial.len() {
            if &initial[i].instruction_type != &InstructionType::Acc {
                let mut new = initial.clone();
                new[i].flip();
                res.push(new);
            }
        }
        res.push(initial);
        res
    }

    fn run(&self, mut instructions: Box<[Instruction]>) -> (isize, bool) {
        let mut index = 0;
        let mut acc = 0;
        loop {
            if index >= instructions.len() {
                break;
            }
            let inst = &mut instructions[index];
            if inst.visited {
                return (acc, true);
            }
            match inst.instruction_type {
                InstructionType::Nop => index += 1,
                InstructionType::Jmp => index = (index as isize + inst.value) as usize,
                InstructionType::Acc => {
                    acc += inst.value;
                    index += 1
                }
            };
            inst.visit();
        }
        (acc, false)
    }
}

impl Day<isize> for Day8 {
    fn part1(&self) -> isize {
        let mut instructions = self.parse();
        self.run(instructions).0
    }

    fn part2(&self) -> isize {
        let mut instructions = self.parse_permutations();
        instructions
            .into_iter()
            .map(|mut x| self.run(x))
            .find(|(_, failed)| !*failed)
            .unwrap()
            .0
    }

    fn input(&self) -> &str {
        include_str!("../inputs/8")
    }
}
