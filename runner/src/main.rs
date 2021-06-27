use poker_game::{cards, dealer, logic};
use rand::thread_rng;

fn main() -> Result<(), ()>{
    let mut deal = dealer::Dealer::new();
    let hand = deal.get_cards(2)?;
    let comm = deal.get_cards(5)?;

    logic::evaluate(&hand, &comm);


    Ok(())
}
