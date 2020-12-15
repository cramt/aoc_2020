use crate::days::Day;
use regex::Regex;
use std::collections::hash_map::RandomState;
use std::collections::{HashMap, HashSet};
use std::net::ToSocketAddrs;
use std::ops::Deref;

#[derive(Debug, Clone)]
struct Bag {
    name: String,
    subs: Vec<(usize, String)>,
    parents: Vec<String>,
}

impl Bag {
    pub fn new<T: ToString>(s: T) -> Self {
        Self {
            name: s.to_string(),
            subs: Vec::new(),
            parents: Vec::new(),
        }
    }

    pub fn endpoint(&self) -> bool {
        self.subs.is_empty()
    }

    pub fn set_subs(&mut self, v: Vec<(usize, String)>) {
        self.subs = v;
    }

    pub fn set_parent(&mut self, v: String) {
        self.parents.push(v);
    }
}

pub struct Day7;

impl Day7 {
    fn raw_parse(&self) -> Vec<(String, Vec<(usize, String)>)> {
        let initial_regex = Regex::new(r"([^\s]+\s[^\s]+)\sbags\scontain\s(.+)").unwrap();
        let argument_regex = Regex::new(r"([0-9])\s([^\s]+\s[^\s]+)\sbags?").unwrap();
        let split: Vec<&str> = self
            .input()
            .split("\r\n")
            .filter(|x| !x.is_empty())
            .collect::<Vec<&str>>();
        split
            .into_iter()
            .map(|x| {
                let caps = initial_regex.captures(x).unwrap();
                let name = caps.get(1).unwrap().as_str().to_string();
                let mut argument = caps.get(2).unwrap().as_str().to_string();
                argument.pop();
                if argument == "no other bags" {
                    return (name, Vec::new());
                }
                let argument_split: Vec<&str> = argument.split(", ").collect::<Vec<&str>>();
                (
                    name,
                    argument_split
                        .into_iter()
                        .map(|y| {
                            let c = argument_regex.captures(y).unwrap();
                            let v = c.get(1).unwrap().as_str().parse::<usize>().unwrap();
                            let n = c.get(2).unwrap().as_str().to_string();
                            (v, n)
                        })
                        .collect::<Vec<(usize, String)>>(),
                )
            })
            .collect::<Vec<(String, Vec<(usize, String)>)>>()
    }

    fn parse(&self) -> HashMap<String, Box<Bag>, RandomState> {
        let raw = self.raw_parse();
        let mut map = HashMap::new();
        for (name, _) in &raw {
            map.insert(name.to_string(), Box::new(Bag::new(name)));
        }
        for (name, value) in raw {
            for (_, n) in &value {
                map.get_mut(n).unwrap().set_parent(name.clone());
            }
            map.get_mut(&name).unwrap().set_subs(value);
        }
        map
    }

    fn count1(&self, a: Vec<String>, map: &HashMap<String, Box<Bag>>) -> Vec<String> {
        let mut re = a.clone();
        for b in &a {
            let mut c = self.count1(map.get(b).unwrap().parents.clone(), &map);
            re.append(&mut c);
        }
        re
    }

    fn count2(&self, a: String, map: &HashMap<String, Box<Bag>>) -> usize {
        println!("{:?}", a);
        let b = map.get(&a).unwrap();
        b.subs
            .clone()
            .into_iter()
            .map(|x| self.count2(x.1, &map) * x.0)
            .sum::<usize>()
            + 1
    }
}

impl Day<usize> for Day7 {
    fn part1(&self) -> usize {
        let map = self.parse();
        self.count1(map.get("shiny gold").unwrap().parents.clone(), &map)
            .into_iter()
            .collect::<HashSet<String>>()
            .len()
    }

    fn part2(&self) -> usize {
        self.count2("shiny gold".to_string(), &self.parse()) - 1
    }

    fn input(&self) -> &str {
        include_str!("../inputs/7")
    }
}
