use crate::player::{PlayerId, Player};
use rand::Rng;
use std::collections::HashMap;

pub struct Pot {
    pub amount: u32,
    pub player_investments: HashMap<PlayerId, u32>,
    pub min_unit: u32, //smallest possible chip size
}

impl Pot {
    pub fn new(players: &Vec<Player>) -> Pot {
        let mut map = HashMap::new();
        for player in players {
            map.insert(player.id, 0);
        }
        Pot {
            amount: 0,
            player_investments: map,
            min_unit: 1,
        }
    }   
    
    pub fn invest(&mut self, amount: u32, player_id: PlayerId) {
        self.amount = self.amount + amount;
        let entry = self.player_investments.entry(player_id).or_insert(0);
        *entry += amount;
    }

    pub fn claim_down_to(&mut self, amount: u32, lower_bound: u32) -> u32 {
        let mut sum = 0;
        for (_id, investment) in &mut self.player_investments {
            if *investment > lower_bound {
                let excess = *investment - lower_bound;
                if excess > amount {
                    *investment -= amount;
                    sum += amount;
                    self.amount -= amount;
                } else {
                    *investment -= excess;
                    sum += excess;
                    self.amount -= excess;

                }
            }
        }
        sum
    }

    // in case there are draws this is a bit more complicated
    pub fn withdraw(&mut self, player_ids: Vec<PlayerId>) -> Vec<(PlayerId, u32)> {
        println!("length of player_ids: {}", player_ids.len());
        if player_ids.len() == 1 {
            println!("only one winner");
            // either he was all in and some other players have bet more, or he gets everything
            let id = player_ids[0];
            let investment = self.player_investments.get(&id).expect("every player_id should be in here").clone();
            let mut win = 0;
            //if another player invested more than *investment* this winner can only claim part of it 
            for (_id, inv) in &mut self.player_investments { 
                if *inv <= investment {
                    win += *inv;
                    self.amount -= *inv;
                    *inv = 0;
                } else {
                    win += investment;
                    *inv -= investment;
                    self.amount -= investment;
                }
            }
            vec![(id, win)]
        } else {
            let mut id_investment = vec![];
            for id in player_ids {
                let investment: u32 = self.player_investments.get(&id).expect("invalid player id, this is a bug").clone();
                id_investment.push((id, investment));
            }
            id_investment.sort_by(|a, b| b.1.cmp(&a.1));
            println!("{:?}", id_investment);

            // this is a huge pain....
            let mut rewards: Vec<(PlayerId, u32)> = id_investment.clone().into_iter().map(|(id, _inv)| (id, 0)).collect();
            
            loop {
                println!("rewards: {:?}", rewards);
                println!("investments: {:?}", self.player_investments);
                println!("winner investments: {:?}", id_investment);
                let mut count_equals: u32 = 0;
                let max_amount = id_investment[0].1;
                let mut second_highest = 0;

                for (_, investment) in &id_investment {
                    if *investment == max_amount {
                        count_equals += 1;
                    } else {
                        second_highest = *investment;
                        break;
                    }
                }
                //if all have invested the same amount, 
                let mut shareable = self.claim_down_to(max_amount - second_highest, second_highest);
                
                for i in 0..(count_equals as usize) {
                    id_investment[i].1 = second_highest;
                }

                let odd_amount = shareable % (self.min_unit * count_equals);
                shareable -= odd_amount;
                let cut_each = shareable / count_equals;
                for i in 0..(count_equals as usize) {
                    rewards[i].1 += cut_each;
                }
                if odd_amount > 0 {
                    let mut rng = rand::thread_rng();
                    let winner = rng.gen_range(0..count_equals) as usize;
                    rewards[winner].1 += odd_amount;
                }
                if second_highest == 0 {
                    break;
                }

                println!("second highest: {:?}", second_highest);
            }
            rewards
        }
    }


}