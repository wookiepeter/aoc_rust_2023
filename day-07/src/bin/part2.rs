use day_07::HandType;

fn main() {
    let input = include_str!("./input2.txt");
    let output = process(input);
    dbg!(output);
}

#[derive(Debug)]
pub struct JokerHand {
    hand: String,
    hand_type_joker: HandType,
    card_value: u32,
    pub betted_funds: u32,
}

impl JokerHand {
    pub fn from(line: &str) -> JokerHand {
        let (cards, bet) = line.split_once(' ').unwrap();
        JokerHand::new(cards, bet.parse::<u32>().unwrap())
    }

    pub fn new(cards: &str, betted_funds: u32) -> JokerHand {
        let hand_type_joker = compute_hand_type(cards);

        let card_value = cards
            .char_indices()
            .map(|(i, c)| {
                let value: u32 = match c {
                    'A' => 14,
                    'K' => 13,
                    'Q' => 12,
                    'J' => 1,
                    'T' => 10,
                    character => character.to_digit(10).unwrap(),
                };

                15u32.pow(4 - i as u32) * value
            })
            .sum::<u32>();

        JokerHand {
            hand: String::from(cards),
            hand_type_joker,
            card_value,
            betted_funds,
        }
    }
}

fn compute_hand_type(cards: &str) -> HandType {
    let mut card_counts = vec![];

    let mut char_vec: Vec<char> = cards.chars().filter(|c| *c != 'J').collect();
    let joker_count = cards.chars().filter(|c| *c == 'J').count();

    if joker_count == 5 {
        return HandType::Five;
    }

    while let Some(c) = char_vec.first() {
        let (matches, remainder): (Vec<char>, Vec<char>) =
            char_vec.iter().partition(|value| *value == c);
        card_counts.push(matches.len());
        char_vec = remainder;
    }

    // Sort and reverse to start with the highest element
    card_counts.sort();
    card_counts.reverse();

    card_counts[0] += joker_count;

    match card_counts[..] {
        [5] => HandType::Five,
        [4, 1] => HandType::Four,
        [3, 2] => HandType::FullHouse,
        [3, 1, 1] => HandType::Three,
        [2, 2, 1] => HandType::TwoPair,
        [2, 1, 1, 1] => HandType::OnePair,
        _ => HandType::HighCard,
    }
}

impl PartialEq for JokerHand {
    fn eq(&self, other: &Self) -> bool {
        self.hand == other.hand
    }
}

impl PartialOrd for JokerHand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for JokerHand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.hand_type_joker != other.hand_type_joker {
            return self.hand_type_joker.cmp(&other.hand_type_joker);
        }

        self.card_value.cmp(&other.card_value)
    }
}

impl Eq for JokerHand {}

fn process(input: &str) -> String {
    let mut hands: Vec<JokerHand> = input.lines().map(JokerHand::from).collect();

    hands.sort();

    dbg!(&hands);

    hands
        .iter()
        .enumerate()
        .map(|(rank, hand)| (rank as u32 + 1) * hand.betted_funds)
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use crate::process;

    #[test]
    fn test_example() {
        let result = process(
            "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483",
        );
        assert_eq!(result, "5905".to_string())
    }
}
