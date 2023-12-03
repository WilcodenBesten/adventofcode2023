use crate::day01::day01;
use crate::day02::day02;
use crate::day03::day03;

pub mod day01;
pub mod day02;
pub mod day03;

fn main() {
    println!(
        "Running inside: {}",
        std::env::current_dir().unwrap().display()
    );
    day01().unwrap();
    day02().unwrap();
    day03().unwrap();

}
