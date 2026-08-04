use rand::{rngs::SmallRng, Rng, SeedableRng};
use crate::hand::Hand;

#[derive(Debug)]
pub struct Deck
{
    cards: [i8; 22],        // 0 - 12: number cards, 13-18: modifiers, 19-21: actions
    total_remaining: i8,
    rng: SmallRng,
}

pub mod card
{
    // Modifier cards
    pub const PLUS_TWO: usize = 13;
    pub const PLUS_FOUR: usize = 14;
    pub const PLUS_SIX: usize = 15;
    pub const PLUS_EIGHT: usize = 16;
    pub const PLUS_TEN: usize = 17;
    pub const TIMES_TWO: usize = 18;

    // Action cards
    pub const FLIP_THREE: usize = 19;
    pub const FREEZE: usize = 20;
    pub const SECOND_CHANCE: usize = 21;
}

impl Deck 
{
    pub fn new(seed: u64) -> Self 
    {
        Self {
            cards: [1, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 1, 1, 1, 1, 1, 1, 3, 3, 3],
            total_remaining: 94,
            rng: SmallRng::seed_from_u64(seed),
        }
    }

    pub fn discard(&mut self, card: usize) -> bool
    {
        // Return positive of able to discard, 
        // negative otherwise
        if card < 13 && self.cards[card] > 0
        {
            self.cards[card] -= 1;
            self.total_remaining -= 1;
            return true;
        }
        false
    }

    pub fn draw(&mut self) -> usize
    {
        let mut card_idx = self.rng.random_range(0..self.total_remaining);

        for (card, &count) in self.cards.iter().enumerate()
        {
            if card_idx < count
            {
                self.cards[card] -= 1;
                self.total_remaining -= 1;
                return card;
            }
            card_idx -= count;
        }

        panic!("Deck state desynced from total_remaining!");
    }

    pub fn reshuffle(&mut self, hands: &Vec<Hand>)
    {
        /* Consider the fact that the players might have cards on their hands when this occurs */
        self.cards = [1, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 1, 1, 1, 1, 1, 1, 3, 3, 3];
        self.total_remaining = 94;

        for &hand in hands
        {
            for card in 0..19 // 19 - 21 are action cards which a instantly discarded upon been drawn
            {
                if hand.contains(card)
                {
                    self.cards[card] -= 1;
                    self.total_remaining -= 1;
                }
            }
        }
    }

    pub fn get_total_remaining(&self) -> i8
    {
        self.total_remaining
    }

    pub fn get_remaining_card(&self, card:usize) -> i8
    {
        let res: i8;
        if card < self.cards.len()
        {
            res = self.cards[card];
        }            
        else
        {
            res = 0;
        }
        res
    }
}

#[cfg(test)]
impl Deck
{
    fn snapshot(&self) -> String 
    {
        format!(
            "remaining={}\n {:?}",
            self.total_remaining,
            self.cards,
        )
    }
}

#[cfg(test)]
mod expect_tests 
{
    use super::*;
    use expect_test::expect;

    #[test]
    fn new_deck()
    {
        let deck = Deck::new(123);

        let output = deck.snapshot();

        expect![[r#"
            remaining=94
             [1, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 1, 1, 1, 1, 1, 1, 3, 3, 3]"#]]
            .assert_eq(&output);
    }

    #[test]
    fn discard()
    {
        let mut deck = Deck::new(123);

        deck.discard(0);
        deck.discard(1);
        deck.discard(12);

        let output = deck.snapshot();

        expect![[r#"
            remaining=91
             [0, 0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 11, 1, 1, 1, 1, 1, 1, 3, 3, 3]"#]]
            .assert_eq(&output);
    }

    #[test]
    fn draw()
    {
        let mut deck = Deck::new(123);

        let mut drawn_cards = String::new();
        for _i in 1..23
        {
            drawn_cards.push_str(&format!(
                    "{} ",
                    deck.draw().to_string()
                    ))
        }

        let output = deck.snapshot() + "\n" + &drawn_cards;

        expect![[r#"
            remaining=72
             [1, 1, 2, 1, 4, 4, 6, 7, 7, 7, 7, 7, 7, 1, 1, 1, 1, 1, 0, 2, 2, 2]
            11 12 11 10 3 10 12 19 20 11 12 9 21 8 11 9 5 3 18 12 10 12 "#]]
            .assert_eq(&output);
    }

    #[test]
    fn reshuffle()
    {
        let mut deck = Deck::new(123);

        for _ in 1..23
        {
            deck.draw();
        }

        let mut hand = Hand::empty();
        hand = hand.with(0);
        hand = hand.with(12);
        hand = hand.with(18);
        let hands: Vec<Hand> = vec![hand];

        deck.reshuffle(&hands);

        let output = deck.snapshot();

        expect![[r#"
            remaining=91
             [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 11, 1, 1, 1, 1, 1, 0, 3, 3, 3]"#]]
            .assert_eq(&output);
    }
}


