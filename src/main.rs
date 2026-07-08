use std::collections::HashMap;

mod deck;
mod player;

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

fn value(mask: u16, deck: &deck::Deck, cache: &mut HashMap<u16, f64>) -> f64
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
    let total = deck.get_total_remaining();

    for card in 0..13
    {
        if mask & (1 << card) == 0
        {
            let p = deck.get_remaining_card(card) as f64 / total as f64;
            hit_score += p * value(mask | (1 << card), deck, cache);
        }
    }

    let ans = stay_score.max(hit_score);
    cache.insert(mask, ans);
    ans
}

fn main() {

    // setting up the "deck"
    let mut deck = deck::Deck::new(15);

    // Setting up player
    let mut player = player::Player::new();

    // Deal a random card
    loop
    {
        let mut cache = HashMap::new();
        let hit_score = value(player.hand, &deck, &mut cache);
        let dealt_card = deck.deal(); 
        player.deal_card(dealt_card);
        if !player.can_draw
        {
            println!("Dealt a duplicate: {dealt_card}");
            break;
        }
        println!("Dealt card: {dealt_card}, predicted score: {hit_score}, score: {}", player.hand_score());
        
    }
}
