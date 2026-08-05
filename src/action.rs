#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Action
{
    Hit(Option<usize>),
    Stay,
}

#[derive(Debug)]
pub struct Recommendation 
{
    pub action: Action,
    pub hit: Option<f64>,
    pub stay: Option<f64>,
}

impl Recommendation
{
    pub fn simple(action: Action) -> Self
    {
        Recommendation {action, hit: None, stay: None }
    }
    
    pub fn with_detail(action: Action, hit_value: f64, stay_value: f64) -> Self
    {
        Recommendation
        {
            action,
            // detail: Some(format!("Hit: {:.2} vs Stay: {:.2}", hit_value, stay_value)),
            hit: Some(hit_value),
            stay: Some(stay_value),
        }            
    }
}

// TODO: Proptesting for recommendations

