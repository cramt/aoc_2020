mod days;

use regex::Regex;
use std::collections::hash_map::Entry::Vacant;
use std::collections::HashMap;
use crate::days::day3::Day3;
use crate::days::Day;
use crate::days::day4::Day4;

fn main() {
    println!("{:?}", Day4.part1())
}
