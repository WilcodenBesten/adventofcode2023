use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn day08() -> std::io::Result<()> {
    println!("Day 8");

    let file = File::open("src/input8.txt")?;
    let reader = BufReader::new(file);

    let mut answer_one: usize = 0;
    let mut answer_two: usize = 0;

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
    let mut current_element = "AAA".to_owned();
    
    for direction in directions.chars().cycle() {
        if current_element == zzz {
            break;
        }

        let result = elements
            .iter()
            .enumerate()
            .find(|&(_, (string, _))| string == &current_element);
        if direction == 'R' {
            current_element = elements.get(result.unwrap().0).unwrap().1.right.to_owned();
        } else {
            current_element = elements.get(result.unwrap().0).unwrap().1.left.to_owned();
        }

        answer_one += 1;
    }

    println!("Answer 1: {}, 2: {}", answer_one, answer_two);
    Ok(())
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
