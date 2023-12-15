use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn day10() -> std::io::Result<()> {
    println!("Day 10");

    let file = File::open("src/input10.txt")?;
    let reader = BufReader::new(file);

    let mut answer_one: i128 = 0;
    let mut answer_two: i128 = 0;

    let mut map = Map {
        lines: Vec::with_capacity(1000),
    };

    for (index, line) in reader.lines().enumerate() {
        let actual_line = line.unwrap();
        map.lines.push(actual_line.chars().collect());
    }

    let start_pipe = map.find('S');

    let mut start_adjacent = map.get_adjacent(&start_pipe);

    answer_one = 1;
    while start_adjacent.0 .1.as_ref().unwrap() != start_adjacent.1 .1.as_ref().unwrap() {
        let first_direction_adjacent = map.get_adjacent(&start_adjacent.0 .1.as_ref().unwrap());
        let second_direction_adjacent = map.get_adjacent(&start_adjacent.1 .1.as_ref().unwrap());

        if &start_adjacent.0 .0.unwrap().get_reverse()
            == first_direction_adjacent.0 .0.as_ref().unwrap()
        {
            start_adjacent.0 = first_direction_adjacent.1;
        } else {
            start_adjacent.0 = first_direction_adjacent.0;
        }

        if &start_adjacent.1 .0.unwrap().get_reverse()
            == second_direction_adjacent.0 .0.as_ref().unwrap()
        {
            start_adjacent.1 = second_direction_adjacent.1;
        } else {
            start_adjacent.1 = second_direction_adjacent.0;
        }

        answer_one += 1;
    }

    println!("Answer 1: {}, 2: {}", answer_one, answer_two);
    Ok(())
}

#[derive(PartialEq, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn get_reverse(&self) -> Direction {
        match self {
            Direction::North => return Direction::South,
            Direction::South => return Direction::North,
            Direction::East => return Direction::West,
            Direction::West => return Direction::East,
        }
    }
}

#[derive(PartialEq, Debug)]
struct Pipe {
    x: usize,
    y: usize,
    shape: char,
}

#[derive(PartialEq, Debug)]
struct Map {
    lines: Vec<Vec<char>>, // x is lines index, y is char in line
}

impl Map {
    fn find(&self, item: char) -> Pipe {
        let mut result = Pipe {
            x: 0,
            y: 0,
            shape: item,
        };

        for (i, line) in self.lines.iter().enumerate() {
            let index = line.iter().position(|p| *p == item);
            if index.is_some() {
                result.x = i;
                result.y = index.unwrap();
            }
        }
        return result;
    }

