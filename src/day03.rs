use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn day03() -> std::io::Result<()> {
    println!("Day 3");

    let file = File::open("src/input3.txt")?;
    let reader = BufReader::new(file);

    let mut answer_one: usize = 0;
    let mut answer_two: i32 = 0;

    let mut prev_part_line = Line {
        numbers: vec![],
        numbers_start: vec![],
        numbers_len: vec![],
        numbers_is_adjacent: vec![],
        symbols_indexes: vec![],
    };

    for (_index, line) in reader.lines().enumerate() {
        let actual_line = line.unwrap();

        let mut next_part_line = parse_line(&actual_line);
        next_part_line.set_adjacent();

        next_part_line.set_adjacent_compared_to(&prev_part_line);

        prev_part_line.set_adjacent_compared_to(&next_part_line);

        answer_one += prev_part_line.get_sum();

        prev_part_line = next_part_line;
    }
    answer_one += prev_part_line.get_sum();

    println!("Answer 1: {}, 2: {}", answer_one, answer_two);
    Ok(())
}

#[derive(Debug, PartialEq)]
struct Line {
    numbers: Vec<usize>,
    numbers_start: Vec<usize>,
    numbers_len: Vec<usize>,
    numbers_is_adjacent: Vec<bool>,
    symbols_indexes: Vec<usize>,
}

impl Line {
    fn get_sum(&self) -> usize {
        let mut sum: usize = 0;
        for (index, number) in self.numbers.iter().enumerate() {
            if *self.numbers_is_adjacent.get(index).unwrap() {
                sum += number;
            }
        }
        return sum;
    }

    fn set_adjacent(&mut self) {
        if !self.symbols_indexes.is_empty() && !self.numbers.is_empty() {
            for symbol_index in self.symbols_indexes.iter() {
                for (list_index, number_start) in self.numbers_start.iter().enumerate() {
                    if number_start + self.numbers_len.get(list_index).unwrap()
                        == *symbol_index
                    {
                        // Verify: 123*
                        self.numbers_is_adjacent[list_index] = true;
                    } else if *symbol_index + 1 == *self.numbers_start.get(list_index).unwrap() {
                        // Verify: *123
                        self.numbers_is_adjacent[list_index] = true;
                    }
                }
            }
        }
    }

    // Other is the line below our line
    fn set_adjacent_compared_to(&mut self, other: &Line) {
        if !other.symbols_indexes.is_empty() {
            for symbol_index in other.symbols_indexes.iter() {
                for (list_index, number_start) in self.numbers_start.iter().enumerate() {
                    if *symbol_index + 1 == *number_start {
                        /*
                        .123.
                        *... */
                        self.numbers_is_adjacent[list_index] = true;
                    } else if *symbol_index >= *self.numbers_start.get(list_index).unwrap()
                        && *symbol_index
                            <= self.numbers_start.get(list_index).unwrap()
                                + self.numbers_len.get(list_index).unwrap()
                    {
                        /*
                        .123.
                        ..*. */
                        self.numbers_is_adjacent[list_index] = true;
                    } else if *symbol_index
                        == self.numbers_start.get(list_index).unwrap()
                            + self.numbers_len.get(list_index).unwrap()
                    {
                        /*
                        .123.
                        ....* */
                        self.numbers_is_adjacent[list_index] = true;
                    }
                }
            }
        }
    }
}

fn parse_line(input: &str) -> Line {
    let mut result = Line {
        numbers: vec![],
        numbers_start: vec![],
        numbers_len: vec![],
        numbers_is_adjacent: vec![],
        symbols_indexes: vec![],
    };

    let mut number_as_string: String = "".to_owned();

    for (index, character) in input.chars().enumerate() {
        let mut do_parse_number = false;

        if character == '.' {
            if !number_as_string.is_empty() {
                do_parse_number = true;
            }
        } else if character.is_ascii_punctuation() {
            result.symbols_indexes.push(index);
            if !number_as_string.is_empty() {
                do_parse_number = true;
            }
        } else if character.is_ascii_digit() {
            if number_as_string.is_empty() {
                result.numbers_start.push(index);
            }
            number_as_string.push(character);
        }

        if do_parse_number {
            result
                .numbers
                .push(number_as_string.parse::<usize>().unwrap());
            result.numbers_len.push(number_as_string.len());
            result.numbers_is_adjacent.push(false);
            number_as_string.clear();
        }
    }

    if !number_as_string.is_empty() {
        result
            .numbers
            .push(number_as_string.parse::<usize>().unwrap());
        result.numbers_len.push(number_as_string.len());
        result.numbers_is_adjacent.push(false);
    }

    return result;
}

