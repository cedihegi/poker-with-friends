use crate::cards::{Card, Rank, Suit};

pub enum Hands {
    RoyalFlush(Suit),
    StraightFlush(Rank),
    FourOfKind(Rank),
    FullHouse(Rank, Rank),
    Flush(Vec<Card>),
    Straight(Rank),
    ThreeOfKind(Rank),
    TwoPair(Rank, Rank),
    Pair(Rank),
    HighCard(Vec<Rank>),
}