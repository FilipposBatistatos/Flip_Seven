use crate::action::Action;
use crate::player::{ ControlMode, Player, PlayerStatus };
use crate::deck::{ Deck, cards };
use crate::hand::Hand;

pub struct Game
{
    pub players: Vec<Player>,
    deck: Deck,
    next_index: usize,
    pending_action: Option<(usize, usize)>, // (acting_player, card)
}

pub enum StepOutcome
{
    NeedsInput 
    { 
        player_index: usize, 
        recommendation: crate::action::Recommendation 
    },
    NeedsTarget
    {
        action_player: usize,       // Index of the player that drew the card
        card: usize,                // Number corresponding to the card type
        valid_targets: Vec<usize>,  // Vector of valid player indexes
        recommendation: usize,      // Holds recommendation for human players
    },
    RoundOver,
}

impl Game
{
    pub fn new(players: Vec<Player>, seed: u64) -> Self
    {
        Game { players, deck: Deck::new(seed), next_index: 0, pending_action: None }
    }

    pub fn start_round(&mut self, starting_index: usize)
    {
        for p in &mut self.players
        {
            p.hand = Hand::empty();
            p.status = PlayerStatus::Active;
        }
        /* In the rules of flip seven the starting player increments every round */
        self.next_index = starting_index;
    }

    pub fn step(&mut self) -> StepOutcome
    {
        // Checking for pending actions
        if let Some((acting_player, card)) = self.pending_action.take() // Take removes 
        {
            match self.players[acting_player].control
            {
                ControlMode::Automatic =>
                {
                    let target = {
                        let strategy = &*self.players[acting_player].strategy;
                        strategy.choose_target(acting_player, &self.players, card)
                    };
                    if self.apply_action_card(card, target, &[])
                    {
                        return StepOutcome::RoundOver;
                    }
                }
                ControlMode::Advisory =>
                {
                    let recommendation = self.players[acting_player].strategy.choose_target(acting_player, &self.players, card);
                    return StepOutcome::NeedsTarget
                    {
                        action_player: acting_player,
                        card,
                        valid_targets: (0..self.players.len())
                            .filter(|&j| self.players[j].status == PlayerStatus::Active)
                            .collect(),
                        recommendation,
                    };
                }
            }
        }
         
        loop
        {
            // Check whether the deck needs reshuffling
            if self.deck.get_total_remaining() == 0
            {
                let mut hands: Vec<Hand> = vec![];
                for player in &self.players 
                {
                    hands.push(player.hand);
                }

                self.deck.reshuffle(&hands);
            }
            
            // Check whether there are active players still
            if !self.players.iter().any(|p| p.status == PlayerStatus::Active)
            {
                self.finish_round();
                return StepOutcome::RoundOver;
            }

            let i = self.next_index % self.players.len();

            self.next_index += 1;
            /* 
             * This catches:
             * - Frozen
             * - Stayed
             * - Busted 
             */
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
            Action::Hit(provided_card) =>
            {
                let card = match provided_card
                { 
                    Some(card) =>
                    {
                        if self.deck.discard(card)
                        {
                            card
                        }
                        else
                        {
                            self.deck.draw()
                        }
                    }
                    None =>
                    {
                        self.deck.draw()
                    }
                };

                /* Handle cards you can hold onto */
                if card < 19
                {
                    /* The modifier cards also go through this check but you cannot bust on them */
                    if self.players[player_index].hand.contains(card)
                    {
                        if self.players[player_index].has_second_chance
                        {
                            self.players[player_index].has_second_chance = false;
                            // Card is discarded and thus not added to the hand
                            // player survives
                        }
                        else
                        {
                            self.players[player_index].status = PlayerStatus::Busted;
                        }
                    }
                    else
                    {
                        self.players[player_index].hand = self.players[player_index].hand.with(card);
                        if self.players[player_index].hand.len() >= 7
                        // Return true if someone reached the seven card target
                        {
                            self.finish_round();
                            return true;
                        }
                    }
                }
                else /* Handle action cards 19, 20, 21 */
                {
                    match self.players[player_index].control
                    {
                        ControlMode::Automatic =>
                        {
                            let target = {
                                // Borrow checker conflict we need to take out strategy from players
                                // before we can give players back to it
                                let strategy = &*self.players[player_index].strategy;
                                strategy.choose_target(player_index, &self.players, card) 
                            };
                            if self.apply_action_card(card, target, &[])
                            {
                                return true;
                            }
                        }
                        ControlMode::Advisory =>
                        {
                            self.pending_action = Some((player_index, card));
                            return false;
                        }
                    }
                }
            }
        }

        if !self.players.iter().any(|p| p.status == PlayerStatus::Active)
        {
            // Return true if there are no active players
            self.finish_round();
            return true;
        }
        false
    }

