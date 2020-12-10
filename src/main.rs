mod days;

use crate::days::Day;
use crate::days::day1::Day1;
use crate::days::day2::Day2;
use crate::days::day3::Day3;
use crate::days::day4::Day4;
use crate::days::day5::Day5;
use crate::days::day6::Day6;
use crate::days::day7::Day7;
use crate::days::day8::Day8;
use crate::days::day9::Day9;
use std::time::Instant;
use crate::days::day10::Day10;

fn main() {
    let now = Instant::now();
    println!("value: {:?}", Day10.part2());
    println!("{:?} micro", now.elapsed().as_micros())
}
