pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;

pub trait Day<T>{
    fn part1(&self) -> T;
    fn part2(&self) -> T;
    fn input(&self) -> &str;
}
