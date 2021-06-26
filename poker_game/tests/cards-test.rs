use poker_game::cards;

#[test]
fn rank_orders(){
    let two = cards::Rank::Two;
    let three = cards::Rank::Three;
    let ten = cards::Rank::Ten;
    let jack = cards::Rank::Jack;
    let ace = cards::Rank::Ace;


    assert!(two < three);
    assert!(three < ace);
    assert!(jack > ten);
    assert!(ace > jack);
}

#[test]
fn card_orders() {
    // use ranks as previous:
    let two = cards::Rank::Two;
    let ten = cards::Rank::Ten;
    let jack = cards::Rank::Jack;
    // two suits: 
    let clubs = cards::Suit::Clubs;
    let diamond = cards::Suit::Diamonds;

    let card1 = cards::Card { rank: two, suit: clubs };
    let card2 = cards::Card { rank: cards::Rank::Two, suit: diamond};
    let card3 = cards::Card { rank: jack, suit: cards::Suit::Clubs };
    let card4 = cards::Card { rank: ten, suit: cards::Suit::Diamonds };

    assert_eq!(card1, card2);
    assert!(card3 > card4);
    assert_ne!(card1, card4);
}