    pub fn apply_action_card(&mut self, card: usize, target: usize, physical_cards: &[Option<usize>]) -> bool
    {
        match card
        {
            cards::FREEZE => self.players[target].status = PlayerStatus::Frozen,
            cards::SECOND_CHANCE => self.players[target].has_second_chance = true,
            cards::FLIP_THREE => {
                for i in 0..3
                {
                    let drawn = physical_cards.get(i).copied().flatten();
                    if self.apply(target, Action::Hit(drawn))
                    {
                        return true; //round over, player busted
                    }
                    if self.players[target].status == PlayerStatus::Busted
                    {
                        break;
                    }
                }
            },
            _ => return false,
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

    pub fn play_round(&mut self, starting_index: usize)
    {
        self.start_round(starting_index);
        loop
        {
            match self.step()
            {
                StepOutcome::RoundOver => return,
                StepOutcome::NeedsInput { .. } =>
                {
                    panic!("play_round() requires every player to be Automatic — got an Advisory pause, input");
                }
                StepOutcome::NeedsTarget { .. } =>
                {
                    panic!("play_round() requires every player to be Automatic — got and Advisory pause, target");
                }
            }
        }
    }

    pub fn play_game(&mut self, target: u32)
    {
        let mut round : usize = 0;
        loop
        {
            self.play_round(round);
            round += 1;
            if self.players.iter().any(|p| p.cumulative_score >= target)
            {
                return;
            }
        }
    }

   pub fn display(&self) -> String
    {
        let headers = ["Player", "Hand", "Hand Score", "Status", "Cumulative"];

        let rows: Vec<[String; 5]> = self.players.iter().map(|p| {
            let status_str = match p.status
            {
                PlayerStatus::Active => "Active",
                PlayerStatus::Busted => "Busted",
                PlayerStatus::Stayed => "Stayed",
                PlayerStatus::Frozen => "Frozen",
            };
            [
                p.name.clone(),
                p.hand.get_cards_in_hand() + if p.has_second_chance { "<3" } else { "" },
                p.hand.score().to_string(),
                status_str.to_string(),
                p.cumulative_score.to_string(),
            ]
        }).collect();

        let mut widths = [0usize; 5];
        for (i, h) in headers.iter().enumerate()
        {
            widths[i] = h.len();
        }
        for row in &rows
        {
            for i in 0..5
            {
                widths[i] = widths[i].max(row[i].len());
            }
        }

        let sep = |widths: &[usize; 5]| -> String {
            let mut s = String::from("+");
            for w in widths
            {
                s += &"-".repeat(w + 2);
                s += "+";
            }
            s + "\n"
        };
        let row_str = |cells: &[String; 5], widths: &[usize; 5]| -> String {
            let mut s = String::from("|");
            for i in 0..5
            {
                s += &format!(" {:<width$} |", cells[i], width = widths[i]);
            }
            s + "\n"
        };

        let mut out = sep(&widths);
        out += &row_str(&headers.map(|s| s.to_string()), &widths);
        out += &sep(&widths);
        for row in &rows
        {
            out += &row_str(row, &widths);
        }
        out += &sep(&widths);
        out
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
                    ControlMode::Automatic)
                );
        }

        let mut game = Game::new(players, 123);
        game.play_round(0);

        let output = game.display();

        expect![[r#"
            +----------+----------+------------+--------+------------+
            | Player   | Hand     | Hand Score | Status | Cumulative |
            +----------+----------+------------+--------+------------+
            | Player 1 | 9 10 11  | 30         | Stayed | 30         |
            | Player 2 | 3 11 12  | 26         | Stayed | 26         |
            | Player 3 | 10       | 10         | Busted | 0          |
            +----------+----------+------------+--------+------------+
        "#]].assert_eq(&output);
    }
}
