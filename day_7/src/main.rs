use std::{io::{stdin, BufRead}, collections::{HashMap, hash_map::Entry}};

fn main() {
    let mut hands = parse_hands(stdin().lock());
    hands.sort();
    // println!("{:#?}", &hands);
    let total_winnings: usize = hands
        .iter()
        .enumerate()
        .map(|(i, hand)| hand.bet * (i + 1))
        .sum();
    println!("{total_winnings}")
}

fn parse_hands(input: impl BufRead) -> Vec<Hand> {
    let mut hands: Vec<Hand> = vec![];
    for line_result in input.lines() {
        let line = line_result.unwrap();
        hands.push(Hand::from_str(&line))
    }
    hands
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    stack: CardStack,
    bet: usize,
    hand_type: HandType,
}

impl Hand {
    fn from_str(s: &str) -> Hand {
        let mut split = s.split(' ');
        let stack = CardStack::from_str(split.next().unwrap());
        let bet = split.next().unwrap().parse::<usize>().unwrap();
        let hand_type = HandType::from_card_stack(&stack.0);
        Hand {
            stack,
            bet,
            hand_type,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.hand_type.partial_cmp(&other.hand_type) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.stack.partial_cmp(&other.stack)
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.hand_type.cmp(&other.hand_type)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct CardStack(Vec<Card>);

impl CardStack {
    fn from_str(s: &str) -> CardStack {
        CardStack(s.chars().map(Card::from_char).collect())
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Card {
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Joker = 11,
    Queen = 12,
    King = 13,
    Ace = 14,
}

impl Card {
    fn from_char(c: char) -> Card {
        match c {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'J' => Card::Joker,
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            _ => panic!("{} not a card character", c),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum HandType {
    HighCard = 0,
    OnePair = 1,
    TwoPairs = 2,
    ThreeOfAKind = 3,
    FullHouse = 4,
    FourOfAKind = 5,
    FiveOfAKind = 6,
}

impl HandType {
    fn value(&self) -> u8 {
        *self as u8
    }

    fn from_card_stack(stack: &[Card]) -> HandType {
        let mut combinations_lookup: HashMap<Card, usize> = HashMap::new();
        for &card in stack {
            match combinations_lookup.entry(card) {
                Entry::Vacant(entry) => {
                    entry.insert(1);
                },
                Entry::Occupied(mut entry) => *entry.get_mut() += 1,
            }
        }

        let mut combinations: Vec<_> = combinations_lookup.iter().collect();
        combinations.sort_by_key(|x| std::cmp::Reverse(x.1));

        /*
        println!("{stack:?}");
        for combination in combinations {
            println!("{combination:?}");
        }
        */

        if *combinations[0].1 == 5 {
            HandType::FiveOfAKind
        } else if *combinations[0].1 == 4 {
            HandType::FourOfAKind
        } else if combinations.len() > 1 && *combinations[0].1 == 3 && *combinations[1].1 == 2 {
            HandType::FullHouse
        } else if *combinations[0].1 == 3 {
            HandType::ThreeOfAKind
        } else if combinations.len() > 1 && *combinations[0].1 == 2 && *combinations[1].1 == 2 {
            HandType::TwoPairs
        } else if *combinations[0].1 == 2 {
            HandType::OnePair
        } else {
            HandType::HighCard
        }
    }
}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value().partial_cmp(&other.value())
    }
}

impl Ord for HandType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value().cmp(&other.value())
    }
}
