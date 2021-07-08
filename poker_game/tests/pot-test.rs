use poker_game::pot::Pot;
use poker_game::player::{PlayerId, Player};

#[test]
fn one_player_claims_full_pot() {
    let player_ids: Vec<PlayerId> = (1..5).collect();
    let players: Vec<Player> = player_ids.into_iter().map(|x| Player::new(x, "name".to_string())).collect();
    let mut pot = Pot::new(&players);
    for player in &players {
        let id = player.id;
        pot.invest(100, id);
    }

    let winner_id = players[0].id;
    println!("start withdrawing");
    let win_vec = pot.withdraw(vec![winner_id]);
    assert!(win_vec.len() == 1);
    let amount = win_vec[0];

    // the won amount is 400
    assert!(amount.1 == 400);

    //the pot is empty after this player withdraws
    assert!(pot.amount == 0);
    
    //no player has an investment in the pot anymore
    for (_id, inv) in pot.player_investments {
        assert!(inv == 0);
    }
}

#[test]
fn two_players_claim_uneqal() {
    println!("starting test two winners:");
    let player_ids: Vec<PlayerId> = (1..5).collect();
    let players: Vec<Player> = player_ids.into_iter().map(|x| Player::new(x, "name".to_string())).collect();
    let mut pot = Pot::new(&players);
    for player in &players {
        let id = player.id;
        pot.invest(100, id);
    }
    pot.invest(100, 1);
    pot.invest(100, 2);
    

    println!("start withdrawing");
    let win_vec = pot.withdraw(vec![1, 3]);
    assert!(win_vec.len() == 2);
    let amount_1 = win_vec[0].1;
    let amount_2 = win_vec[1].1;
    // the won amount is 400
    assert!(amount_1 == 400);
    assert!(amount_2 == 200);

    //the pot is empty after this player withdraws
    assert!(pot.amount == 0);
    
    //no player has an investment in the pot anymore
    for (_id, inv) in pot.player_investments {
        assert!(inv == 0);
    }
}

#[test]
pub fn winner_is_all_in() {
    let player_ids: Vec<PlayerId> = (1..5).collect();
    let players: Vec<Player> = player_ids.into_iter().map(|x| Player::new(x, "name".to_string())).collect();
    let mut pot = Pot::new(&players);
    for player in &players {
        let id = player.id;
        pot.invest(100, id);
    }
    pot.invest(100, 1);
    pot.invest(100, 2);

    let win_vec = pot.withdraw(vec![3]);
    let amount = win_vec[0].1;
    assert!(amount==400);
    assert!(pot.amount == 200);
}