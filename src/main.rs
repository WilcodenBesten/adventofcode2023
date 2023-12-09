use crate::day01::day01;
use crate::day02::day02;
use crate::day03::day03;
use crate::day04::day04;
use crate::day05::day05;
use crate::day06::day06;
use crate::day07::day07;
use crate::day08::day08;
use crate::day09::day09;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;

fn main() {
    println!(
        "Running inside: {}",
        std::env::current_dir().unwrap().display()
    );
    // day01().unwrap();
    // day02().unwrap();
    // day03().unwrap();
    // day04().unwrap();
    // day05().unwrap();
    // day06().unwrap();
    // day07().unwrap();
    // day08().unwrap();
    day09().unwrap();
}
