use core::panic;
use std::usize;
use std::cmp::Ordering;
use crate::cards::{Card, Rank};

pub enum Value {
    RoyalFlush,
    StraightFlush(usize), //contains highest card's rank 
    FourOfKind(usize), //contains rank of those 4 cards -> can never draw! 
    FullHouse(usize, usize), // contains rank of upper 3 cards and lower 2, but only upper 3 matter for ordering
    Flush(Vec<usize>), // contains all 5 cards, here it actually matters
    Straight(usize), // contains highest card of straight
    ThreeOfKind(usize), // contains rank of triplet, other cards will never matter
    TwoPair(usize, usize, usize), //contains rank of higher pair, lower pair, and other card
    Pair(usize, Vec<usize>), // Rank of pair, should also contain ranks of the 3 other cards
    HighCard(Vec<usize>), // contains ranks of all 5 cards
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        self.eq_helper(other) == 0
    }
}


impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let res = self.eq_helper(other);
        if res == 1 {
            Some(Ordering::Greater)
        } else if res == -1 {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Equal)
        }
    }
}

impl Value {
    fn to_int(&self) -> usize {
        match self {
            Self::HighCard(_) => 1,
            Self::Pair(_,_) => 2,
            Self::TwoPair(_,_,_) => 3,
            Self::ThreeOfKind(_) => 4,
            Self::Straight(_) => 5,
            Self::Flush(_) => 6,
            Self::FullHouse(_,_) => 7,
            Self::FourOfKind(_) => 8,
            Self::StraightFlush(_) => 9,
            Self::RoyalFlush => 10,
        }
    }
    fn eq_helper(&self, other: &Self) -> i8 {
        let val1 = self.to_int();
        let val2 = other.to_int();
        if val1 > val2 {
            1
        } else if val2 > val1 {
            -1
        } else {
            match self {
                Self::RoyalFlush => 0, //always equal
                Self::StraightFlush(start) => {
                    if let Self::StraightFlush(otherstart) = other {
                        comp_helper(*start, *otherstart)
                    } else {
                        panic!();
                    }
                },
                Self::FourOfKind(value) => {
                    if let Self::FourOfKind(otherval) = other {
                        comp_helper(*value, *otherval)
                    } else {
                        panic!();
                    }
                },
                Self::FullHouse(upper, lower) => {
                    if let Self::FullHouse(other_upper, other_lower) = other {
                        // higher triplet wins always, since there can be no equality there
                        comp_helper(*upper, *other_upper)
                    } else {
                        panic!()
                    }
                },
                Self::Flush(list) => {
                    assert!(list.len() == 5);
                    if let Self::Flush(otherlist) = other {
                        assert!(otherlist.len() == 5);
                        for i in 0..5 {
                            let res = comp_helper(list[i], otherlist[i]);
                            if res != 0 {
                                return res
                            } else {
                                continue;
                            }
                        }
                        return 0
                    } else {
                        panic!()
                    }
                },
                Self::Straight(start) => {
                    if let Self::Straight(otherstart) = other {
                        comp_helper(*start, *otherstart)
                    } else {
                        panic!()
                    }
                },
                Self::ThreeOfKind(val) => {
                    if let Self::ThreeOfKind(otherval) = other {
                        comp_helper(*val, *otherval)
                    } else {
                        panic!()
                    }
                },
                Self::TwoPair(val1, val2, val3) => {
                    if let Self::TwoPair(oval1, oval2, oval3) = other {
                        let res1 = comp_helper(*val1, *oval1);
                        if res1 != 0 {
                            res1
                        } else {
                            let res2 = comp_helper(*val2, *oval2);
                            if res2 != 0 {
                                res2
                            } else {
                                comp_helper(*val3, *oval3)
                            }
                        }
                    } else {
                        panic!()
                    }
                },
                Self::Pair(val, list) => {
                    assert!(list.len() == 3);
                    if let Self::Pair(val1, otherlist) = other {
                        let res1 = comp_helper(*val, *val1);
                        if res1 != 0 {
                            return res1
                        } else {
                            assert!(otherlist.len() == 5);
                            for i in 0..3 {
                                let res = comp_helper(list[i], otherlist[i]);
                                if res != 0 {
                                    return res
                                } else {
                                    continue;
                                }
                            }
                            return 0
                        }
                    } else {
                        panic!()
                    }                    
                },
                Self::HighCard(list) => {
                    assert!(list.len() == 5);
                    if let Self::HighCard(otherlist) = other {
                        assert!(otherlist.len() == 5);
                        for i in 0..5 {
                            let res = comp_helper(list[i], otherlist[i]);
                            if res != 0 {
                                return res
                            } else {
                                continue;
                            }
                        }
                        return 0
                    } else {
                        panic!()
                    }
                }
            }
        }
    }
}

