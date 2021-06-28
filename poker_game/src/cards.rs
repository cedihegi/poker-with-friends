use std::cmp::Ordering;

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
    pub fn from_int(val: usize) -> Rank {
        match val {
            2 => Rank::Two,
            3 => Rank::Three,
            4 => Rank::Four,
            5 => Rank::Five,
            6 => Rank::Six,
            7 => Rank::Seven,
            8 => Rank::Eight,
            9 => Rank::Nine,
            10 => Rank::Ten,
            11 => Rank::Jack,
            12 => Rank::Queen,
            13 => Rank::King,
            14 => Rank::Ace,
            _ => panic!(),
        }
    }
    pub fn to_int(self) -> usize {
        self as usize
    }
    pub fn to_vec() -> Vec<Rank> {
        (2..15).map(|x| Rank::from_int(x)).collect()
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
    pub fn from_int(num: usize) -> Suit {
        match num {
            1 => Suit::Clubs,
            2 => Suit::Diamonds,
            3 => Suit::Hearts,
            4 => Suit::Spades,
            _ => panic!()
        }
    }
    pub fn to_int(self) -> usize {
        self as usize
    }
    fn to_vec() -> Vec<Suit> {
        (1..5).map(Suit::from_int).collect()
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
    pub fn from_tup(suit_int: usize, rank_int: usize) -> Card {
        let rank = Rank::from_int(rank_int);
        let suit = Suit::from_int(suit_int);
        Card {
            rank,
            suit
        }
    }
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

    pub fn suit_int(&self) -> usize {
        self.suit.clone().to_int()
    }

    pub fn rank_int(&self) -> usize {
        self.rank.clone().to_int()
    }
}