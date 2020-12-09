mod days;

use regex::Regex;
use std::collections::hash_map::Entry::Vacant;
use std::collections::HashMap;
use crate::days::day3::Day3;
use crate::days::Day;
use crate::days::day4::Day4;
use crate::days::day5::Day5;
use crate::days::day6::Day6;
use crate::days::day7::Day7;
use crate::days::day8::Day8;

fn main() {
    println!("{:?}", Day8.part2());
}
