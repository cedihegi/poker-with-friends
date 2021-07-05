use std::cmp::Ordering;
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
                Self::FullHouse(upper, _lower) => {
                    if let Self::FullHouse(other_upper, _other_lower) = other {
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


pub fn comp_helper(x: usize, y: usize) -> i8{
    if x == y {
        0
    } else if x > y {
        1
    } else {
        -1
    }
}