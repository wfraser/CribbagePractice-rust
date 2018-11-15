// Deck :: represents a standard 52-card deck of cards.
//
// Copyright (c) 2016 by William R. Fraser
//

use rand;
use rand::seq::SliceRandom;

use super::card::{Card, Suit};
use super::hand::Hand;

pub struct Deck {
    cards: Vec<Card>,
}

impl<'a> Deck {
    pub fn new() -> Deck {
        let mut cards = vec![];

        for number in 1 ..= 13 {
            for suit in &[Suit::Spades, Suit::Clubs, Suit::Hearts, Suit::Diamonds] {
                cards.push( Card {
                    number,
                    suit: *suit,
                });
            }
        }
        assert_eq!(52, cards.len());

        Deck {
            cards,
        }
    }

    pub fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        self.cards.shuffle(&mut rng);
    }

    pub fn deal_hand(&'a self, size: usize) -> Hand<'a> {
        Hand::new(&self.cards[0..size])
    }

}
