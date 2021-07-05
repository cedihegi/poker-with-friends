use crate::player::{PlayerId};
use std::collections::HashMap;

pub struct Pot {
    pub amount: u32,
    pub player_investments: HashMap<PlayerId, u32>,
}

impl Pot {
    pub fn invest(&self, amount: u32) {
        
    }
}