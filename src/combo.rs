// Combo :: represents a combination of cards that score points in the game.
//
// Copyright (c) 2016 by William R. Fraser
//

use crate::card::Card;

#[derive(Debug)]
pub struct Combo<'a> {
    pub cards: Vec<&'a Card>,
    pub score: i8,
    pub text: String,
}

impl<'a> PartialEq<[&'a Card]> for Combo<'a> {
    fn eq(&self, other: &[&Card]) -> bool {
        if self.cards.len() != other.len() {
            return false;
        }

        for card in &self.cards {
            if !other.iter().any(|x| x == card) {
                return false;
            }
        }
        true
    }
}

impl<'a> PartialEq<[Card]> for Combo<'a> {
    fn eq(&self, other: &[Card]) -> bool {
        if self.cards.len() != other.len() {
            return false;
        }

        for card in &self.cards {
            if !other.iter().any(|x| &x == card) {
                return false;
            }
        }
        true
    }
}
