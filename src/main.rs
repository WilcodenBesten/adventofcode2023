use crate::day01::day01;

pub mod day01;

fn main() {
    println!(
        "Running inside: {}",
        std::env::current_dir().unwrap().display()
    );
    day01().unwrap();
}
