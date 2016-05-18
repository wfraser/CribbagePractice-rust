// Deck :: represents a standard 52-card deck of cards.
//
// Copyright (c) 2016 by William R. Fraser
//

use rand::{self, Rng};

use super::card::{Card, Suit};
use super::hand::Hand;

pub struct Deck {
    cards: Vec<Card>,
}

impl<'a> Deck {
    pub fn new() -> Deck {
        let mut cards = vec![];

        for num in 0..13 {
            for suit in 0..4 {
                cards.push(Card {
                    number: num + 1,
                    suit: match suit {
                        0 => Suit::Spades,
                        1 => Suit::Clubs,
                        2 => Suit::Hearts,
                        3 => Suit::Diamonds,
                        _ => unreachable!(),
                    }
                });
            }
        }

        Deck {
            cards: cards,
        }
    }

    pub fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        for i in 0 .. self.cards.len() - 1 {
            let j = rng.gen_range(i+1, self.cards.len());
            self.cards.swap(i, j);
        }
    }

    pub fn deal_hand(&'a self, size: usize) -> Hand<'a> {
        Hand::new(&self.cards[0..size])
    }
}
