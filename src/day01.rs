use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn day01() -> std::io::Result<()> {
    println!("Day 1");

    let file = File::open("src/input1.txt")?;
    let reader = BufReader::new(file);

    let mut answer_one: i32 = 0;
    let mut answer_two: i32 = 0;
    for line in reader.lines() {
        let actual_line = line.unwrap();
        answer_one += get_calibration_value(&actual_line);
        answer_two += get_calibration_value_extended(&actual_line);
    }

    println!("Answer 1: {}, 2: {}", answer_one, answer_two);
    Ok(())
}

fn get_calibration_value(value: &str) -> i32 {
    let mut result: i32 = 0;

    let numbers = value.trim_matches(char::is_alphabetic);
    let first_value = numbers.chars().next();
    let second_value = numbers.chars().next_back();

    if first_value.is_some() && second_value.is_some() {
        let combined = format!("{}{}", first_value.unwrap(), second_value.unwrap());
        result = combined.parse::<i32>().unwrap();
    }

    return result;
}

fn get_calibration_value_extended(value: &str) -> i32 {
    let modified_line = value
        .to_string()
        .replace("one", "one1one")
        .replace("two", "two2two")
        .replace("three", "three3three")
        .replace("four", "four4four")
        .replace("five", "five5five")
        .replace("six", "six6six")
        .replace("seven", "seven7seven")
        .replace("eight", "eight8eight")
        .replace("nine", "nine9nine");

    return get_calibration_value(&modified_line);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_part_one() {
        assert_eq!(get_calibration_value("1abc2"), 12);
        assert_eq!(get_calibration_value("pqr3stu8vwx"), 38);
        assert_eq!(get_calibration_value("a1b2c3d4e5f"), 15);
        assert_eq!(get_calibration_value("treb7uchet"), 77);
    }

    #[test]
    fn verify_part_two() {
        assert_eq!(get_calibration_value_extended("1abc2"), 12);
        assert_eq!(get_calibration_value_extended("pqr3stu8vwx"), 38);
        assert_eq!(get_calibration_value_extended("a1b2c3d4e5f"), 15);
        assert_eq!(get_calibration_value_extended("treb7uchet"), 77);

        assert_eq!(get_calibration_value_extended("two1nine"), 29);
        assert_eq!(get_calibration_value_extended("eightwothree"), 83);
        assert_eq!(get_calibration_value_extended("abcone2threexyz"), 13);
        assert_eq!(get_calibration_value_extended("xtwone3four"), 24);
        assert_eq!(get_calibration_value_extended("4nineeightseven2"), 42);
        assert_eq!(get_calibration_value_extended("zoneight234"), 14);
        assert_eq!(get_calibration_value_extended("7pqrstsixteen"), 76);
        assert_eq!(get_calibration_value_extended("eighthree"), 83);
        assert_eq!(get_calibration_value_extended("sevenine"), 79);
        assert_eq!(get_calibration_value_extended("oneight"), 18);
        assert_eq!(get_calibration_value_extended("tgppgp9"), 99);
        assert_eq!(get_calibration_value_extended("one1one"), 11);
        assert_eq!(get_calibration_value_extended("twoone"), 21);
        assert_eq!(get_calibration_value_extended("eighttwo"), 82);
        assert_eq!(get_calibration_value_extended("twone"), 21);
        assert_eq!(get_calibration_value_extended("eightwo"), 82);

        assert_eq!(get_calibration_value_extended("one"), 11);
        assert_eq!(get_calibration_value_extended("two"), 22);
        assert_eq!(get_calibration_value_extended("three"), 33);
        assert_eq!(get_calibration_value_extended("four"), 44);
        assert_eq!(get_calibration_value_extended("five"), 55);
        assert_eq!(get_calibration_value_extended("six"), 66);
        assert_eq!(get_calibration_value_extended("seven"), 77);
        assert_eq!(get_calibration_value_extended("eight"), 88);
        assert_eq!(get_calibration_value_extended("nine"), 99);
        assert_eq!(get_calibration_value_extended("nineight"), 98);
        assert_eq!(get_calibration_value_extended("onenineonenineonenine"), 19);

        assert_eq!(
            get_calibration_value_extended("six7ninetwosgtrpsqzltmjqkghrgbninexnmbbj"),
            69
        );
        assert_eq!(
            get_calibration_value_extended("one6rzeight61eightrdxgsdxx4"),
            14
        );
        assert_eq!(get_calibration_value_extended("xkvzhqj75"), 75);
        assert_eq!(get_calibration_value_extended("1565"), 15);
        assert_eq!(get_calibration_value_extended("qzhmmsqfc7"), 77);
        assert_eq!(
            get_calibration_value_extended("vjchzt7btthreesix1tcngpbtzsfmvsx"),
            71
        );
        assert_eq!(get_calibration_value_extended("sjv8"), 88);
        assert_eq!(
            get_calibration_value_extended("ncqpkzh5twooneoneqfxlqbjjhqsrlkhvdnvtbzpcbj"),
            51
        );
        assert_eq!(get_calibration_value_extended("449three45three"), 43);
    }
}
