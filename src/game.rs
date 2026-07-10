use crate::action::Action;
use crate::player::{ ControlMode, Player, PlayerStatus };
use crate::deck::Deck;
use crate::hand::Hand;

pub struct Game
{
    pub players: Vec<Player>,
    deck: Deck,
}

impl Game
{   
    pub fn new(players: Vec<Player>, seed: u64) -> Self
    {
        Game { players, deck: Deck::new(seed) }
    }

    pub fn play_round(&mut self)
    {
        for p in &mut self.players
        {
            p.hand = Hand::empty();
            p.status = PlayerStatus::Active;
        }

        'round: loop
        {
            let mut any_active = false;

            for i in 0..self.players.len()
            {
                if self.players[i].status != PlayerStatus::Active
                {
                    continue;
                }
                any_active = true;

                let rec = self.players[i].strategy.decide(self.players[i].hand, &self.deck);

                let action = match self.players[i].control
                {
                    ControlMode::Automatic => rec.action,
                    ControlMode::Advisory => 
                    {
                        println!(
                            "{}: recommends {:?} ({:?})",
                            self.players[i].name, rec.action, rec.detail
                        );
                        rec.action
                    }
                };

                match action
                {
                    Action::Stay => self.players[i].status = PlayerStatus::Stayed,
                    Action::Hit => 
                    {
                        let card = self.deck.draw();
                        if self.players[i].hand.contains(card)
                        {
                            // Player drew a duplicate card
                            self.players[i].status = PlayerStatus::Busted;
                        }
                        else
                        {
                            self.players[i].hand = self.players[i].hand.with(card);
                            if self.players[i].hand.len() >= 7
                            {
                                break 'round; // End the round for ever player
                            }
                        }
                    }
                }
            }
            if !any_active
            {
                break;
            }
        }

        for p in &mut self.players 
        {
            let round_score = if p.status == PlayerStatus::Busted { 0 } else { p.hand.score() };
            p.cumulative_score += round_score as u32;
        }
    }
}

#[cfg(test)]
impl Game
{
    fn state(self) -> String
    {
        let mut output: String = String::new();

        for i in 0..self.players.len()
        {
            output += &format!("{}\n", self.players[i].hand.get_cards_in_hand());
        }

        output
    }
}

#[cfg(test)]
mod test
{
    use super::*;
    use expect_test::expect;
    use crate::strategy::{ Strategy, Threshold, ExpectedValue };

    #[test]
    fn test_round()
    {
        let mut players = Vec::new();
        for i in 1..4
        {
            players.push(Player::new(
                    &format!("Player {}", i),
                    Box::new(Threshold(25)), 
                    ControlMode::Advisory)
                );
        }

        let mut game = Game::new(players, 123);
        game.play_round();

        let output = game.state();

        expect![[r#"
            9 10 11 
            3 11 12 
            10 
        "#]].assert_eq(&output);
    }
}
