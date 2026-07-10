#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Hand(pub u16);

impl Hand
{
    pub fn empty() -> Self
    {
        Hand(0)
    }

    /* Used in exploration so we don't mutate the actual hand */
    pub fn with(self, value: usize) -> Self
    {
        Hand(self.0 | (1 << value))
    }

    pub fn contains(self, value: usize) -> bool
    {
        self.0 & (1 << value) != 0
    }

    pub fn len(self) -> u32
    {
        self.0.count_ones()
    }

    pub fn score(self) -> i32
    {
        let mut total = 0;
        for v in 0..13u32
        {
            if self.0 & (1 << v) != 0
            {
                total += v as i32;
            }
        }
        if self.len() >= 7
        {
            total += 15;
        }
        total
    }
}

#[cfg(test)]
impl Hand
{
    fn snapshot(self) -> String
    {
        format!(
            "[{}]\nlen={}\nScore={}",
            self.get_cards_in_hand(), self.len(), self.score(),
        )
    }

    pub fn get_cards_in_hand(self) -> String
    {
        let mut output = String::new();

        for i in 0..13
        {
            if (self.0 & (1 << i)) != 0
            {
                output.push_str(&format!("{} ", i.to_string()))
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
    fn new_hand()
    {
        let hand = Hand::empty();

        let output = hand.snapshot();
        
        expect![[r#"
            []
            len=0
            Score=0"#]]
            .assert_eq(&output);
    }

    #[test]
    fn add_cards()
    {
        let mut hand = Hand::empty();

        hand = hand.with(2);
        hand = hand.with(3);
        hand = hand.with(5);

        let output = hand.snapshot();

        expect![[r#"
            [2 3 5 ]
            len=3
            Score=10"#]]
            .assert_eq(&output);
    }

    #[test]
    fn seven_card_bonus()
    {
        let mut hand = Hand::empty();

        for i in 0..7
        {
            hand = hand.with(i);
        }

        let output = hand.snapshot();

        expect![[r#"
            [0 1 2 3 4 5 6 ]
            len=7
            Score=36"#]]
            .assert_eq(&output);
    }

    #[test]
    fn with_makes_copy()
    {
        let mut hand = Hand::empty();
        hand = hand.with(4);

        let new_hand = hand.with(5);

        let output = hand.snapshot() + &new_hand.snapshot();

        expect![[r#"
            [4 ]
            len=1
            Score=4[4 5 ]
            len=2
            Score=9"#]]
            .assert_eq(&output);
    }

    // TODO: Better contains testing, perhaps exhaustive
    #[test]
    fn contains()
    {
        let mut hand = Hand::empty();

        hand.with(4);
        hand.with(5);
        hand.with(6);

        let mut output = format!("Contains 7={}", hand.contains(7).to_string());

        hand.with(7);
        output = format!("{}\nContains 7={}", &output, hand.contains(7).to_string());

        expect![[r#"
            Contains 7=false
            Contains 7=false"#]]
            .assert_eq(&output);
    }
}
