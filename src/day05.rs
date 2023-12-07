use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn day05() -> std::io::Result<()> {
    println!("Day 5");

    let file = File::open("src/input5.txt")?;
    let reader = BufReader::new(file);

    let mut lines: Vec<String> = Vec::new();

    for line in reader.lines() {
        lines.push(line.unwrap());
    }

    let mut seed_to_soil = Map { ranges: vec![] };
    let mut soil_to_fertilizer = Map { ranges: vec![] };
    let mut fertilizer_to_water = Map { ranges: vec![] };
    let mut water_to_light = Map { ranges: vec![] };
    let mut light_to_temp = Map { ranges: vec![] };
    let mut temp_to_hum = Map { ranges: vec![] };
    let mut hum_to_loc = Map { ranges: vec![] };

    for (index, line) in lines.iter().enumerate() {
        if index > 2 && index < 38 {
            seed_to_soil.ranges.push(parse(&line));
        }
        if index > 39 && index < 73 {
            soil_to_fertilizer.ranges.push(parse(&line));
        }
        if index > 74 && index < 103 {
            fertilizer_to_water.ranges.push(parse(&line));
        }
        if index > 104 && index < 120 {
            water_to_light.ranges.push(parse(&line));
        }
        if index > 121 && index < 154 {
            light_to_temp.ranges.push(parse(&line));
        }
        if index > 155 && index < 188 {
            temp_to_hum.ranges.push(parse(&line));
        }
        if index > 189 && index < 206 {
            hum_to_loc.ranges.push(parse(&line));
        }
    }
    let seeds: Vec<u128> = vec![
        1367444651, 99920667, 3319921504, 153335682, 67832336, 139859832, 2322838536, 666063790,
        1591621692, 111959634, 442852010, 119609663, 733590868, 56288233, 2035874278, 85269124,
        4145746192, 55841637, 864476811, 347179760,
    ];
    let mut locations: Vec<u128> = Vec::new();

    for seed in &seeds {
        locations.push(hum_to_loc.lookup(temp_to_hum.lookup(light_to_temp.lookup(
            water_to_light.lookup(
                fertilizer_to_water.lookup(soil_to_fertilizer.lookup(seed_to_soil.lookup(*seed))),
            ),
        ))));
    }

    let answer_one = *locations.iter().min().unwrap();
    // Part two
    locations.clear();
    let mut min_of_each_range: Vec<u128> = Vec::new();

    let mut i: usize = 0;
    while i < seeds.len() {
        println!("Range start {}", i);
        let mut range: Vec<u128> = Vec::from_iter(seeds[i]..seeds[i] + seeds[i + 1]);
        let mut lowest: u128 = u128::MAX;
        for seed in &range {
            lowest = lowest.min(
                hum_to_loc.lookup(
                    temp_to_hum.lookup(
                        light_to_temp.lookup(
                            water_to_light.lookup(
                                fertilizer_to_water
                                    .lookup(soil_to_fertilizer.lookup(seed_to_soil.lookup(*seed))),
                            ),
                        ),
                    ),
                ),
            );
        }
        min_of_each_range.push(lowest);
        range.clear();
        locations.clear();
        i += 2;
    }
    // Only took like an hour LOL

    println!(
        "Answer 1: {}, 2: {}",
        answer_one,
        min_of_each_range.iter().min().unwrap()
    );
    Ok(())
}

fn parse(input: &str) -> Range {
    let mut range = Range {
        dest_start: 0,
        src_start: 0,
        len: 0,
    };

    let mut values = input.split_ascii_whitespace();
    range.dest_start = values.next().unwrap().parse::<u128>().unwrap();
    range.src_start = values.next().unwrap().parse::<u128>().unwrap();
    range.len = values.next().unwrap().parse::<u128>().unwrap();

    return range;
}

#[derive(PartialEq, Debug)]
struct Range {
    dest_start: u128,
    src_start: u128,
    len: u128,
}

impl Range {
    fn get_mapped_value(&self, value: u128) -> u128 {
        if value >= self.src_start && value < self.src_start + self.len {
            if self.src_start < self.dest_start {
                return value + self.src_start.abs_diff(self.dest_start);
            } else {
                return value - self.src_start.abs_diff(self.dest_start);
            }
        }
        return value;
    }
}

#[derive(PartialEq, Debug)]
struct Map {
    ranges: Vec<Range>,
}

