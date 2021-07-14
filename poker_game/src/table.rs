use crate::player::{Player, PlayerId};
use crate::pot::Pot;
use crate::game::{Action, Game, GameEvent};

pub struct Table<T: Game> {
    seats: Vec<Seat>,
    dealer: usize,
    pot: Option<Pot>,
    nr_sitting: usize,
    game: T,
}

impl<T: Game> Table<T> {
    pub fn new(game: T, length: usize) -> Self {
        let mut seats = Vec::new();
        for i in 0..length {
            seats.push(Seat::new(i));
        }
        Table {
            seats,
            dealer: 0,
            pot: None,
            nr_sitting: 0,
            game,
        }
    }

    pub fn sitdown(&mut self, amount: u32, pid: PlayerId) -> Result<usize, ()> {
        for seat in &mut self.seats {
            if seat.player.is_none() {
                match seat.place(pid, amount) {
                    Err(()) => {},
                    x => return x
                }
            }
        }
        Err(())
    }

    pub fn play_move(&mut self, action: Action, pid: PlayerId) {
        
        
    }
}


pub struct Seat {
    player: Option<PlayerId>,
    placed_amount: u32,
    position: usize,

}

impl Seat {
    pub fn new(position: usize) -> Self {
        Seat {
            player: None,
            placed_amount: 0,
            position,
        }
    }

    pub fn place(&mut self, pid: PlayerId, amount: u32) -> Result<usize, ()> {
        //place at any position
        if self.player.is_none() {
            self.player = Some(pid);
            self.placed_amount += amount;
            Ok(self.position)
        } else {
            Err(())
        }
    }

    pub fn get_up(&mut self) -> Result<u32, ()> {
        if self.player.is_some() {
            self.player = None;
            let withdraw = self.placed_amount;
            self.placed_amount = 0;
            Ok(withdraw)
        } else {
            Err(())
        }
    }
}