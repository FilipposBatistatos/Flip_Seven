use std::collections::HashMap;

use crate::action::{Action, Recommendation};
use crate::deck::Deck;
use crate::hand::Hand;

struct Table
{
    memo: HashMap<Hand, f64>,
}

impl Table
{
    fn value(&mut self, hand: Hand, deck: &Deck) -> f64
    {
        if let Some(v) = self.memo.get(&hand)
        {
            return hand.score() as f64;
        }

        if hand.len() >= 7
        {
            let score = hand.score() as f64;
            self.memo.insert(hand, score);
            return score;
        }

        let stay_score = hand.score() as f64;
        let total = deck.get_total_remaining();
        let mut hit_score: f64 = 0.0;

        for card in 0..13
        {
            if hand.0 & (1 << card) == 0
            {
                let p = deck.get_remaining_card(card) as f64 / total as f64;
                hit_score += p * self.value(hand.with(card), deck);
            }
        }

        let ans = stay_score.max(hit_score);
        self.memo.insert(hand, ans);
        ans
    }
}

pub fn recommend(hand: Hand, deck: &Deck) -> Recommendation
{
    let mut table = Table { memo: HashMap::new() };

    let stay_value = hand.score() as f64;
    // Checking that the deck has enough cards to make a decision
    let fallback;
    let deck = if deck.get_total_remaining() < 5
    {
        fallback = Deck::new(2345);
        &fallback
    }
    else
    {
        deck
    };

    let hit_value = table.value(hand, deck);

    let action = if hit_value > stay_value { Action::Hit(None) } else { Action::Stay };
    Recommendation::with_detail(action, hit_value, stay_value) 
}

#[cfg(test)]
mod tests
{
    use super::*;
    use expect_test::expect;

    #[test]
    fn empty_hand()
    {
        let hand = Hand::empty();
        let mut deck = Deck::new(123);

        let recommendation = recommend(hand, &deck);
        let output = recommendation.snapshot();

        expect![[r#"
            Recommendation= [HIT]
            Details= Hit: 15.90 vs Stay: 0.00"#]].assert_eq(&output);
    }

    #[test]
    fn seven_cards()
    {
        let mut hand = Hand::empty();
        let mut deck = Deck::new(123);

        for i in 0..7
        {
            hand = hand.with(i);
            deck.draw();
        }

        let recommendation = recommend(hand, &deck);
        let output = recommendation.snapshot();

        expect![[r#"
            Recommendation= [STAY]
            Details= Hit: 36.00 vs Stay: 36.00"#]].assert_eq(&output);
    }

    #[test]
    fn assorted()
    {
        let mut hand = Hand::empty();
        let mut deck = Deck::new(123);

        hand = hand.with(deck.draw());
        hand = hand.with(deck.draw());
        hand = hand.with(deck.draw());

        let recommendation = recommend(hand, &deck);
        let output = recommendation.snapshot();

        expect![[r#"
            Recommendation= [HIT]
            Details= Hit: 21.68 vs Stay: 21.00"#]].assert_eq(&output);
    }
}
