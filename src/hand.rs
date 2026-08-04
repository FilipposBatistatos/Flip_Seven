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
        (self.0 & 0x1FFF).count_ones()
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
        
        if (self.0 & (1 << 18)) != 0
        {
            total *= 2;
        }

        for v in 1..6
        {
            if self.0 & (1 << (12 + v)) != 0 
            {
                total += (v * 2) as i32;
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
                output.push_str(&format!("{} ", i.to_string()));
            }
        }

        for i in 1..6
        {
            if (self.0 & (1 << (i + 12))) != 0
            {
                output.push_str(&format!("+{} ", (2 * i).to_string()));
            }
        }
        
        if (self.0 & (1 << 18)) != 0
        {
            output.push_str(&"x2");
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

    #[test]
    fn modifiers_add_to_score()
    {
        let hand = Hand::empty().with(13).with(14).with(15).with(16).with(17).with(18);

        let output = hand.snapshot();

        expect![[r#"
            [+2 +4 +6 +8 +10 x2]
            len=0
            Score=30"#]]
            .assert_eq(&output);
    }

    #[test]
    fn multiplier_effects_base_score_only()
    {
        let hand = Hand::empty().with(2).with(3).with(13).with(14).with(18);

        let output = hand.snapshot();

        expect![[r#"
            [2 3 +2 +4 x2]
            len=2
            Score=16"#]]
            .assert_eq(&output);
    }
}

#[cfg(test)]
mod proptests
{
    use super::*;
    use proptest::prelude::*;

    const PLUS_TWO: usize = 13;
    const PLUS_FOUR: usize = 14;
    const PLUS_SIX: usize = 15;
    const PLUS_EIGHT: usize = 16;
    const PLUS_TEN: usize = 17;
    const TIMES_TWO: usize = 18;

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

        /* ============== Special cards ============== */

        /* Multipler doubles base score */
        #[test]
        fn multiplier_doubles_base_score(base_cards in prop::collection::vec(0..13usize, 1..7))
        {
            let mut base_hand = Hand::empty();
            for card in base_cards
            {
                base_hand = base_hand.with(card);
            }

            let modified_hand = base_hand.with(TIMES_TWO); 
            prop_assert!(modified_hand.score() >= base_hand.score() * 2);
        }
        
        #[test]
        fn multiplier_doubles_only_base_score(
            c0 in 0..13usize, c1 in 0..13usize, c2 in 0..13usize,
            c3 in 0..13usize, c4 in 0..13usize, c5 in 0..13usize,
            mod_card in prop::sample::select(vec![PLUS_TWO, PLUS_FOUR, PLUS_SIX, PLUS_EIGHT, PLUS_TEN])
        )
        {
            let base = Hand::empty().with(c0).with(c1).with(c2).with(c3).with(c4).with(c5);

            let with_mod = base.with(mod_card);
            let doubled = with_mod.with(TIMES_TWO);
            let increase = doubled.score() - with_mod.score();

            prop_assert_eq!(increase, base.score()); 
        }
    } 
}
