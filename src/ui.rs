// UserInterface trait specification.
//
// Copyright (c) 2016 by William R. Fraser
//

use super::card::Card;
use super::combo::Combo;
use super::hand::Hand;

pub struct Guess {
    pub cards: Vec<Card>,
    pub score: i8,
}

pub trait UserInterface {
    fn display_hand(&mut self, hand: &Hand);
    fn display_missed_combos(&mut self, combos: &[Combo]);
    fn display_win_message(&mut self, score: i32);
    fn display_lose_message(&mut self, score: i32);
    fn display_bad_guess_wrong_score(&mut self, actual_combo: &Combo);
    fn display_bad_guess_invalid_combo(&mut self);
    fn display_correct_guess(&mut self, combo: &Combo);
    fn add_score_player(&mut self, score: i32);
    fn add_score_cpu(&mut self, score: i32);
    fn get_guess(&mut self, hand: &Hand) -> Option<Guess>;
}
