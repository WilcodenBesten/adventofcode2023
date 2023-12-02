use std::cmp::max;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn day02() -> std::io::Result<()> {
    println!("Day 2");

    let file = File::open("src/input2.txt")?;
    let reader = BufReader::new(file);

    let mut answer_one: usize = 0;
    let mut answer_two: i32 = 0;

    for (index, line) in reader.lines().enumerate() {
        let actual_line = line.unwrap();

        if is_game_possible(12, 13, 14, &actual_line) {
            answer_one += index + 1;
        }

        answer_two += get_power(&actual_line);
    }

    println!("Answer 1: {}, 2: {}", answer_one, answer_two);
    Ok(())
}

#[derive(Debug, PartialEq)]
struct CubesShown {
    red: i32,
    green: i32,
    blue: i32,
}

fn is_game_possible(
    nr_red_cubes: i32,
    nr_green_cubes: i32,
    nr_blue_cubes: i32,
    input: &str,
) -> bool {
    let mut result = true;

    let start_of_subsets_index = input.find(":").unwrap();
    let raw_subsets = &input[start_of_subsets_index + 1..];
    let raw_cubes = raw_subsets.split(";");

    for raw_cube in raw_cubes {
        let cubes_shown = get_cubes_shown(&raw_cube);

        if cubes_shown.red > nr_red_cubes
            || cubes_shown.green > nr_green_cubes
            || cubes_shown.blue > nr_blue_cubes
        {
            result = false;
            break;
        }
    }

    return result;
}

fn get_cube_number(input: &str, index: usize) -> i32 {
    let start_index = index.saturating_sub(3);
    let raw = &input[start_index..index]
        .trim_matches(char::is_alphabetic)
        .trim();
    return raw.parse::<i32>().unwrap();
}

fn get_cubes_shown(input: &str) -> CubesShown {
    let mut result = CubesShown {
        red: 0,
        green: 0,
        blue: 0,
    };

    let red = input.find("red");
    let green = input.find("green");
    let blue = input.find("blue");

    if red.is_some() {
        result.red = get_cube_number(input, red.unwrap());
    }

    if green.is_some() {
        result.green = get_cube_number(input, green.unwrap());
    }

    if blue.is_some() {
        result.blue = get_cube_number(input, blue.unwrap());
    }

    return result;
}

fn get_power(input: &str) -> i32 {
    let start_of_subsets_index = input.find(":").unwrap();
    let raw_subsets = &input[start_of_subsets_index + 1..];
    let raw_cubes = raw_subsets.split(";");

    let mut max_cubes = CubesShown {
        red: 0,
        green: 0,
        blue: 0,
    };

    for item in raw_cubes {
        let cubes_shown = get_cubes_shown(&item);

        max_cubes.red = max(max_cubes.red, cubes_shown.red);
        max_cubes.green = max(max_cubes.green, cubes_shown.green);
        max_cubes.blue = max(max_cubes.blue, cubes_shown.blue);
    }

    return max_cubes.red * max_cubes.green * max_cubes.blue;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_get_cube_number() {
        assert_eq!(get_cube_number("1 red, 2 green, 6 blue", 2), 1);
        assert_eq!(get_cube_number("10 red, 2 green, 6 blue", 3), 10);
        assert_eq!(get_cube_number("1 red, 2 green, 6 blue", 9), 2);
        assert_eq!(get_cube_number("1 red, 20 green, 6 blue", 10), 20);
        assert_eq!(get_cube_number("1 red, 2 green, 6 blue", 18), 6);
        assert_eq!(get_cube_number("1 red, 2 green, 60 blue", 19), 60);
    }

    #[test]
    fn verify_get_cubes_shown() {
        assert_eq!(
            get_cubes_shown("1 red, 2 green, 3 blue"),
            CubesShown {
                red: 1,
                green: 2,
                blue: 3
            }
        );
        assert_eq!(
            get_cubes_shown("2 green, 3 blue"),
            CubesShown {
                red: 0,
                green: 2,
                blue: 3
            }
        );
        assert_eq!(
            get_cubes_shown("3 blue"),
            CubesShown {
                red: 0,
                green: 0,
                blue: 3
            }
        );
    }

    #[test]
    fn verify_is_game_possible() {
        assert_eq!(
            is_game_possible(
                12,
                13,
                14,
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
            ),
            true
        );
        assert_eq!(
            is_game_possible(
                12,
                13,
                14,
                "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"
            ),
            true
        );
        assert_eq!(
            is_game_possible(
                12,
                13,
                14,
                "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"
            ),
            false
        );
        assert_eq!(
            is_game_possible(
                12,
                13,
                14,
                "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"
            ),
            false
        );
        assert_eq!(
            is_game_possible(
                12,
                13,
                14,
                "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            ),
            true
        );
    }

    #[test]
    fn verify_get_power() {
        assert_eq!(
            get_power("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            48
        );
        assert_eq!(
            get_power("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"),
            12
        );
        assert_eq!(
            get_power("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"),
            1560
        );
        assert_eq!(
            get_power("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"),
            630
        );
        assert_eq!(
            get_power("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"),
            36
        );
    }
}
