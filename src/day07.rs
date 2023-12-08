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

        let mut jocker_hand = parse(&actual_line);
        jocker_hand.apply_joker_rule();
        joker_hands.push(jocker_hand);
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
        ..Default::default()
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
    is_jocker_rule: bool,
}

impl Default for Hand {
    fn default() -> Self {
        Hand {
            cards: "".to_owned(),
            bid: 0,
            hand_type: HandType::HighCard,
            is_jocker_rule: false,
        }
    }
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
            let mut strengths: Vec<char> = vec![
                'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
            ];

            if self.is_jocker_rule {
                strengths = vec![
                    'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
                ];
            }

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

    fn apply_joker_rule(&mut self) {
        self.is_jocker_rule = true;
        let nr_jokers = self.cards.matches('J').count();
        if nr_jokers == 0 {
            return;
        }
        match self.hand_type {
            HandType::FourKind => self.hand_type = HandType::FiveKind,
            HandType::ThreeKind => {
                if nr_jokers == 1 {
                    self.hand_type = HandType::FourKind;
                } else if nr_jokers == 3 {
                    self.hand_type = HandType::FourKind;
                }
                // Don't need to check 2 as that would be a full house
            }
            HandType::FullHouse => {
                if nr_jokers >= 2 {
                    self.hand_type = HandType::FiveKind;
                }
            }
            HandType::TwoPair => {
                if nr_jokers == 1 {
                    self.hand_type = HandType::FullHouse;
                } else if nr_jokers == 2 {
                    self.hand_type = HandType::FourKind;
                }
                // Don't need to check 3 as that would be a full house
            }
            HandType::OnePair => {
                if nr_jokers == 1 {
                    self.hand_type = HandType::ThreeKind;
                } else if nr_jokers == 2 {
                    self.hand_type = HandType::ThreeKind;
                }
                // Don't need to check for 3 as that would be a five of a kind
            }
            HandType::HighCard => {
                if nr_jokers == 1 {
                    self.hand_type = HandType::OnePair;
                }
            }
            HandType::FiveKind => return,
        }
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
        assert_eq!(parse("32T3K 765"), Hand {cards: "32T3K".to_owned(), bid: 765, hand_type: HandType::OnePair, is_jocker_rule: false});
        assert_eq!(parse("T55J5 684"), Hand {cards: "T55J5".to_owned(), bid: 684, hand_type: HandType::ThreeKind, is_jocker_rule: false});
        assert_eq!(parse("KK677 28",),  Hand {cards: "KK677".to_owned(), bid: 28, hand_type: HandType::TwoPair, is_jocker_rule: false});
        assert_eq!(parse("KTJJT 220"), Hand {cards: "KTJJT".to_owned(), bid: 220, hand_type: HandType::TwoPair, is_jocker_rule: false});
        assert_eq!(parse("QQQJA 483"), Hand {cards: "QQQJA".to_owned(), bid: 483, hand_type: HandType::ThreeKind, is_jocker_rule: false});
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
    #[test]
    fn test_apply_joker() {
        let mut high_card = parse("A2345 000");
        let mut one_pair = parse("AA345 000");
        let mut two_pair = parse("AA335 000");
        let mut three = parse("AAA12 000");
        let mut four = parse("AAAA1 000");
        let mut five = parse("AAAAA 000");
        let mut full = parse("AAA22 000");
        high_card.apply_joker_rule();
        one_pair.apply_joker_rule();
        two_pair.apply_joker_rule();
        three.apply_joker_rule();
        four.apply_joker_rule();
        five.apply_joker_rule();
        full.apply_joker_rule();
        assert_eq!(high_card.hand_type, HandType::HighCard);
        assert_eq!(one_pair.hand_type, HandType::OnePair);
        assert_eq!(two_pair.hand_type, HandType::TwoPair);
        assert_eq!(three.hand_type, HandType::ThreeKind);
        assert_eq!(four.hand_type, HandType::FourKind);
        assert_eq!(five.hand_type, HandType::FiveKind);
        assert_eq!(full.hand_type, HandType::FullHouse);

        let mut high_card_with_j = parse("A234J 000");
        let mut one_pair_with_j = parse("AA34J 000");
        let mut two_pair_with_j = parse("AA33J 000");
        let mut three_with_j = parse("AAA1J 000");
        let mut four_with_j = parse("AAAAJ 000");
        let mut full_with_j = parse("AAAJJ 000");
        let mut full_with_j2 = parse("AAJJJ 000");
        high_card_with_j.apply_joker_rule();
        one_pair_with_j.apply_joker_rule();
        two_pair_with_j.apply_joker_rule();
        three_with_j.apply_joker_rule();
        four_with_j.apply_joker_rule();
        full_with_j.apply_joker_rule();
        full_with_j2.apply_joker_rule();
        assert_eq!(high_card_with_j.hand_type, HandType::OnePair);
        assert_eq!(one_pair_with_j.hand_type, HandType::ThreeKind);
        assert_eq!(two_pair_with_j.hand_type, HandType::FullHouse);
        assert_eq!(three_with_j.hand_type, HandType::FourKind);
        assert_eq!(four_with_j.hand_type, HandType::FiveKind);
        assert_eq!(full_with_j.hand_type, HandType::FiveKind);
        assert_eq!(full_with_j2.hand_type, HandType::FiveKind);

        let mut jokers = parse("JJJJJ, 000");
        let mut jokers2 = parse("JJJJ1, 000");
        let mut jokers3 = parse("JJJ21, 000");
        let mut jokers4 = parse("JJ321, 000");
        let mut jokers5 = parse("J4321, 000");
        jokers.apply_joker_rule();
        jokers2.apply_joker_rule();
        jokers3.apply_joker_rule();
        jokers4.apply_joker_rule();
        jokers5.apply_joker_rule();
        assert_eq!(jokers.hand_type, HandType::FiveKind);
        assert_eq!(jokers2.hand_type, HandType::FiveKind);
        assert_eq!(jokers3.hand_type, HandType::FourKind);
        assert_eq!(jokers4.hand_type, HandType::ThreeKind);
        assert_eq!(jokers5.hand_type, HandType::OnePair);

        let mut special = parse("Q2KJJ 000");
        special.apply_joker_rule();
        assert_eq!(special.hand_type, HandType::ThreeKind);

    }
}
