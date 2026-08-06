use crate::action::{ Action, Recommendation };
use crate::deck::Deck;
use crate::hand::Hand;
use crate::dp_solver;

pub trait Strategy
{
    fn decide(&self, hand: Hand, deck: &Deck) -> Recommendation;
    fn name(&self) -> &str;

    fn choose_target(
        &self, 
        _acting: usize,
        _players: &[Player],
        _card: usize,
    ) -> usize 
    {
        // Default implementation for strategy: Always return highest hand

        _players
            .iter()
            .enumerate()
            .filter(|(i, p)| *i != acting && p.status == PlayerStatus::Active)
            .max_by_key(|(_, p)| p.hand.score())
            .map(|(i, _)| i)
            .unwrap_or(acting)
    }
}

pub struct ExpectedValue;
impl Strategy for ExpectedValue
{
    fn decide(&self, hand: Hand, deck: &Deck) -> Recommendation
    {
        dp_solver::recommend(hand, deck)
    }

    fn name(&self) -> &str
    {
        "Expected value"
    }
}

pub struct Threshold(pub i32);
impl Strategy for Threshold
{
    fn decide(&self, hand: Hand, deck: &Deck) -> Recommendation
    {
        let action = if hand.score() < self.0 { Action::Hit(None) } else { Action::Stay };
        Recommendation::simple(action)
    }

    fn name(&self) -> &str
    {
        "threshold"
    }
}
