use std::{cmp::Ordering, str::Chars};

pub struct Hand {
    hand: String,
    hand_type: HandType,
    card_value: u32,
    pub betted_funds: u32,
}

#[derive(PartialEq, PartialOrd, Ord, Eq, Debug, Clone, Copy)]
pub enum HandType {
    Five = 7,
    Four = 6,
    FullHouse = 5,
    Three = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

impl Hand {
    pub fn from(line: &str) -> Hand {
        let (cards, bet) = line.split_once(' ').unwrap();
        Hand::new(cards, bet.parse::<u32>().unwrap())
    }

    pub fn new(cards: &str, betted_funds: u32) -> Hand {
        let mut char_vec: Vec<char> = cards.chars().collect();
        let mut card_counts = vec![];

        while let Some(c) = char_vec.first() {
            let (matches, remainder): (Vec<char>, Vec<char>) =
                char_vec.iter().partition(|value| *value == c);
            card_counts.push(matches.len());
            char_vec = remainder;
        }

        // Sort and reverse to start with the highest element
        card_counts.sort();
        card_counts.reverse();

        let hand_type = match card_counts[..] {
            [5] => HandType::Five,
            [4, 1] => HandType::Four,
            [3, 2] => HandType::FullHouse,
            [3, 1, 1] => HandType::Three,
            [2, 2, 1] => HandType::TwoPair,
            [2, 1, 1, 1] => HandType::OnePair,
            _ => HandType::HighCard,
        };

        let card_value = cards
            .char_indices()
            .map(|(i, c)| {
                let value: u32 = match c {
                    'A' => 14,
                    'K' => 13,
                    'Q' => 12,
                    'J' => 11,
                    'T' => 10,
                    character => character.to_digit(10).unwrap(),
                };

                15u32.pow(4 - i as u32) * value
            })
            .sum::<u32>();

        Hand {
            hand: String::from(cards),
            hand_type,
            card_value,
            betted_funds,
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand == other.hand && self.hand_type == other.hand_type
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.hand_type != other.hand_type {
            return self.hand_type.partial_cmp(&other.hand_type);
        }

        self.card_value.partial_cmp(&other.card_value)
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.hand_type != other.hand_type {
            return self.hand_type.cmp(&other.hand_type);
        }

        self.card_value.cmp(&other.card_value)
    }
}

impl Eq for Hand {}

#[cfg(test)]
mod tests {
    use crate::{Hand, HandType};

    #[test]
    fn test_hand_creation() {
        let hand = Hand::new("AAAAA", 2);
        assert_eq!(hand.hand_type, HandType::Five);
        let hand = Hand::new("23AAA", 2);
        assert_eq!(hand.hand_type, HandType::Three);
        let hand = Hand::new("23456", 2);
        assert_eq!(hand.hand_type, HandType::HighCard);
        let hand = Hand::new("A232A", 2);
        assert_eq!(hand.hand_type, HandType::TwoPair);
    }
}