    fn get_adjacent(
        &self,
        start_point: &Pipe,
    ) -> (
        (Option<Direction>, Option<Pipe>),
        (Option<Direction>, Option<Pipe>),
    ) {
        let mut result = (
            (Option::<Direction>::None, Option::<Pipe>::None),
            (Option::<Direction>::None, Option::<Pipe>::None),
        );

        if start_point.x < self.lines.len()
            && start_point.y < self.lines.get(start_point.x).unwrap().len()
        {
            let check_west = start_point.shape == 'S'
                || start_point.shape == '-'
                || start_point.shape == 'J'
                || start_point.shape == '7';
            let check_east = start_point.shape == 'S'
                || start_point.shape == '-'
                || start_point.shape == 'L'
                || start_point.shape == 'F';
            let check_north = start_point.shape == 'S'
                || start_point.shape == '|'
                || start_point.shape == 'L'
                || start_point.shape == 'J';
            let check_south = start_point.shape == 'S'
                || start_point.shape == '|'
                || start_point.shape == '7'
                || start_point.shape == 'F';

            // West
            if check_west && start_point.y > 0 {
                let char: &char = self
                    .lines
                    .get(start_point.x)
                    .unwrap()
                    .get(start_point.y - 1)
                    .unwrap();
                if *char != '.' {
                    result.0 .1 = Some(Pipe {
                        x: start_point.x,
                        y: start_point.y - 1,
                        shape: *char,
                    });
                    result.0 .0 = Some(Direction::West);

                    if start_point.shape == 'S' {
                        if *char != '-' && *char != 'L' && *char != 'F' {
                            result.0 = (Option::<Direction>::None, Option::<Pipe>::None);
                        }
                    }
                }
            }
            // East
            if check_east && start_point.y < self.lines.get(start_point.x).unwrap().len() - 1 {
                let char: &char = self
                    .lines
                    .get(start_point.x)
                    .unwrap()
                    .get(start_point.y + 1)
                    .unwrap();
                if *char != '.' {
                    if result.0 .1.is_none() {
                        result.0 .1 = Some(Pipe {
                            x: start_point.x,
                            y: start_point.y + 1,
                            shape: *char,
                        });
                        result.0 .0 = Some(Direction::East);

                        if start_point.shape == 'S' {
                            if *char != '-' && *char != 'J' && *char != '7' {
                                result.0 = (Option::<Direction>::None, Option::<Pipe>::None);
                            }
                        }
                    } else {
                        result.1 .1 = Some(Pipe {
                            x: start_point.x,
                            y: start_point.y + 1,
                            shape: *char,
                        });
                        result.1 .0 = Some(Direction::East);

                        if start_point.shape == 'S' {
                            if *char != '-' && *char != 'J' && *char != '7' {
                                result.1 = (Option::<Direction>::None, Option::<Pipe>::None);
                            }
                        }
                    }
                }
            }
            // North
            if check_north && start_point.x > 0 {
                let char: &char = self
                    .lines
                    .get(start_point.x - 1)
                    .unwrap()
                    .get(start_point.y)
                    .unwrap();
                if *char != '.' {
                    if result.0 .1.is_none() {
                        result.0 .1 = Some(Pipe {
                            x: start_point.x - 1,
                            y: start_point.y,
                            shape: *char,
                        });
                        result.0 .0 = Some(Direction::North);

                        if start_point.shape == 'S' {
                            if *char != '|' && *char != '7' && *char != 'F' {
                                result.0 = (Option::<Direction>::None, Option::<Pipe>::None);
                            }
                        }
                    } else {
                        result.1 .1 = Some(Pipe {
                            x: start_point.x - 1,
                            y: start_point.y,
                            shape: *char,
                        });
                        result.1 .0 = Some(Direction::North);

                        if start_point.shape == 'S' {
                            if *char != '|' && *char != '7' && *char != 'F' {
                                result.1 = (Option::<Direction>::None, Option::<Pipe>::None);
                            }
                        }
                    }
                }
            }
            // South
            if check_south && start_point.x < self.lines.len() - 1 {
                let char: &char = self
                    .lines
                    .get(start_point.x + 1)
                    .unwrap()
                    .get(start_point.y)
                    .unwrap();
                if *char != '.' {
                    result.1 .1 = Some(Pipe {
                        x: start_point.x + 1,
                        y: start_point.y,
                        shape: *char,
                    });
                    result.1 .0 = Some(Direction::South);

                    if start_point.shape == 'S' {
                        if *char != '|' && *char != 'L' && *char != 'J' {
                            result.1 = (Option::<Direction>::None, Option::<Pipe>::None);
                        }
                    }
                }
            }
        }
        return result;
    }
}

#[rustfmt::skip]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_find() {
        let map = Map{lines: vec![vec!['a', 'b', 'c'], vec!['1', '2', '3']]};

        assert_eq!(map.find('a'), Pipe{x:0, y: 0, shape: 'a'});
        assert_eq!(map.find('b'), Pipe{x:0, y: 1, shape: 'b'});
        assert_eq!(map.find('c'), Pipe{x:0, y: 2, shape: 'c'});
        assert_eq!(map.find('1'), Pipe{x:1, y: 0, shape: '1'});
        assert_eq!(map.find('2'), Pipe{x:1, y: 1, shape: '2'});
        assert_eq!(map.find('3'), Pipe{x:1, y: 2, shape: '3'});
    }

}
