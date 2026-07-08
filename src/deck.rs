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

    pub fn deal(&mut self) -> usize
    {
        let mut card_idx = self.rng.random_range(0..=self.total_remaining);

        let mut i = 0;
        while card_idx > 0 && i < self.cards.len()
        {
            card_idx -= self.cards[i];
            i += 1;
        }

        self.total_remaining -= 1;
        i - 1
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