// How do we evaluate a set of 7 cards? First, what is the result of 
// an evaluation? the best possible interpretation. How to find this easily?

#[derive(Debug)]
pub struct Hand {
    cards: Vec<Card>, // your cards sorted by rank, decreasing
    ranks: Vec<usize>,
    matrix: Vec<Vec<bool>>,
    quads: Vec<usize>,
    trips: Vec<usize>,
    tuples: Vec<usize>,
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

        let mut quads = vec![];
        let mut trips = vec![];
        let mut tuples = vec![];

        for rank in (2..15).rev() {
            let quant = ranks[rank];
            if quant == 4 {
                quads.push(rank);
            }
            if quant >= 3 {
                trips.push(rank);
            }
            if quant >= 2 {
                tuples.push(rank);
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
    pub fn evaluate(&self) -> Value {
        if let Some(result) = self.top_tier() {
            return result
        } else if let Some(result) = self.quad() {
            return result
        } else if let Some(result) = self.flush() {
            return result
        } else if let Some(result) = self.full_house() {
            return result
        } else if let Some(result) = self.three_of_kind() {
            return result
        } 
        

        return Value::HighCard(vec!())
    } 

    pub fn top_tier(&self) -> Option<Value> {
        for suit in 1..5 {
            match Self::straight_in_vec(&self.matrix[suit]) {
                Ok(start) => {
                    if start == 14 {
                        return Some(Value::RoyalFlush)
                    } else {
                        let result = Value::StraightFlush(start);
                        return Some(result)
                    }
                },
                Err(()) => {},
            }
        }
        None
    }

    pub fn quad(&self) -> Option<Value> {
        //find fourOfKind
        if !self.quads.is_empty() {
            let quad_rank = self.quads[0];
            return Some(Value::FourOfKind(quad_rank))
        } 
        None
    }

    pub fn full_house(&self) -> Option<Value> {
        if self.trips.len() >= 1 && self.tuples.len() >= 2 {
            //there definitely is a full house:
            let high_trip = self.trips[0];
            let tup = if self.tuples[0] == high_trip {
                self.tuples[1]
            } else {
                self.tuples[0]
            };

            Some(Value::FullHouse(high_trip, tup))
        } else {
            None
        }
    }

    pub fn flush(&self) -> Option<Value> {
        for i in 1..5 {
            //now it is bad that the aces are counted twice in the matrix
            let correction = if self.matrix[i][1] { 1 } else { 0 };
            let occ: usize = (&self.matrix[i]).into_iter().count() - correction;
            if occ >= 5 {
                let mut cards = vec![];
                let mut count = 0;

                //get the 5 highest cards of this color
                for j in (2..15).rev() {
                    assert!(j != 1);
                    let curr = self.matrix[i][j];
                    if curr {
                        cards.push(j);
                        count += 1;
                    }
                    if count >= 5 {
                        return Some(Value::Flush(cards))
                    }
                }
            }
        }
        None
    }

    pub fn three_of_kind(&self) -> Option<Value> {
        if !self.trips.is_empty() {
            let trip_rank = self.trips[0];
            Some(Value::ThreeOfKind(trip_rank))
        } else {
            None
        }
    }

    pub fn two_pair(&self) -> Option<Value> {
        if self.tuples.len() >= 2 {
            let r1 = self.tuples[0];
            let r2 = self.tuples[1];
            let mut r3 = 0;
            for card in &self.cards {
                let rank = card.rank_int();
                if rank == r1 || rank == r2 {
                    continue;
                } else {
                    r3 = rank;
                }
            }
            Some(Value::TwoPair(r1, r2, r3))
        } else {
            None
        }
    }

    pub fn pair(&self) -> Option<Value> {
        if !self.tuples.is_empty() {
            let r1 = self.tuples[0];
            let mut rest = vec!();
            let mut count = 0;
            
            for card in &self.cards {
                if count == 3 {
                    break;
                }
                let rank = card.rank_int();
                if rank == r1 {
                    continue;
                } else {
                    rest.push(rank);
                    count += 1;
                }
            }
            Some(Value::Pair(r1, rest))
        } else {
            None
        }
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


pub fn evaluate(h: &Vec<Card>, community: &Vec<Card>) -> Value {
    let hand = Hand::new_split(h, community);
    hand.evaluate()
}


pub fn comp_helper(x: usize, y: usize) -> i8{
    if x == y {
        0
    } else if x > y {
        1
    } else {
        -1
    }
}