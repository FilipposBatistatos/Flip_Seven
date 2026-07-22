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
    pub detail: Option<String>,
}

impl Recommendation
{
    pub fn simple(action: Action) -> Self
    {
        Recommendation {action, detail: None }
    }
    
    pub fn with_detail(action: Action, hit_value: f64, stay_value: f64) -> Self
    {
        Recommendation
        {
            action,
            detail: Some(format!("Hit: {:.2} vs Stay: {:.2}", hit_value, stay_value)),
        }            
    }
}

#[cfg(test)]
impl Recommendation
{
    pub fn snapshot(self) -> String
    {
        let action_text = match self.action
        {
            Action::Hit(_) => "[HIT]",
            Action::Stay => "[STAY]",
        };

        match &self.detail
        {
            Some(details_text) => format!("Recommendation= {}\nDetails= {}", action_text, details_text),
            None => format!("Recommendation= {}", action_text),
        }
    }
}

