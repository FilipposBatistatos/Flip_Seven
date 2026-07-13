use crate::action::Action;
use crate::player::{ ControlMode, Player, PlayerStatus };
use crate::deck::Deck;
use crate::hand::Hand;

pub struct Game
{
    pub players: Vec<Player>,
    deck: Deck,
    next_index: usize,
}

pub enum StepOutcome
{
    NeedsInput { player_index: usize, recommendation: crate::action::Recommendation },
    RoundOver,
}

impl Game
{
    pub fn new(players: Vec<Player>, seed: u64) -> Self
    {
        Game { players, deck: Deck::new(seed), next_index: 0 }
    }

    pub fn start_round(&mut self)
    {
        for p in &mut self.players
        {
            p.hand = Hand::empty();
            p.status = PlayerStatus::Active;
        }
        self.next_index = 0;
    }

    pub fn step(&mut self) -> StepOutcome
    {
        loop
        {
            if !self.players.iter().any(|p| p.status == PlayerStatus::Active)
            {
                self.finish_round();
                return StepOutcome::RoundOver;
            }

            let i = self.next_index % self.players.len();
            self.next_index += 1;

            if self.players[i].status != PlayerStatus::Active
            {
                continue;
            }

            let rec = self.players[i].strategy.decide(self.players[i].hand, &self.deck);

            match self.players[i].control
            {
                ControlMode::Automatic =>
                {
                    if self.apply(i, rec.action)
                    {
                        return StepOutcome::RoundOver;
                    }
                }
                ControlMode::Advisory =>
                {
                    return StepOutcome::NeedsInput { player_index: i, recommendation: rec };
                }
            }
        }
    }

    pub fn apply(&mut self, player_index: usize, action: Action) -> bool
    {
        match action
        {
            Action::Stay => self.players[player_index].status = PlayerStatus::Stayed,
            Action::Hit =>
            {
                let card = self.deck.draw();
                if self.players[player_index].hand.contains(card)
                {
                    self.players[player_index].status = PlayerStatus::Busted;
                }
                else
                {
                    self.players[player_index].hand = self.players[player_index].hand.with(card);
                    if self.players[player_index].hand.len() >= 7
                    {
                        self.finish_round();
                        return true;
                    }
                }
            }
        }

        if !self.players.iter().any(|p| p.status == PlayerStatus::Active)
        {
            self.finish_round();
            return true;
        }
        false
    }

    fn finish_round(&mut self)
    {
        for p in &mut self.players
        {
            let round_score = if p.status == PlayerStatus::Busted { 0 } else { p.hand.score() };
            p.cumulative_score += round_score as u32;
        }
    }

    pub fn play_round(&mut self)
    {
        self.start_round();
        loop
        {
            match self.step()
            {
                StepOutcome::RoundOver => return,
                StepOutcome::NeedsInput { .. } =>
                {
                    panic!("play_round() requires every player to be Automatic — got an Advisory pause");
                }
            }
        }
    }

    pub fn play_game(&mut self, target: u32)
    {
        loop
        {
            self.play_round();
            if self.players.iter().any(|p| p.cumulative_score >= target)
            {
                return;
            }
        }
    }

    pub fn display(&self) -> String
    {
        let mut output: String = String::new();

        for i in 0..self.players.len()
        {
            output += &format!(
                "+--------------------------------+\n|{}\n+--------------------------------+\n| Hand: {} \n| Status: {} \n| Total: {} \n+--------------------------------+\n",
                self.players[i].name,
                self.players[i].hand.get_cards_in_hand(),
                match self.players[i].status
                {
                    PlayerStatus::Active => "Active",
                    PlayerStatus::Busted => "Busted",
                    PlayerStatus::Stayed => "Stayed",
                },
                self.players[i].cumulative_score
                );
        }

        output
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
