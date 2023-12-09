use std::{
    cmp::{Ordering, Reverse},
    collections::HashMap,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
enum Card {
    J = 0,
    N2 = 1,
    N3 = 2,
    N4 = 3,
    N5 = 4,
    N6 = 5,
    N7 = 6,
    N8 = 7,
    N9 = 8,
    T = 9,
    Q = 10,
    K = 11,
    A = 12,
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            'A' => Self::A,
            'K' => Self::K,
            'Q' => Self::Q,
            'J' => Self::J,
            'T' => Self::T,
            '9' => Self::N9,
            '8' => Self::N8,
            '7' => Self::N7,
            '6' => Self::N6,
            '5' => Self::N5,
            '4' => Self::N4,
            '3' => Self::N3,
            '2' => Self::N2,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
enum HandKind {
    HighCard = 0,
    OnePair = 1,
    TwoPairs = 2,
    ThreeOfAKind = 3,
    FullHouse = 4,
    FourOfAKind = 5,
    FiveOfAKind = 6,
}

impl From<&Vec<Card>> for HandKind {
    fn from(value: &Vec<Card>) -> Self {
        let mut groups = HashMap::new();

        let jokers_count = value.iter().filter(|&c| *c == Card::J).count();
        if jokers_count == 5 {
            return HandKind::FiveOfAKind;
        }

        for v in value.iter().filter(|&c| *c != Card::J) {
            *groups.entry(v).or_insert(0) += 1;
        }

        let mut group_values = groups.values().cloned().collect::<Vec<usize>>();
        group_values.sort_by_key(|k| Reverse(*k));
        group_values[0] += jokers_count;

        match groups.len() {
            5 => HandKind::HighCard,
            4 => HandKind::OnePair,
            3 if group_values.iter().filter(|&v| *v == 2).count() == 2 => HandKind::TwoPairs,
            3 => HandKind::ThreeOfAKind,
            2 if group_values.iter().filter(|&v| *v == 3).count() == 1 => HandKind::FullHouse,
            2 => HandKind::FourOfAKind,
            1 => HandKind::FiveOfAKind,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd)]
struct Hand {
    kind: HandKind,
    cards: Vec<Card>,
}

impl From<&str> for Hand {
    fn from(value: &str) -> Self {
        let cards = value.chars().map(|c| c.into()).collect::<Vec<Card>>();

        Self {
            kind: (&cards).into(),
            cards,
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let mut result = self.kind.cmp(&other.kind);

        if result.is_eq() {
            result = self.cards.cmp(&other.cards);
        }

        result
    }
}

fn main() {
    let mut hands = include_str!("input01.txt")
        .lines()
        .map(|l| {
            let splits = l.split(' ').collect::<Vec<&str>>();
            (splits[0].into(), splits[1].parse::<u64>().unwrap())
        })
        .collect::<Vec<(Hand, u64)>>();

    hands.sort_by(|a, b| b.0.cmp(&a.0));

    let result = hands
        .iter()
        .enumerate()
        .map(|(i, v)| v.1 * (hands.len() - i) as u64)
        .sum::<u64>();

    println!("result = {:?}", result);
}
