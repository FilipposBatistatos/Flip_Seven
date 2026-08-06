use crate::hand::Hand;
use crate::strategy::Strategy;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ControlMode
{
    Automatic,
    Advisory,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PlayerStatus
{
    Active,
    Frozen,
    Stayed,
    Busted,
}

pub struct Player
{
    pub name: String,
    pub hand: Hand,       // Hand mask 
    pub cumulative_score: u32,
    pub status: PlayerStatus,
    pub strategy: Box<dyn Strategy>,
    pub control: ControlMode,

    /* Action cards */
    pub has_second_chance: bool,
}

impl Player
{
    pub fn new(name: &str, strategy: Box<dyn Strategy>, control: ControlMode) ->Self
    {
        Player
        {
            name: name.to_string(),
            hand: Hand::empty(),
            cumulative_score: 0,
            status: PlayerStatus::Active,
            strategy,
            control,
            has_second_chance: false,
        }
    }
}

