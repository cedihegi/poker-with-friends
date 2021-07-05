use crate::player::Player;

// this should give us an interface to go through the different stages of a game of poker. 
// For now, games always start with a set of 2 - 10 players

// this layer does not involve any timing constraints, it gives every player infinite time to decide
// and only proceeds if players are active. 


pub struct Game {
    pub players: Vec<Player>,
    pub size: usize,
    
}

pub struct GameState {
    pub turn: usize, //tells us which player's turn it is
    pub dealer: usize, //dealer - 1 % size ist big blind, -2 small blind
    pub pot: Pot,
    
}