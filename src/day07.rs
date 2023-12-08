use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn day07() -> std::io::Result<()> {
    println!("Day 7");

    let file = File::open("src/input7.txt")?;
    let reader = BufReader::new(file);

    let mut answer_one: usize = 0;
    let mut answer_two: usize = 0;

    let mut hands: Vec<Hand> = Vec::with_capacity(1000);
    let mut joker_hands: Vec<Hand> = Vec::with_capacity(1000);

    for (_index, line) in reader.lines().enumerate() {
        let actual_line = line.unwrap();
        hands.push(parse(&actual_line));
        joker_hands.push(parse(&actual_line));
    }

    hands.sort();
    joker_hands.sort();

    for (index, hand) in hands.iter().enumerate() {
        answer_one += hand.bid * (index + 1);
    }

    for (index, joker_hand) in joker_hands.iter().enumerate() {
        answer_two += joker_hand.bid * (index + 1);
    }

    println!("Answer 1: {}, 2: {}", answer_one, answer_two);
    Ok(())
}

fn parse(line: &str) -> Hand {
    let mut hand = Hand {
        cards: "".to_owned(),
        bid: 0,
        hand_type: HandType::OnePair,
    };

    let mut parts = line.split_ascii_whitespace();
    hand.cards = parts.next().unwrap().to_string();
    hand.bid = parts.next().unwrap().parse::<usize>().unwrap();
    hand.hand_type = get_type(&hand.cards);

    return hand;
}

#[derive(Debug, PartialEq, PartialOrd)]
enum HandType {
    FiveKind = 7,
    FourKind = 6,
    FullHouse = 5,
    ThreeKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

#[derive(Debug, PartialEq)]
struct Hand {
    cards: String,
    bid: usize,
    hand_type: HandType,
}

fn get_type(cards: &str) -> HandType {
    let mut result = HandType::HighCard;

    let map = cards.chars().fold(HashMap::new(), |mut acc, c| {
        *acc.entry(c).or_insert(0) += 1;
        acc
    });

    let max = map.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap();
    let min = map.iter().min_by(|a, b| a.1.cmp(&b.1)).unwrap();

    if max.1 == &5 {
        result = HandType::FiveKind;
    } else if max.1 == &4 {
        result = HandType::FourKind;
    } else if max.1 == &3 && min.1 == &2 {
        result = HandType::FullHouse;
    } else if max.1 == &3 {
        result = HandType::ThreeKind;
    } else if max.1 == &2 {
        result = HandType::OnePair;
        for card in map.iter() {
            if card.0 != max.0 && card.1 == &2 {
                result = HandType::TwoPair;
                break;
            }
        }
    }

    return result;
}

impl Hand {
    fn is_stronger(&self, other: &Hand) -> bool {
        if self.hand_type != other.hand_type {
            return self.hand_type > other.hand_type;
        } else {
            let strengths: Vec<char> = vec![
                'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
            ];

            for i in 0..self.cards.len() {
                let our: char = self.cards.chars().nth(i).unwrap();
                let them: char = other.cards.chars().nth(i).unwrap();
                if our != them {
                    return strengths.iter().position(|&x| x == our).unwrap()
                        < strengths.iter().position(|&x| x == them).unwrap();
                }
            }
        }

        return false;
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.is_stronger(other) {
            return Some(std::cmp::Ordering::Greater);
        }
        return Some(std::cmp::Ordering::Less);
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.is_stronger(other) {
            return std::cmp::Ordering::Greater;
        }
        return std::cmp::Ordering::Less;
    }
}

impl Eq for Hand {
    // Satisy the compiler, but we don't need this.
}

#[rustfmt::skip]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(parse("32T3K 765"), Hand {cards: "32T3K".to_owned(), bid: 765, hand_type: HandType::OnePair});
        assert_eq!(parse("T55J5 684"), Hand {cards: "T55J5".to_owned(), bid: 684, hand_type: HandType::ThreeKind});
        assert_eq!(parse("KK677 28",),  Hand {cards: "KK677".to_owned(), bid: 28, hand_type: HandType::TwoPair});
        assert_eq!(parse("KTJJT 220"), Hand {cards: "KTJJT".to_owned(), bid: 220, hand_type: HandType::TwoPair});
        assert_eq!(parse("QQQJA 483"), Hand {cards: "QQQJA".to_owned(), bid: 483, hand_type: HandType::ThreeKind});
    }

    #[test]
    fn test_is_stronger() {
        let five_kind = parse("AAAAA 000");
        let high_card = parse("A2345 000");
        assert_eq!(five_kind.is_stronger(&high_card), true);
        assert_eq!(high_card.is_stronger(&five_kind), false);

        let four_kind_1 = parse("AAAAQ 000");
        let four_kind_2 = parse("AAAAJ 000");
        assert_eq!(four_kind_1.is_stronger(&four_kind_2), true);
        assert_eq!(four_kind_2.is_stronger(&four_kind_1), false);
    }
}
