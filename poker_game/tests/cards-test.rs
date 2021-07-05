use poker_game::{cards, hand_values::Value, logic};

#[test]
fn recognize_royal_flush() {
    let mut cards = vec![];
    cards.push(cards::Card::from_tup(2,2));
    cards.push(cards::Card::from_tup(3,3));
    for i in 10..15 {
        cards.push(cards::Card::from_tup(1, i));
    }
    for card in &cards {
        println!("{}", card);
    }

    match logic::evaluate_vec(&cards) {
        Value::RoyalFlush => {},
        _ => panic!()
    }

}

#[test]
fn recognize_straight_flush() {
}