impl Map {
    fn lookup(&self, value: u128) -> u128 {
        let mut result: u128 = value;
        for range in &self.ranges {
            result = range.get_mapped_value(result);
            if result != value {
                break;
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
    fn test_parse() {
        assert_eq!(parse("50 98 2"), Range { dest_start: 50, src_start: 98, len: 2 });
        assert_eq!(parse("52 50 48"), Range { dest_start: 52, src_start: 50, len: 48 });

        assert_eq!(parse("374586838 0 164719538"), Range { dest_start: 374586838, src_start: 0, len: 164719538 });
        assert_eq!(parse("1923454454 1644399397 1264527167"), Range { dest_start: 1923454454, src_start: 1644399397, len: 1264527167 });
    }

    #[test]
    fn test_get_mapped_value() {
        assert_eq!(Range { dest_start: 50, src_start: 98, len: 2 }.get_mapped_value(98), 50);
        assert_eq!(Range { dest_start: 50, src_start: 98, len: 2 }.get_mapped_value(99), 51);
        assert_eq!(Range { dest_start: 50, src_start: 98, len: 2 }.get_mapped_value(97), 97);
        assert_eq!(Range { dest_start: 50, src_start: 98, len: 2 }.get_mapped_value(100), 100);
        
        assert_eq!(Range { dest_start: 52, src_start: 50, len: 48 }.get_mapped_value(79), 81);
    }

    #[test]
    fn test_lookup() {
        let seed_to_soil = Map{ranges: vec![Range { dest_start: 50, src_start: 98, len: 2 },
                                                 Range { dest_start: 52, src_start: 50, len: 48 }]};
        assert_eq!(seed_to_soil.lookup(79), 81);
        assert_eq!(seed_to_soil.lookup(14), 14);
        assert_eq!(seed_to_soil.lookup(55), 57);
        assert_eq!(seed_to_soil.lookup(13), 13);

        let fertilizer_to_water = Map{ranges: vec![Range { dest_start: 49, src_start: 53, len: 8 },
                                        Range { dest_start: 0, src_start: 11, len: 42 },
                                        Range { dest_start: 42, src_start: 0, len: 7 },
                                        Range { dest_start: 57, src_start: 7, len: 4 }]};
        assert_eq!(fertilizer_to_water.lookup(53), 49);
    }

    #[test]
    fn test_examples() {
        let seed_to_soil = Map{ranges: vec![Range { dest_start: 50, src_start: 98, len: 2 },
                                                 Range { dest_start: 52, src_start: 50, len: 48 }]};

        let soil_to_fertilizer = Map{ranges: vec![Range { dest_start: 0, src_start: 15, len: 37 },
                                                       Range { dest_start: 37, src_start: 52, len: 2 },
                                                       Range { dest_start: 39, src_start: 0, len: 15 }]};

        let fertilizer_to_water = Map{ranges: vec![Range { dest_start: 49, src_start: 53, len: 8 },
                                                        Range { dest_start: 0, src_start: 11, len: 42 },
                                                        Range { dest_start: 42, src_start: 0, len: 7 },
                                                        Range { dest_start: 57, src_start: 7, len: 4 }]};

        let water_to_light = Map{ranges: vec![Range { dest_start: 88, src_start: 18, len: 7 },
                                                   Range { dest_start: 18, src_start: 25, len: 70 }]};

        let light_to_temp = Map{ranges: vec![Range { dest_start: 45, src_start: 77, len: 23 },
                                                Range { dest_start: 81, src_start: 45, len: 19 },
                                                Range { dest_start: 68, src_start: 64, len: 13 }]};
        
        let temp_to_hum = Map{ranges: vec![Range { dest_start: 0, src_start: 69, len: 1 },
                                                Range { dest_start: 1, src_start: 0, len: 69 }]};                                                
        
        let hum_to_loc = Map{ranges: vec![Range { dest_start: 60, src_start: 56, len: 37 },
                                               Range { dest_start: 56, src_start: 93, len: 4 }]};
        
        let seeds: Vec<u128> = vec![79, 14, 55, 13];
        let mut locations: Vec<u128> = Vec::new();

        for seed in seeds{
            print!("seed: {}", seed);
            let mut result = seed_to_soil.lookup(seed);
            print!(" soil: {}", result);
            result = soil_to_fertilizer.lookup(result);
            print!(" fert: {}", result);
            result = fertilizer_to_water.lookup(result);
            print!(" water: {}", result);
            result = water_to_light.lookup(result);
            print!(" light: {}", result);
            result = light_to_temp.lookup(result);
            print!(" temp: {}", result);
            result = temp_to_hum.lookup(result);
            print!(" hum: {}", result);
            result = hum_to_loc.lookup(result);
            print!(" hum: {}", result);
            locations.push(result);
            println!();
        }
        assert_eq!(locations[0], 82);
        assert_eq!(locations[1], 43);
        assert_eq!(locations[2], 86);
        assert_eq!(locations[3], 35);

        // Part two
        locations.clear();
        let range_one: Vec<u128> = (79..79+14).collect();
        let range_two: Vec<u128> = (55 ..55+13).collect();

        for seed in range_one {
            locations.push(hum_to_loc.lookup(temp_to_hum.lookup(light_to_temp.lookup(
                water_to_light.lookup(
                    fertilizer_to_water.lookup(soil_to_fertilizer.lookup(seed_to_soil.lookup(seed))),
                ),
            ))));
        }

        for seed in range_two {
            locations.push(hum_to_loc.lookup(temp_to_hum.lookup(light_to_temp.lookup(
                water_to_light.lookup(
                    fertilizer_to_water.lookup(soil_to_fertilizer.lookup(seed_to_soil.lookup(seed))),
                ),
            ))));
        }
        
        assert_eq!(locations.iter().min().unwrap(), &46u128);

    }

}