#[rustfmt::skip]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_parse_line() {
        assert_eq!(parse_line("467..114.."), Line{ numbers: vec![467, 114], numbers_start: vec![0, 5], numbers_len: vec![3, 3], numbers_is_adjacent: vec![false, false], symbols_indexes: vec![] });
        assert_eq!(parse_line("...*......"), Line{ numbers: vec![],         numbers_start: vec![],     numbers_len: vec![],     numbers_is_adjacent: vec![], symbols_indexes: vec![3] });
        assert_eq!(parse_line("..35..633."), Line{ numbers: vec![35, 633],  numbers_start: vec![2, 6], numbers_len: vec![2, 3], numbers_is_adjacent: vec![false, false], symbols_indexes: vec![] });
        assert_eq!(parse_line("......#..."), Line{ numbers: vec![],         numbers_start: vec![],     numbers_len: vec![],     numbers_is_adjacent: vec![], symbols_indexes: vec![6] });
        assert_eq!(parse_line("617*......"), Line{ numbers: vec![617],      numbers_start: vec![0],    numbers_len: vec![3],    numbers_is_adjacent: vec![false], symbols_indexes: vec![3] });
        assert_eq!(parse_line(".....+.58."), Line{ numbers: vec![58],       numbers_start: vec![7],    numbers_len: vec![2],    numbers_is_adjacent: vec![false], symbols_indexes: vec![5] });
        assert_eq!(parse_line("..592....."), Line{ numbers: vec![592],      numbers_start: vec![2],    numbers_len: vec![3],    numbers_is_adjacent: vec![false], symbols_indexes: vec![] });
        assert_eq!(parse_line("......755."), Line{ numbers: vec![755],      numbers_start: vec![6],    numbers_len: vec![3],    numbers_is_adjacent: vec![false], symbols_indexes: vec![] });
        assert_eq!(parse_line("...$.*...."), Line{ numbers: vec![],         numbers_start: vec![],     numbers_len: vec![],     numbers_is_adjacent: vec![], symbols_indexes: vec![3,5] });
        assert_eq!(parse_line(".664.598.."), Line{ numbers: vec![664, 598], numbers_start: vec![1,5],  numbers_len: vec![3, 3], numbers_is_adjacent: vec![false, false], symbols_indexes: vec![] });
        assert_eq!(parse_line(".................................................324.663............775...290=.301...............=...15........=....780..................562"), 
                   Line{ numbers: vec![324, 663, 775, 290, 301, 15, 780, 562], 
                        numbers_start: vec![49,53, 68,74,79, 101, 116, 137],  
                        numbers_len: vec![3, 3, 3, 3, 3, 2, 3, 3], 
                        numbers_is_adjacent: vec![false, false, false, false, false, false, false, false], 
                        symbols_indexes: vec![77, 97,111] });
    }

    #[test]
    fn verify_get_sum() {
        assert_eq!(Line{ numbers: vec![100, 200], numbers_start: vec![0, 5], numbers_len: vec![3, 3], numbers_is_adjacent: vec![false, false], symbols_indexes: vec![] }.get_sum(), 0);
        assert_eq!(Line{ numbers: vec![100, 200], numbers_start: vec![0, 5], numbers_len: vec![3, 3], numbers_is_adjacent: vec![true, false], symbols_indexes: vec![] }.get_sum(), 100);
        assert_eq!(Line{ numbers: vec![100, 200], numbers_start: vec![0, 5], numbers_len: vec![3, 3], numbers_is_adjacent: vec![false, true], symbols_indexes: vec![] }.get_sum(), 200);
        assert_eq!(Line{ numbers: vec![100, 200], numbers_start: vec![0, 5], numbers_len: vec![3, 3], numbers_is_adjacent: vec![true, true], symbols_indexes: vec![] }.get_sum(), 300);
    }

    #[test]
    fn verify_line_set_adjacent() {
        let mut line = parse_line("467.");
        line.set_adjacent();

        assert_eq!(line.numbers_is_adjacent, vec![false]);
        
        let mut line = parse_line("617*.");
        line.set_adjacent();

        assert_eq!(line.numbers_is_adjacent, vec![true]);

        let mut line = parse_line("*617.");
        line.set_adjacent();

        assert_eq!(line.numbers_is_adjacent, vec![true]);

        let mut line = parse_line(".................................................324.663............775...290=.301...............=...15........=....780..................562");
        line.set_adjacent();

        assert_eq!(line.numbers_is_adjacent, vec![false, false, false, true, false, false, false, false]);
    }

    #[test]
    fn verify_line_set_adjacent_compared_to() {
        let mut line = parse_line("..467..");
        // Not adjacent
        let line2 = parse_line("*......");
        let line3 = parse_line("......*");

        line.set_adjacent_compared_to(&line2);
        assert_eq!(line.numbers_is_adjacent, vec![false]);
        line.numbers_is_adjacent[0] = false;

        line.set_adjacent_compared_to(&line3);
        assert_eq!(line.numbers_is_adjacent, vec![false]);
        line.numbers_is_adjacent[0] = false;

        // Adjacent
        let line4 = parse_line(".*.....");
        let line5 = parse_line("..*....");
        let line6 = parse_line("...*....");
        let line7 = parse_line("....*..");
        let line8 = parse_line(".....*.");

        line.set_adjacent_compared_to(&line4);
        assert_eq!(line.numbers_is_adjacent, vec![true]);
        line.numbers_is_adjacent[0] = false;

        line.set_adjacent_compared_to(&line5);
        assert_eq!(line.numbers_is_adjacent, vec![true]);
        line.numbers_is_adjacent[0] = false;

        line.set_adjacent_compared_to(&line6);
        assert_eq!(line.numbers_is_adjacent, vec![true]);
        line.numbers_is_adjacent[0] = false;

        line.set_adjacent_compared_to(&line7);
        assert_eq!(line.numbers_is_adjacent, vec![true]);
        line.numbers_is_adjacent[0] = false;

        line.set_adjacent_compared_to(&line8);
        assert_eq!(line.numbers_is_adjacent, vec![true]);
        line.numbers_is_adjacent[0] = false;
    }

}
