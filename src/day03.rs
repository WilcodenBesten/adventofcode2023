use std::fs::File;
use std::io::{BufRead, BufReader};
use std::usize;

pub fn day03() -> std::io::Result<()> {
    println!("Day 3");

    let file = File::open("src/input3.txt")?;
    let reader = BufReader::new(file);

    let mut answer_one: usize = 0;
    let mut answer_two: usize = 0;

    let mut lines: Vec<PartLine> = Vec::new();
    let mut sum: usize = 0;

    for (_index, line) in reader.lines().enumerate() {
        let actual_line = line.unwrap();

        lines.push(parse(&actual_line));
    }

    // Part one
    for i in 0..lines.len() {
        let mut line_sum: usize = 0;

        for y in 0..lines[i].parts.len() {
            let prev_part = if y > 0 {
                Some(&lines[i].parts[y - 1])
            } else {
                None
            };
            let next_part = if y < lines[i].parts.len() - 1 {
                Some(&lines[i].parts[y + 1])
            } else {
                None
            };

            let current_part = &lines[i].parts[y];

            // Check on same line
            if current_part.is_symbol {
                continue;
            }

            if let Some(prev_val) = prev_part {
                if prev_val.is_symbol && current_part.is_adjacent(prev_val) {
                    line_sum += current_part.value;
                }
            }

            if let Some(next_val) = next_part {
                if next_val.is_symbol && current_part.is_adjacent(next_val) {
                    line_sum += current_part.value;
                }
            }
        }

        // Check against prev and next line
        let prev_line = if i > 0 { Some(&lines[i - 1]) } else { None };
        let next_line = if i < lines.len() - 1 {
            Some(&lines[i + 1])
        } else {
            None
        };

        for part in &lines[i].parts {
            if part.is_symbol {
                continue;
            }

            if let Some(prev) = prev_line {
                for prev_part in &prev.parts {
                    if prev_part.is_symbol && part.is_adjacent(prev_part) {
                        line_sum += part.value;
                    }
                }
            }

            if let Some(next) = next_line {
                for next_part in &next.parts {
                    if next_part.is_symbol && part.is_adjacent(next_part) {
                        line_sum += part.value;
                    }
                }
            }
        }

        sum += line_sum;
    }
    answer_one = sum;
    sum = 0;

    // Part two
    for i in 0..lines.len() {
        let mut matches: Vec<usize> = Vec::new();

        let prev_line = if i > 0 { Some(&lines[i - 1]) } else { None };
        let next_line = if i < lines.len() - 1 {
            Some(&lines[i + 1])
        } else {
            None
        };

        for y in 0..lines[i].parts.len() {
            let prev = if y > 0 {
                Some(&lines[i].parts[y - 1])
            } else {
                None
            };
            let next = if y < lines[i].parts.len() - 1 {
                Some(&lines[i].parts[y + 1])
            } else {
                None
            };

            let current_part = &lines[i].parts[y];

            if !current_part.is_symbol || !current_part.is_gear {
                continue;
            }

            // Check on same line against previous part
            if let Some(prev) = prev {
                if !prev.is_symbol && current_part.is_adjacent(prev) {
                    matches.push(prev.value);
                }
            }

            // Check on same line against next part
            if let Some(next) = next {
                if !next.is_symbol && current_part.is_adjacent(next) {
                    matches.push(next.value);
                }
            }

            // Check on previous line against all parts
            if let Some(prev) = prev_line {
                for prev_part in &prev.parts {
                    if !prev_part.is_symbol && current_part.is_adjacent(prev_part) {
                        matches.push(prev_part.value);
                    }
                }
            }

            // Check on next line against all parts
            if let Some(next) = next_line {
                for next_part in &next.parts {
                    if !next_part.is_symbol && current_part.is_adjacent(next_part) {
                        matches.push(next_part.value);
                    }
                }
            }

            if matches.len() > 1 {
                sum += matches[0] * matches[1];
            }
            matches.clear();
        }
    }
    answer_two = sum;

    println!("Answer 1: {}, 2: {}", answer_one, answer_two);
    Ok(())
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Part {
    start: usize,
    size: usize,
    value: usize,
    is_symbol: bool,
    is_gear: bool,
}

impl Part {
    fn is_adjacent(self, other: &Part) -> bool {
        if other.start + 1 == self.start {
            /*
            .123.
            *... */
            return true;
        } else if other.start == self.start + 1 {
            return true;
        } else if other.start >= self.start && other.start <= self.start + self.size {
            /*
            .123.
            ..*. */
            return true;
        } else if self.start >= other.start && self.start <= other.start + other.size {
            return true;
        } else if other.start == self.start + self.size {
            /*
            .123.
            ....* */
            return true;
        } else if other.start + other.size == self.start {
            return true;
        }
        return false;
    }
}

#[derive(Debug, PartialEq)]
struct PartLine {
    parts: Vec<Part>,
}

fn parse(input: &str) -> PartLine {
    let mut part_line: PartLine = PartLine { parts: Vec::new() };

    let mut number_as_string: String = "".to_owned();
    let mut start_index: usize = 0;

    for (index, character) in input.chars().enumerate() {
        let mut do_parse_number = false;
        let mut push_symbol = false;

        if character == '.' {
            if !number_as_string.is_empty() {
                do_parse_number = true;
            }
        } else if character.is_ascii_punctuation() {
            push_symbol = true;
            if !number_as_string.is_empty() {
                do_parse_number = true;
            }
        } else if character.is_ascii_digit() {
            if number_as_string.is_empty() {
                start_index = index;
            }
            number_as_string.push(character);
        }

        if do_parse_number {
            part_line.parts.push(Part {
                start: start_index,
                size: number_as_string.len(),
                value: number_as_string.parse::<usize>().unwrap(),
                is_symbol: false,
                is_gear: false,
            });
            start_index = 0;
            number_as_string.clear();
        }
        if push_symbol {
            if character == '*' {
                part_line.parts.push(Part {
                    start: index,
                    size: 1,
                    value: 0,
                    is_symbol: true,
                    is_gear: true,
                });
            } else {
                part_line.parts.push(Part {
                    start: index,
                    size: 1,
                    value: 0,
                    is_symbol: true,
                    is_gear: false,
                });
            }
        }
    }

    if !number_as_string.is_empty() {
        part_line.parts.push(Part {
            start: start_index,
            size: number_as_string.len(),
            value: number_as_string.parse::<usize>().unwrap(),
            is_symbol: false,
            is_gear: false,
        });
    }

    return part_line;
}

#[rustfmt::skip]
#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(parse("467..114.."), PartLine{parts: vec![Part{start: 0, size: 3, value: 467, is_symbol: false, is_gear: false}, 
                                                             Part{start: 5, size: 3, value: 114, is_symbol: false, is_gear: false}]});
        assert_eq!(parse("...*......"), PartLine{parts: vec![Part{start: 3, size: 1, value: 0, is_symbol: true, is_gear: true}]});
        assert_eq!(parse("..35..633."), PartLine{parts: vec![Part{start: 2, size: 2, value: 35, is_symbol: false, is_gear: false}, 
                                                             Part{start: 6, size: 3, value: 633, is_symbol: false, is_gear: false}]});
        assert_eq!(parse("......#..."), PartLine{parts: vec![Part{start: 6, size: 1, value: 0, is_symbol: true, is_gear: false}]});
        assert_eq!(parse("617*......"), PartLine{parts: vec![Part{start: 0, size: 3, value: 617, is_symbol: false, is_gear: false}, 
                                                             Part{start: 3, size: 1, value: 0, is_symbol: true, is_gear: true}]});
        assert_eq!(parse(".....+.58."), PartLine{parts: vec![Part{start: 5, size: 1, value: 0, is_symbol: true, is_gear: false},
                                                             Part{start: 7, size: 2, value: 58, is_symbol: false, is_gear: false}]});
        assert_eq!(parse("..592....."), PartLine{parts: vec![Part{start: 2, size: 3, value: 592, is_symbol: false, is_gear: false}]});
        assert_eq!(parse("......755."), PartLine{parts: vec![Part{start: 6, size: 3, value: 755, is_symbol: false, is_gear: false}]});
        assert_eq!(parse("...$.*...."), PartLine{parts: vec![Part{start: 3, size: 1, value: 0, is_symbol: true, is_gear: false},
                                                             Part{start: 5, size: 1, value: 0, is_symbol: true, is_gear: true}]});
        assert_eq!(parse(".664.598.."), PartLine{parts: vec![Part{start: 1, size: 3, value: 664, is_symbol: false, is_gear: false},
                                                             Part{start: 5, size: 3, value: 598, is_symbol: false, is_gear: false}]});
        assert_eq!(parse(".................................................324.663............775...290=.301...............=...15........=....780..................562"),
                   PartLine{parts: vec![
                        Part{start: 49, size: 3, value: 324, is_symbol: false, is_gear: false},
                        Part{start: 53, size: 3, value: 663, is_symbol: false, is_gear: false},
                        Part{start: 68, size: 3, value: 775, is_symbol: false, is_gear: false},
                        Part{start: 74, size: 3, value: 290, is_symbol: false, is_gear: false},
                        Part{start: 77, size: 1, value: 0, is_symbol: true, is_gear: false},
                        Part{start: 79, size: 3, value: 301, is_symbol: false, is_gear: false},
                        Part{start: 97, size: 1, value: 0, is_symbol: true, is_gear: false},
                        Part{start: 101, size: 2, value: 15, is_symbol: false, is_gear: false},
                        Part{start: 111, size: 1, value: 0, is_symbol: true, is_gear: false},
                        Part{start: 116, size: 3, value: 780, is_symbol: false, is_gear: false},
                        Part{start: 137, size: 3, value: 562, is_symbol: false, is_gear: false},
                   ]});
        assert_eq!(parse("246*637"), PartLine{parts: vec![Part {start: 0, size: 3, value: 246, is_symbol: false, is_gear: false},
                                                          Part {start: 3, size: 1, value: 0, is_symbol: true, is_gear: true},
                                                          Part {start: 4, size: 3, value: 637, is_symbol: false, is_gear: false}]});

    }

    #[test]
    fn test_line_is_adjacent() {
        // ..467..
        let number = Part {start: 2, size: 3, value: 467, is_symbol: false, is_gear: false };
       
        // *......
        let mut symbol = Part {start: 0, size: 1, value: 0, is_symbol: true, is_gear: true };
        assert_eq!(number.is_adjacent(&symbol), false);
        // .*.....
        symbol.start += 1;
        assert_eq!(number.is_adjacent(&symbol), true);
        // ..*....
        symbol.start += 1;
        assert_eq!(number.is_adjacent(&symbol), true);
        // ...*...
        symbol.start += 1;
        assert_eq!(number.is_adjacent(&symbol), true);
        // ....*..
        symbol.start += 1;
        assert_eq!(number.is_adjacent(&symbol), true);
        // .....*.
        symbol.start += 1;
        assert_eq!(number.is_adjacent(&symbol), true);
        // ......*
        symbol.start += 1;
        assert_eq!(number.is_adjacent(&symbol), false);

        // 246*637
        let first = Part {start: 0, size: 3, value: 246, is_symbol: false, is_gear: false};
        let gear = Part {start: 3, size: 1, value: 0, is_symbol: true, is_gear: true};
        let second = Part {start: 4, size: 3, value: 637, is_symbol: false, is_gear: false};
        assert!(first.is_adjacent(&gear));
        assert!(second.is_adjacent(&gear));

        let number_467 = Part {start: 0, size: 3, value: 467, is_symbol: false, is_gear: false }; 
        let gear = Part {start: 3, size: 1, value: 0, is_symbol: true, is_gear: true };
        assert_eq!(number_467.is_adjacent(&gear), true);
        assert_eq!(gear.is_adjacent(&number_467), true);

        let number_69 = Part {start: 0, size: 2, value: 69, is_symbol: false, is_gear: false }; 
        let number_973 = Part {start: 0, size: 3, value: 973, is_symbol: false, is_gear: false }; 
        let gear = Part {start: 0, size: 1, value: 0, is_symbol: true, is_gear: true };
        assert_eq!(number_69.is_adjacent(&gear), true);
        assert_eq!(gear.is_adjacent(&number_69), true);
        assert_eq!(number_973.is_adjacent(&gear), true);
        assert_eq!(gear.is_adjacent(&number_973), true);

        let number_635 = Part {start: 127, size: 3, value: 635, is_symbol: false, is_gear: false };
        let number_168 = Part {start: 127, size: 3, value: 168, is_symbol: false, is_gear: false };
        let gear = Part{start: 129, size: 1, value: 0, is_symbol: true, is_gear: true};
        assert_eq!(gear.is_adjacent(&number_635), true);
        assert_eq!(gear.is_adjacent(&number_168), true);

    }

}
