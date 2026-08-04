#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Hand(pub u32);

impl Hand
{
    pub fn empty() -> Self
    {
        Hand(0)
    }

    /* Used in exploration so we don't mutate the actual hand */
    pub fn with(self, value: usize) -> Self
    {
        if self.len() == 7 && !self.contains(value)
        {
            self
        }
        else
        {
            Hand(self.0 | (1 << value))
        }
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
        if self.len() == 7
        {
            total += 15;
        }
        total
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
impl Hand
{
    fn snapshot(self) -> String
    {
        format!(
            "[{}]\nlen={}\nScore={}",
            self.get_cards_in_hand(), self.len(), self.score(),
        )
    }
}

#[cfg(test)]
mod expect_tests
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

#[cfg(test)]
mod proptests
{
    use super::*;
    use proptest::prelude::*;

    proptest! 
    {
        /* Hand length never exceeds 7 */
        #[test]
        fn hand_len_never_exceeds_seven(cards in prop::collection::vec(0..13usize, 0..20)) 
        {
            let mut hand = Hand::empty();
            for card in cards 
            {
                hand = hand.with(card);
            }
            prop_assert!(hand.len() <= 7);
        }

        /* Cannot add cards already contained by the hand */
        #[test]
        fn duplicates_are_idempotent(card in 0..13usize)
        {
            let hand = Hand::empty().with(card);       
            let len_before = hand.len();

            let hand_after = hand.with(card);
            prop_assert_eq!(hand_after.len(), len_before);
            prop_assert_eq!(hand_after, hand);
        }

        /* Contains reflects added cards */
        #[test]
        fn contains_return_true_for_added_cards(cards in prop::collection::vec(0..13usize, 1..7))
        {
            let mut hand = Hand::empty();
            for &card in &cards
            {
                hand = hand.with(card);
            }

            for &card in &cards
            {
                prop_assert!(hand.contains(card));
            }
        }

        /* Score calculations */
        #[test]
        fn score_matches_sum_plus_bonus(cards in prop::collection::vec(0..13usize, 0..15))
        {
            let mut hand = Hand::empty();
            for card in cards
            {
                hand = hand.with(card);
            }

            let mut expect_sum = 0;
            for v in 0..13
            {
                if hand.contains(v)
                {
                    expect_sum += v as i32;
                }
            }

            if hand.len() == 7 
            {
                expect_sum += 15;
            }

            prop_assert_eq!(hand.score(), expect_sum);
        }
    } 
}
