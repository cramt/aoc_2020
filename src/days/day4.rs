use crate::days::Day;
use regex::Regex;
use std::collections::HashMap;

pub struct Day4;

impl Day4 {
    fn parse(&self) -> Vec<HashMap<String, String>> {
        self.input()
            .split("\r\n\r\n")
            .map(|x| {
                x.split_ascii_whitespace()
                    .map(|y| {
                        let z = y
                            .split(":")
                            .map(|x| x.to_string())
                            .collect::<Vec<String>>()
                            .into_boxed_slice();
                        (z[0].clone(), z[1].clone())
                    })
                    .collect()
            })
            .collect()
    }
}

impl Day<usize> for Day4 {
    fn part1(&self) -> usize {
        let required_fields = [
            "byr".to_string(),
            "iyr".to_string(),
            "eyr".to_string(),
            "hgt".to_string(),
            "hcl".to_string(),
            "ecl".to_string(),
            "pid".to_string(),
        ];
        self.parse()
            .into_iter()
            .filter(|x| required_fields.iter().all(|y| x.contains_key(y.as_str())))
            .count()
    }

    fn part2(&self) -> usize {
        let length_regex = Regex::new(r"(\d+)([a-z]+)").unwrap();
        let colour_regex = Regex::new(r"#[0-f][0-f][0-f][0-f][0-f][0-f]").unwrap();
        let required_fields: [(String, Box<dyn Fn(&String) -> bool>); 7] = [
            (
                "byr".to_string(),
                Box::new(|x: &String| {
                    x.parse::<usize>()
                        .ok()
                        .map(|y| (1920usize..=2002usize).contains(&y))
                        .unwrap_or(false)
                }),
            ),
            (
                "iyr".to_string(),
                Box::new(|x: &String| {
                    x.parse::<usize>()
                        .ok()
                        .map(|y| (2010usize..=2020usize).contains(&y))
                        .unwrap_or(false)
                }),
            ),
            (
                "eyr".to_string(),
                Box::new(|x: &String| {
                    x.parse::<usize>()
                        .ok()
                        .map(|y| (2020usize..=2030usize).contains(&y))
                        .unwrap_or(false)
                }),
            ),
            (
                "hgt".to_string(),
                Box::new(|x: &String| {
                    let captures = match length_regex.captures(x.as_str()) {
                        Some(c) => c,
                        None => return false,
                    };
                    let measurement = captures.get(2).unwrap().as_str();
                    let value = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
                    match measurement {
                        "cm" => Some(150..=193),
                        "in" => Some(59..=76),
                        _ => None,
                    }
                    .map(|y| y.contains(&value))
                    .unwrap_or(false)
                }),
            ),
            (
                "hcl".to_string(),
                Box::new(|x: &String| colour_regex.is_match(x.as_str())),
            ),
            (
                "ecl".to_string(),
                Box::new(|x: &String| {
                    ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&x.as_str())
                }),
            ),
            (
                "pid".to_string(),
                Box::new(|x: &String| x.len() == 9 && x.parse::<u128>().is_ok()),
            ),
        ];
        self.parse()
            .into_iter()
            .filter(|x| {
                required_fields.iter().all(|y| match x.get(y.0.as_str()) {
                    None => false,
                    Some(value) => y.1(value),
                })
            })
            .count()
    }

    fn input(&self) -> &str {
        include_str!("../inputs/4")
    }
}
