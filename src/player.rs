#[derive(Debug)]
pub struct Player
{
    name: String,
    pub hand: u16,       // Hand mask 
    cumulative_score: i32,
    pub can_draw: bool,
}

impl Player
{
    pub fn new() ->Self
    {
        Self
        {
            name: "Player".to_string(),
            hand: 0,
            cumulative_score: 0,
            can_draw: true,
        }
    }

    pub fn hand_size(&self) ->u32
    {
        self.hand.count_ones()
    }

    pub fn hand_score(&self) -> i32
    {
        let mut score = 0;
        for card in 0..=12
        {
            if (self.hand & (1 << card)) != 0
            {
                score += card;
            }
        }

        if self.hand_size() >= 7
        {
            score += 15;
        }

        score
    } 

    pub fn give_card(&mut self, card: usize)
    {
        self.hand ^= 1 << card; 
        if (self.hand & (1 << card)) == 0
        {
            self.can_draw = false;
        }
    }

    pub fn on_round_end(&mut self)
    {
        self.cumulative_score += self.hand_score();
        self.hand = 0;
        self.can_draw = true;
    }
}

#[cfg(test)]
impl Player
{
    fn snapshot(&self) -> String
    {
        format!(
            "Name={}\nScore={}\nHand={} len={}\nHand score={}\nCan draw={}",
            self.name, self.cumulative_score, self.get_cards_in_hand(), self.hand_size(), self.hand_score(), self.can_draw, 
        )
    }

    fn get_cards_in_hand(&self) -> String
    {
        let mut output = String::new();

        for i in 0..13
        {
            if (self.hand & (1 << i)) != 0
            {
                output.push_str(&format!(
                        "{} ",
                        i.to_string()
                    ))
            }
        }

        output
    }
}

#[cfg(test)]
mod tests
{
    use super::*;
    use expect_test::expect;

    #[test]
    fn new_player()
    {
        let player = Player::new();

        let output = player.snapshot();

        expect![[r#"
            Name=Player
            Score=0
            Hand= len=0
            Hand score=0
            Can draw=true"#]]
            .assert_eq(&output);
    }

    #[test]
    fn hand()
    {
        let mut player = Player::new();

        player.give_card(2 as usize);
        player.give_card(5 as usize);
        player.give_card(7 as usize);

        let output = player.snapshot();
        
        expect![[r#"
            Name=Player
            Score=0
            Hand=2 5 7  len=3
            Hand score=14
            Can draw=true"#]]
            .assert_eq(&output);
    }

    #[test]
    fn detect_duplicate()
    {
        let mut player = Player::new();

        player.give_card(2 as usize);
        player.give_card(5 as usize);
        player.give_card(7 as usize);
        player.give_card(5 as usize);

        let output = player.snapshot();

        expect![[r#"
            Name=Player
            Score=0
            Hand=2 7  len=2
            Hand score=9
            Can draw=false"#]]
            .assert_eq(&output);
    }

    #[test]
    fn round_end()
    {
        let mut player = Player::new();

        player.give_card(2 as usize);
        player.give_card(5 as usize);
        player.give_card(7 as usize);

        player.on_round_end();

        let output = player.snapshot();

        expect![[r#"
            Name=Player
            Score=14
            Hand= len=0
            Hand score=0
            Can draw=true"#]]
            .assert_eq(&output);
    }
}
