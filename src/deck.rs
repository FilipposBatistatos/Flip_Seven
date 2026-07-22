use rand::{rngs::SmallRng, Rng, SeedableRng};

#[derive(Debug)]
pub struct Deck
{
    cards: [i8; 13],
    total_remaining: i8,
    rng: SmallRng,
}

impl Deck 
{
    pub fn new(seed: u64) -> Self 
    {
        Self {
            cards: [1, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            total_remaining: 79,
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
        // Draws from deck, updates deck state 
        if self.total_remaining <= 0
        {
            self.reshuffle();
        }

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

    pub fn reshuffle(&mut self)
    {
        /* Consider the fact that the players might have cards on their hands when this occurs */
        self.cards = [1, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
        self.total_remaining = 79;
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
mod tests 
{
    use super::*;
    use expect_test::expect;

    #[test]
    fn new_deck()
    {
        let deck = Deck::new(123);

        let output = deck.snapshot();

        expect![[r#"
            remaining=79
             [1, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]"#]]
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
            remaining=76
             [0, 0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 11]"#]]
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
            remaining=57
             [1, 1, 2, 1, 4, 4, 6, 6, 6, 7, 5, 6, 8]
            10 11 10 9 3 10 11 12 12 10 11 8 12 7 10 8 5 3 12 11 9 11 "#]]
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

        deck.reshuffle();

        let output = deck.snapshot();

        expect![[r#"
            remaining=79
             [1, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]"#]]
            .assert_eq(&output);
    }

    #[test]
    fn reshuffle_on_empty()
    {
        let mut deck = Deck::new(123);

        for _ in 1..81
        {
            deck.draw();
        }

        let output = deck.snapshot();

        expect![[r#"
            remaining=78
             [1, 1, 2, 3, 4, 5, 6, 7, 7, 9, 10, 11, 12]"#]]
            .assert_eq(&output);
    }
}


