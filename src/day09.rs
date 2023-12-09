use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn day09() -> std::io::Result<()> {
    println!("Day 9");

    let file = File::open("src/input9.txt")?;
    let reader = BufReader::new(file);

    let mut answer_one: i128 = 0;
    let mut answer_two: i128 = 0;

    let mut lines: Vec<Vec<i128>> = Vec::with_capacity(1000);

    for (index, line) in reader.lines().enumerate() {
        let actual_line = line.unwrap();

        lines.push(
            actual_line
                .split_ascii_whitespace()
                .map(|x| x.parse::<i128>().unwrap())
                .collect(),
        );
    }

    for line in lines {
        answer_one += get_next_history(&line);
        answer_two += get_prev_history(&line);
    }

    println!("Answer 1: {}, 2: {}", answer_one, answer_two);
    Ok(())
}

fn get_next_history(input: &Vec<i128>) -> i128 {
    let mut last_numbers: Vec<i128> = vec![*input.last().unwrap()];

    let mut differences: Vec<i128> = input.to_owned();

    while last_numbers.last().unwrap() != &0 {
        for i in 0..differences.len() {
            if differences.get(i + 1).is_none() {
                break;
            }
            differences[i] = differences[i + 1] - differences[i];
        }
        differences.pop();

        last_numbers.push(*differences.last().unwrap());
    }

    return last_numbers.iter().sum();
}

fn get_prev_history(input: &Vec<i128>) -> i128 {
    let mut first_numbers: Vec<i128> = vec![*input.first().unwrap()];

    let mut differences: Vec<i128> = input.to_owned();
    differences.reverse();

    while differences.first().unwrap() != &0 {
        for i in 0..differences.len() {
            if differences.get(i + 1).is_none() {
                break;
            }
            differences[i] = differences[i] - differences[i + 1];
        }
        differences.pop();

        first_numbers.push(*differences.last().unwrap());
    }

    first_numbers.reverse();

    for i in 1..first_numbers.len() {
        first_numbers[i] = first_numbers[i] - first_numbers[i - 1];
    }

    return *first_numbers.last().unwrap();
}

#[rustfmt::skip]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_next_history () {
        let first: Vec<i128> = [0, 3, 6, 9, 12, 15].to_vec();
        let sec: Vec<i128> = [1, 3, 6, 10, 15, 21].to_vec();
        let mut third: Vec<i128> = [10, 13, 16, 21, 30, 45].to_vec();

        assert_eq!(get_next_history(&first), 18);
        assert_eq!(get_next_history(&sec), 28);
        assert_eq!(get_next_history(&third), 68);

        assert_eq!(get_prev_history(&third), 5);
    }

}
