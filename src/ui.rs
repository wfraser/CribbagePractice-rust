// UserInterface trait specification.
//
// Copyright (c) 2016 by William R. Fraser
//

use crate::card::Card;
use crate::combo::Combo;
use crate::hand::Hand;

pub struct Guess {
    pub cards: Vec<Card>,
    pub score: i8,
}

pub trait UserInterface {
    fn display_hand(&mut self, hand: &Hand<'_>);
    fn display_missed_combos(&mut self, combos: &[Combo<'_>]);
    fn display_win_message(&mut self, score: i32);
    fn display_lose_message(&mut self, score: i32);
    fn display_bad_guess_wrong_score(&mut self, actual_combo: &Combo<'_>);
    fn display_bad_guess_invalid_combo(&mut self);
    fn display_correct_guess(&mut self, combo: &Combo<'_>);
    fn add_score_player(&mut self, score: i32);
    fn add_score_cpu(&mut self, score: i32);
    fn get_guess(&mut self, hand: &Hand<'_>) -> Option<Guess>;
}
