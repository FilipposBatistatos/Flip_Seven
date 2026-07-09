use std::collections::HashMap;

mod deck;
mod hand;

use deck::Deck;
use hand::Hand;

fn main() {

    // setting up the "deck"
    let mut deck = Deck::new(15);

    // Setting up player
    let mut hand = Hand::empty();

    // Deal a random card
    loop
    {
        let dealt_card = deck.draw(); 
        if hand.contains(dealt_card)
        {
            println!("Dealt a duplicate: {dealt_card}");
            break;
        }
        hand = hand.with(dealt_card);
        println!("Dealt card: {dealt_card}, score: {}", hand.score());
    }
}
