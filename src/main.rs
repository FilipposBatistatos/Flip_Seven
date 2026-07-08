use rand::{rngs::SmallRng, Rng, SeedableRng};

#[derive(Debug)]
struct Deck
{
    cards: [i8; 13],
    remaining_count: i8,
    rng: SmallRng,
}

impl Deck
{
    fn new(seed: u64) -> Self
    {
        Self
        {
            cards: [1, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            remaining_count: 79,
            rng: SmallRng::seed_from_u64(seed),
        }
    }

    fn deal(&mut self) -> usize
    {
        let mut card_idx = self.rng.random_range(0..=self.remaining_count);

        let mut i = 0;
        while card_idx > 0 && i <= 13
        {
            card_idx -= self.cards[i];
            i += 1
        }
        self.remaining_count -= 1;
        i - 1
    }
}

fn main() {
    let mut hand_mask: i16 = 0b0000_0000_0000_0000;

    // setting up the "deck"
    let mut deck = Deck::new(2);

    // Deal a random card
    loop
    {
        let dealt_card = deck.deal(); 
        hand_mask ^= 1 << dealt_card;
        if (hand_mask & (1 << dealt_card)) == 0
        {
            println!("Dealt a duplicate: {dealt_card}");
            break;
        }
        println!("Dealt card: {dealt_card}, map: {hand_mask}");
    }
}
