use crate::action::{ Action, Recommendation };
use crate::deck::Deck;
use crate::hand::Hand;
use crate::dp_solver;

pub trait Strategy
{
    fn decide(&self, hand: Hand, deck: &Deck) -> Recommendation;
    fn name(&self) -> &str;
}

pub struct Expected_value;
impl Expected_value
{
    fn decide(&self, hand: Hand, deck: &Deck) -> Recommendation
    {
        solver::recommend(hand, deck);
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
        let action = if hand.score() < self.0 { Action::Hit } else { Action::Stay };
        Recommendation::simple(action)
    }

    fn name(&self) -> &str
    {
        "threshold"
    }
}
