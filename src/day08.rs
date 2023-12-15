use num::integer::lcm;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn day08() -> std::io::Result<()> {
    println!("Day 8");

    let file = File::open("src/input8.txt")?;
    let reader = BufReader::new(file);

    let mut answer_one: usize = 0;
    let mut answer_two: u128 = 0;

    let mut directions: String = "".to_owned();
    let mut elements: Vec<(String, Node)> = Vec::with_capacity(1000);

    for (index, line) in reader.lines().enumerate() {
        let actual_line = line.unwrap();
        if index == 0 {
            directions = actual_line;
            continue;
        }
        if index == 1 {
            continue;
        }
        elements.push((actual_line[0..3].to_owned(), parse(&actual_line)));
    }

    let zzz = "ZZZ".to_string();
    let mut part_one_element = "AAA".to_owned();
    let mut part_two_current_elements: Vec<(u128, String)> = Vec::with_capacity(1000);

    for element in &elements {
        if element.0.ends_with('A') {
            part_two_current_elements.push((0, element.0.to_owned()));
        }
    }

    for direction in directions.chars().cycle() {
        if part_one_element != zzz {
            part_one_element = get_next_element(&elements, direction, &part_one_element);
            answer_one += 1;
        }

        for part_two_element in part_two_current_elements.iter_mut() {
            if part_two_element.1.ends_with('Z') {
                continue;
            }

            part_two_element.1 = get_next_element(&elements, direction, &part_two_element.1);
            part_two_element.0 += 1;
        }

        if part_two_current_elements.iter().all(|x| x.1.ends_with('Z')) {
            break;
        }
    }

    for (i, part_two_element) in part_two_current_elements.iter().enumerate() {
        if i == 0 {
            answer_two = part_two_element.0;
        } else {
            answer_two = lcm(answer_two, part_two_element.0);
        }
    }

    println!("Answer 1: {}, 2: {}", answer_one, answer_two);
    Ok(())
}

fn get_next_element(
    elements: &Vec<(String, Node)>,
    direction: char,
    current_element: &str,
) -> String {
    let result = elements
        .iter()
        .enumerate()
        .find(|&(_, (string, _))| string == &current_element);
    if direction == 'R' {
        return elements.get(result.unwrap().0).unwrap().1.right.to_owned();
    } else {
        return elements.get(result.unwrap().0).unwrap().1.left.to_owned();
    }
}

#[derive(PartialEq, Debug)]
struct Node {
    left: String,
    right: String,
}

impl Default for Node {
    fn default() -> Self {
        Node {
            left: "".to_owned(),
            right: "".to_owned(),
        }
    }
}

fn parse(line: &str) -> Node {
    let mut result = Node {
        ..Default::default()
    };

    result.left = line[7..10].to_string();
    result.right = line[12..15].to_string();

    return result;
}

#[rustfmt::skip]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse () {
        assert_eq!(parse("AAA = (BBB, CCC)"), 
            Node {left: "BBB".to_owned(), right: "CCC".to_owned()});
    }

}
