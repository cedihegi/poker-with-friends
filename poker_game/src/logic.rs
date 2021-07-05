use std::usize;
use crate::cards::Card;
use crate::hand_values::Value;

// How do we evaluate a set of 7 cards? First, what is the result of 
// an evaluation? the best possible interpretation. How to find this easily?

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>, // your cards sorted by rank, decreasing
    ranks: Vec<usize>,
    matrix: Vec<Vec<bool>>,
    quads: Vec<usize>,
    trips: Vec<usize>,
    tuples: Vec<usize>,
}

impl Hand {
    //initialize
    pub fn new(cards: &Vec<Card>) -> Hand {
        assert!(cards.len() == 7);
        let mut matrix = vec![vec![false; 15]; 5];
        let mut ranks = vec![0; 15];
        for card in cards {
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
        Hand {
            cards: cards_sorted,
            ranks,
            matrix,
            quads,
            trips,
            tuples
        }
    }

    pub fn new_split(hand: &Vec<Card>, community: &Vec<Card>) -> Hand {
        assert!(hand.len() == 2);
        assert!(community.len() == 5);

        let mut combined = hand.clone();
        combined.append(&mut community.clone());
        Hand::new(&combined)
    }
}

impl Hand {
    //analyze
    pub fn evaluate(&self) -> Value {
        if let Some(result) = self.top_tier() {
            return result
        } else if let Some(result) = self.quad() {
            return result
        } else if let Some(result) = self.full_house() {
            return result
        } else if let Some(result) = self.flush() {
            return result
        } else if let Some(result) = self.straight() {
            return result
        } else if let Some(result) = self.three_of_kind() {
            return result
        } else if let Some(result) = self.two_pair() {
            return result
        } else if let Some(result) = self.pair() {
            return result
        } else {
            self.high_card()
        } 
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

    pub fn straight(&self) -> Option<Value> {
        let bool_vec = self.ranks.clone().into_iter().map(|x| x > 0).collect();
        let res = Self::straight_in_vec(&bool_vec);
        match res {
            Ok(start) => Some(Value::Straight(start)),
            Err(_) => None
        }
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

    pub fn high_card(&self) -> Value {
        //since cards are sorted we take first 5 values:
        let mut ranks = vec![];
        for i in 0..5 {
            ranks.push(self.cards[i].rank_int());
        }
        Value::HighCard(ranks)
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

//evaluates a
pub fn evaluate(h: &Vec<Card>, community: &Vec<Card>) -> Value {
    assert!(h.len() == 2);
    assert!(community.len() == 5);
    let hand = Hand::new_split(h, community);
    hand.evaluate()
}

pub fn evaluate_vec(cards: &Vec<Card>) -> Value {
    let hand = Hand::new(cards);
    hand.evaluate()
}


