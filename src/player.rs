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

    pub fn deal_card(&mut self, card: usize)
    {
        self.hand ^= 1 << card; 
        if (self.hand & (1 << card)) == 0
        {
            // Player dealt a duplicate card
            println!("{} busted", self.name);
            self.can_draw = false;
        }
    }
}
