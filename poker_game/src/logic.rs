use crate::cards::{Card, Rank, Suit};

pub enum Hand {
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

// How do we evaluate a set of 7 cards? First, what is the result of 
// an evaluation?

pub fn evaluate(hand: &Vec<Card>, community: &Vec<Card>) -> Hand {
    assert!(hand.len() == 2);
    assert!(community.len() == 5);

    let mut combined = hand.clone();
    combined.append(&mut community.clone());
    combined.sort_by(|a, b| b.cmp(a));

    println!("Sorted Cards: {:?}", combined);

    //now find the best hand:
    // to do this we create a few extra structures:
    let mut ranks = vec![0;14];
    for card in &combined {
        let disc = card.rank.clone() as usize;
        ranks[disc] += 1;
    }

    

    return Hand::HighCard(vec!());
}