pub fn day06() -> std::io::Result<()> {
    println!("Day 6");

    /*
    Time:        55     99     97     93
    Distance:   401   1485   2274   1405

     */
    let race_1 = Race {
        time: 55,
        record: 401,
    };
    let race_2 = Race {
        time: 99,
        record: 1485,
    };
    let race_3 = Race {
        time: 97,
        record: 2274,
    };
    let race_4 = Race {
        time: 93,
        record: 1405,
    };

    let answer_one: u64 = race_1.get_nr_record_beaters()
        * race_2.get_nr_record_beaters()
        * race_3.get_nr_record_beaters()
        * race_4.get_nr_record_beaters();

    // Part two
    let race_5 = Race {
        time: 55999793,
        record: 401148522741405,
    };

    println!(
        "Answer 1: {}, 2: {}",
        answer_one,
        race_5.get_nr_record_beaters()
    );
    Ok(())
}

struct Race {
    time: u64,
    record: u64,
}

impl Race {
    fn get_nr_record_beaters(&self) -> u64 {
        let mut lowest: u64 = 1;
        let mut highest: u64 = self.time - 1;
        let mut lowest_record: u64 = 0;
        let mut highest_record: u64 = 0;

        while highest > lowest {
            let low_distance: u64 = (self.time - lowest) * lowest;
            let high_distance: u64 = (self.time - highest) * highest;

            if low_distance > self.record && low_distance > lowest_record {
                lowest_record = low_distance;
            }

            if high_distance > self.record && high_distance > highest_record {
                highest_record = high_distance;
            }

            if lowest_record != 0 && highest_record != 0 {
                break;
            }

            lowest += 1;
            highest -= 1;
        }

        return highest - lowest + 1;
    }
}

#[rustfmt::skip]
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_get_nr_record_beaters() {
        assert_eq!(Race {time: 7, record: 9}.get_nr_record_beaters(), 4);
        assert_eq!(Race {time: 15, record: 40}.get_nr_record_beaters(), 8);
        assert_eq!(Race {time: 30, record: 200}.get_nr_record_beaters(), 9);
        assert_eq!(Race {time: 71530, record: 940200}.get_nr_record_beaters(), 71503);
    }

}
