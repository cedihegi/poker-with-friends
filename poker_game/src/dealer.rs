use crate::cards::Card;
use rand::prelude::*;


pub struct Dealer {
    deck: Vec<Card>,
    order: Vec<usize>,
    position: usize,
}

impl Dealer {
    pub fn new() -> Dealer {
        let deck = Card::to_vec();
        let mut rng = rand::thread_rng();
        let mut order: Vec<usize> = (0..52).collect();
        order.shuffle(&mut rng);
        Dealer {
            deck,
            order,
            position: 0,
        }
    }
    pub fn get_card(&mut self) -> Result<Card, ()> {
        if self.position >= 52 {
            Err(())
        } else {
            let pos = self.position;
            let deckpos = self.order[pos];
            self.position += 1;
            Ok(self.deck[deckpos].clone())
        }
    }

    pub fn get_cards(&mut self, number: usize) -> Result<Vec<Card>, ()> {
        let mut result = vec![];
        if self.position + number > 52 {
            Err(())
        } else {
            for _i in 0..number {
                result.push(self.get_card()?);
            }
            Ok(result)
        }
    }
}