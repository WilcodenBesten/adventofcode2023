use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn day15() -> std::io::Result<()> {
    println!("Day 15");

    let file = File::open("src/input15.txt")?;
    let reader = BufReader::new(file);

    let mut answer_one: u32 = 0;
    let mut answer_two: i128 = 0;

    for line in reader.lines() {
        let actual_line = line.unwrap();

        let mut current_value: u32 = 0;
        for character in actual_line.chars() {
            if character == ',' {
                answer_one += current_value;
                current_value = 0;
                continue;
            }
            let ascii_value = character as u32;
            current_value += ascii_value;
            current_value *= 17;
            current_value = current_value % 256;
        }
    }

    println!("Answer 1: {}, 2: {}", answer_one, answer_two);
    Ok(())
}

#[rustfmt::skip]
#[cfg(test)]
mod tests {
    use super::*;

}
