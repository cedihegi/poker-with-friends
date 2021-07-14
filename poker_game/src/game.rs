use crate::player::PlayerId;
use crate::table::Table;
// this should give us an interface to go through the different stages of a game of poker. 
// For now, games always start with a set of 2 - 10 players

// this layer does not involve any timing constraints, it gives every player infinite time to decide
// and only proceeds if players are active. 

pub trait Game {
    fn start_with_pos(position: u32) -> Result<GameEvent, GameEvent>; //the table keeps track of some state, e.g. the dealer
    fn player_performs_action(pid: PlayerId, action: Action) -> Result<GameEvent, GameEvent>;
}

pub enum Action {
    Call,
    Raise(usize),
    Fold,
}

pub enum GameEvent {
    AskForNextMove(PlayerId), //suggestion of what player should move next
    ShowNewCards,
    SharePot(Vec<(PlayerId, u32)>),
}