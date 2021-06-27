use std::usize;

use crate::cards::{Card, Rank, Suit};

pub enum Value {
    RoyalFlush,
    StraightFlush(Rank), //contains highest card's rank 
    FourOfKind(Rank), //contains rank of those 4 cards -> can never draw! 
    FullHouse(Rank, Rank), // contains rank of upper 3 cards and lower 2, but only upper 3 matter for ordering
    Flush(Vec<Card>), // contains all 5 cards, here it actually matters
    Straight(Rank), // contains highest card of straight
    ThreeOfKind(Rank), // contains rank of triplet, other cards will never matter
    TwoPair(Rank, Rank, Rank), //contains rank of higher pair, lower pair, and other card
    Pair(Rank, Vec<Rank>), // Rank of pair, should also contain ranks of the 3 other cards
    HighCard(Vec<Rank>), // contains ranks of all 5 cards
}

// How do we evaluate a set of 7 cards? First, what is the result of 
// an evaluation? the best possible interpretation. How to find this easily?

#[derive(Debug)]
pub struct Hand {
    cards: Vec<Card>,
    ranks: Vec<usize>,
    matrix: Vec<Vec<bool>>,
    quads: Option<usize>,
    trips: usize,
    tuples: usize,
}

impl Hand {
    //initialize
    pub fn new(cards: Vec<Card>) -> Hand {
        let mut matrix = vec![vec![false; 15]; 5];
        let mut ranks = vec![0; 15];
        assert!(cards.len() == 7);
        for card in &cards {
            let rank = card.rank.clone() as usize;
            let suit = card.suit.clone() as usize;
            matrix[suit][rank] = true;
            ranks[rank] += 1;

            if rank == 14 {
                matrix[suit][1] = true;
                ranks[1] += 1;
            }
        }
        let mut cards_sorted = cards.clone();
        cards_sorted.sort_by(|a, b| b.cmp(a));

        let mut quads = None;
        let mut trips = 0;
        let mut tuples = 0;

        for rank in 2..15 {
            let quant = ranks[rank];
            if quant == 4 {
                quads = Some(rank);
            }
            if quant >= 3 {
                trips += 1;
            }
            if quant >= 2 {
                tuples += 1;
            }
        }
        let res = Hand {
            cards: cards_sorted,
            ranks,
            matrix,
            quads,
            trips,
            tuples
        };
        println!("created new hand: {:?}", res);
        res
    }

    pub fn new_split(hand: &Vec<Card>, community: &Vec<Card>) -> Hand {
        assert!(hand.len() == 2);
        assert!(community.len() == 5);

        let mut combined = hand.clone();
        combined.append(&mut community.clone());
        Hand::new(combined)
    }
}

impl Hand {
    //analyze
    pub fn top_tier(&self) -> Option<(Value, Vec<Card>)> {
        for suit in 1..5 {
            match Self::straight_in_vec(&self.matrix[suit]) {
                Ok(start) => {
                    let mut cards = vec![];
                    for i in 0..5 {
                        let card = Card::from_tup(start - i, suit);
                        cards.push(card);
                    }
                    if start == 14 {
                        return Some((Value::RoyalFlush, cards))
                    } else {
                        let result = Value::StraightFlush(Rank::from_int(start));
                        return Some((result, cards))
                    }
                },
                Err(()) => {},
            }
        }
        None
    }

    pub fn semi_tier(&self) -> Option<(Value, Vec<Card>)> {
        if let Some(quad_rank) = self.quads {
            let mut cards = vec![];
            for i in 1..5 {
                let card = Card::from_tup(quad_rank, i);
                cards.push(card);
            }
            for c in &self.cards {
                let c_local = c.clone();
                if (c_local.rank.clone() as usize) != quad_rank {
                    cards.push(c_local);
                    break;
                }
            }
            return Some((Value::FourOfKind(Rank::from_int(quad_rank)), cards))
        }
        None
    }



    fn straight_in_vec(v: &Vec<bool>) -> Result<usize, ()> {
        //returns OK(index of largest element of straight)
        let mut length = 0;
        let mut start = 100;
        for i in (1..15).rev() { //from 14 to 1 (both included)
            if v[i] {
                if length == 0 {
                    start = i;
                }
                length += 1;
            } else {
                length = 0
            }
            if length == 5 {
                return Ok(start)
            }
        }
        return Err(())
    }    
}


pub fn evaluate(h: &Vec<Card>, community: &Vec<Card>) -> (Value, Vec<Card>) {
    let hand = Hand::new_split(h, community);
    
    if let Some(result) = hand.top_tier() {
        return result
    } else if let Some(result) = hand.semi_tier() {
        return result
    }

    

    return (Value::HighCard(vec!()), vec![])
}