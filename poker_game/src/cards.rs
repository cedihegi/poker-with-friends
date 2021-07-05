use std::cmp::Ordering;
use std::fmt;

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
            _ => panic!("not a valid number for a card-rank"),
        }
    }

    pub fn to_int(&self) -> usize {
        self.clone() as usize
    }
    pub fn to_vec() -> Vec<Rank> {
        (2..15).map(|x| Rank::from_int(x)).collect()
    }

}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
           Rank::Two => write!(f, "2"),
           Rank::Three => write!(f, "3"),
           Rank::Four => write!(f, "4"),
           Rank::Five => write!(f, "5"),
           Rank::Six => write!(f, "6"),
           Rank::Seven => write!(f, "7"),
           Rank::Eight => write!(f, "8"),
           Rank::Nine => write!(f, "9"),
           Rank::Ten => write!(f, "10"),
           Rank::Jack => write!(f, "J"),
           Rank::Queen => write!(f, "Q"),
           Rank::King => write!(f, "K"),
           Rank::Ace => write!(f, "A"), 
        }
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
    pub fn to_int(&self) -> usize {
        self.clone() as usize
    }
    fn to_vec() -> Vec<Suit> {
        (1..5).map(Suit::from_int).collect()
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Suit::Clubs => write!(f, "♣"),
            Suit::Diamonds => write!(f, "♦"),
            Suit::Hearts => write!(f, "♥"),
            Suit::Spades => write!(f, "♠"),
        }
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

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.rank, self.suit)
    }
}