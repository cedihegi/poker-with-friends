use crate::cards::{Card, Rank, Suit};

pub enum Hands {
    RoyalFlush(Suit),
    StraightFlush(Rank), //contains highest card's rank 
    FourOfKind(Rank, Card), //contains rank of those 4 cards -> can never draw! 
    FullHouse(Rank, Rank), // contains rank of upper 3 cards and lower 2, but only upper 3 matter for ordering
    Flush(Vec<Card>), // contains all 5 cards, here it actually matters
    Straight(Rank), // contains highest card of straight
    ThreeOfKind(Rank), // contains rank of triplet, other cards will never matter
    TwoPair(Rank, Rank, Rank), //contains rank of higher pair, lower pair, and other card
    Pair(Rank, Vec<Rank>), // Rank of pair, should also contain ranks of the 3 other cards
    HighCard(Vec<Rank>), // contains ranks of all 5 cards
}
