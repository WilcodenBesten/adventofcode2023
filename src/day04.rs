use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn day04() -> std::io::Result<()> {
    println!("Day 4");

    let file = File::open("src/input4.txt")?;
    let reader = BufReader::new(file);

    let mut answer_one: u32 = 0;
    let mut answer_two: u32 = 0;

    // nr cards, nr winning numbers
    let mut cards: Vec<(u32, u32)> = Vec::new();

    for (_index, line) in reader.lines().enumerate() {
        let actual_line = line.unwrap();

        answer_one += get_points(&actual_line);

        // Calculate this new card copies by traversing the cards list
        // and adding up all copies of existing cards, subtracting the number of wins remaining by one
        // and removing any entries that have no more wins remaining
        let mut nr_cards_copies = 1;
        cards.retain_mut(|(c, n)| {
            nr_cards_copies += *c;
            *n -= 1;
            *n > 0
        });

        answer_two += nr_cards_copies;

        let nr_matching_numbers = get_nr_matching_numbers(&actual_line);
        if nr_matching_numbers > 0 {
            cards.push((nr_cards_copies, nr_matching_numbers));
        }
    }

    println!("Answer 1: {}, 2: {}", answer_one, answer_two);
    Ok(())
}

fn get_nr_matching_numbers(input: &str) -> u32 {
    let start_winning_numbers = input.find(':').unwrap();
    let start_our_numbers = input.find('|').unwrap();

    let winning_numbers = &input[start_winning_numbers + 1..start_our_numbers]
        .trim()
        .split(' ')
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    let our_numbers = &input[start_our_numbers + 1..]
        .trim()
        .split(' ')
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    let mut sum: u32 = 0;
    for number in our_numbers {
        if number.len() == 0 {
            continue;
        }

        for winning_number in winning_numbers {
            if number == winning_number {
                sum += 1;
                break;
            }
        }
    }

    return sum;
}

fn get_points(input: &str) -> u32 {
    let mut points: u32 = get_nr_matching_numbers(input);
    if points > 0 {
        points = u32::pow(2, points - 1);
    }
    return points;
}

#[rustfmt::skip]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_nr_matching_numbers() {
        assert_eq!(get_nr_matching_numbers("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"), 4);
        assert_eq!(get_nr_matching_numbers("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19"), 2);
        assert_eq!(get_nr_matching_numbers("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1"), 2);
        assert_eq!(get_nr_matching_numbers("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83"), 1);
        assert_eq!(get_nr_matching_numbers("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36"), 0);
        assert_eq!(get_nr_matching_numbers("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"), 0);
    }

    #[test]
    fn test_get_points() {
        assert_eq!(get_points("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"), 8);
        assert_eq!(get_points("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19"), 2);
        assert_eq!(get_points("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1"), 2);
        assert_eq!(get_points("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83"), 1);
        assert_eq!(get_points("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36"), 0);
        assert_eq!(get_points("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"), 0);
    }

}
