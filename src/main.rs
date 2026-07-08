use std::collections::HashMap;
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
            i += 1;
        }
        self.remaining_count -= 1;
        i - 1
    }

    fn get_remaining_count(&self) -> i8
    {
        self.remaining_count
    }

    fn get_remaining_v(&self, v: usize) -> i8
    {
        if v < self.cards.len()
        {
            self.cards[v]
        }
        else
        {
            0 as i8
        }
    }
}

fn eval_score(mask: u16) -> i32
{
    let mut score = 0;
    let mut card_count = 0;

    for i in 0..13
    {
        if (mask & (1 << i)) != 0
        {
            score += i;
            card_count += 1;
        }
    }

    if card_count >= 7
    {
        score += 15;
    }
    
    score
}

fn value(mask: u16, deck: &Deck, cache: &mut HashMap<u16, f64>) -> f64
{
    if let Some(v) = cache.get(&mask)
    {
        return *v;
    }

    if mask.count_ones() >= 7
    {
        let score = eval_score(mask) as f64;
        cache.insert(mask, score);
        return score;
    }

    let stay_score = eval_score(mask) as f64;

    let mut hit_score = 0.0;
    let total = deck.get_remaining_count();

    for card in 0..13
    {
        if mask & (1 << card) == 0
        {
            let p = deck.get_remaining_v(card) as f64 / total as f64;
            hit_score += p * value(mask | (1 << card), deck, cache);
        }
    }

    let ans = stay_score.max(hit_score);
    cache.insert(mask, ans);
    ans
}

fn main() {
    let mut hand_mask: u16 = 0b0000_0000_0000_0000;

    // setting up the "deck"
    let mut deck = Deck::new(15);

    // Deal a random card
    loop
    {
        let mut cache = HashMap::new();
        let hit_score = value(hand_mask, &deck, &mut cache);
        let dealt_card = deck.deal(); 
        hand_mask ^= 1 << dealt_card;
        if (hand_mask & (1 << dealt_card)) == 0
        {
            println!("Dealt a duplicate: {dealt_card}");
            break;
        }
        println!("Dealt card: {dealt_card}, predicted score: {hit_score}, score: {}", eval_score(hand_mask));
        
    }
}
