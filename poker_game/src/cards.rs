use std::{cmp::Ordering, str};

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug, Clone)]
pub enum Rank {
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
    Ace = 14,
}

impl Rank {
    pub fn to_vec() -> Vec<Rank> {
        vec![
            Rank::Two,
            Rank::Three,
            Rank::Four,
            Rank::Five,
            Rank::Six,
            Rank::Seven,
            Rank::Eight,
            Rank::Nine,
            Rank::Ten,
            Rank::Jack,
            Rank::Queen,
            Rank::King,
            Rank::Ace,
        ]
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Suit {
    Clubs = 1,
    Diamonds = 2,
    Hearts = 3,
    Spades = 4,
}

impl Suit {
    fn to_vec() -> Vec<Suit> {
        vec![
            Suit::Clubs,
            Suit::Diamonds,
            Suit::Hearts,
            Suit::Spades,
        ]
    }
}

#[derive(Eq, Debug, Clone)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.rank.cmp(&other.rank)
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.rank == other.rank
    }
}

impl Card {
    pub fn to_vec() -> Vec<Card> {
        let ranks = Rank::to_vec();
        let suits = Suit::to_vec();
        let mut deck = Vec::new();
        for suit in &suits {
            for rank in &ranks {
                let card = Card { suit: suit.clone(), rank: rank.clone() };
                deck.push(card);
            }
        }
        deck
    }
}