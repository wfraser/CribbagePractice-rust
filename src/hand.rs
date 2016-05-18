// Hand :: represents a cribbage hand, and functions for finding scoring card combinations.
//
// Copyright (c) 2016 by William R. Fraser
//

use std::fmt::{self, Display, Formatter};

use super::card::Card;
use super::combo::Combo;
use super::util;

#[cfg(test)]
use super::card::Suit;

pub struct Hand<'a> {
    pub cards: &'a [Card],
}

impl<'a> Hand<'a> {
    pub fn new(cards: &'a [Card]) -> Hand<'a> {
        Hand {
            cards: cards,
        }
    }

    fn find_fifteens(&self, combos: &mut Vec<Combo<'a>>) {
        for set in util::power_set(self.cards) {
            if set.iter().fold(0, |acc, card| acc + card.value()) == 15 {
                combos.push(Combo {
                    cards: set.clone(),
                    score: 2,
                    text: format!("fifteen"),
                });
            }
        }
    }

    fn find_n_of_kind(&self, combos: &mut Vec<Combo<'a>>) {
        let by_number = util::group_by(self.cards, |card| card.number);
        for group in by_number.values() {
            if group.len() > 1 {
                combos.push(Combo {
                    cards: group.clone(),
                    score: 2 * (util::factorial(group.len() as i8) / 4), // n take 2 = n! / (2!)^2
                    text: format!("{} of a kind", group.len()),
                });
            }
        }
    }

    fn find_flush(&self, combos: &mut Vec<Combo<'a>>) {
        let by_suit = util::group_by(self.cards, |card| card.suit);
        for group in by_suit.values() {
            if group.len() > 4 {
                combos.push(Combo {
                    cards: group.clone(),
                    score: group.len() as i8,
                    text: format!("{}-flush", group.len()),
                });
            }
        }
    }

    fn make_run_combos(cards: &[Vec<&'a Card>], combos: &mut Vec<Combo<'a>>) {
        let mut indices: Vec<usize> = vec![];
        for _ in 0 .. cards.len() { indices.push(0); }
        loop {
            let mut combo_cards: Vec<&'a Card> = vec![];
            for (i, idx) in indices.iter().enumerate() {
                combo_cards.push(cards[i][*idx]);
            }

            combos.push(Combo {
                cards: combo_cards,
                score: cards.len() as i8,
                text: format!("run of {}", cards.len()),
            });

            for i in 0 .. cards.len() + 1 {
                if i == cards.len() {
                    return;
                } else if indices[i] == cards[i].len() - 1 {
                    indices.as_mut_slice()[i] = 0;
                } else {
                    indices.as_mut_slice()[i] += 1;
                    break;
                }
            }
        }
    }

    fn find_runs(&self, combos: &mut Vec<Combo<'a>>) {
        let by_number = util::group_by(self.cards, |card| card.number);
        // There's probably some more clever way to do this but whatever.
        let mut candidates: Vec<Vec<&'a Card>> = vec![];
        let mut prev = -1i8;
        for (number, group) in by_number.iter() {
            if prev != -1 && *number != prev + 1 {
                // numbers not adjacent; reset.
                if candidates.len() >= 3 {
                    Self::make_run_combos(&candidates, combos);
                }
                candidates.clear();
            }
            prev = *number;
            candidates.push(group.clone());
        }

        if candidates.len() >= 3 {
            Self::make_run_combos(&candidates, combos);
        }
    }

    pub fn find_all_combos(&self) -> Vec<Combo> {
        let mut combos: Vec<Combo> = vec![];
        self.find_fifteens(&mut combos);
        self.find_n_of_kind(&mut combos);
        self.find_flush(&mut combos);
        self.find_runs(&mut combos);
        combos
    }
}

impl<'a> Display for Hand<'a> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        for (idx, card) in self.cards.iter().enumerate() {
            try!((card as &Display).fmt(fmt));
            if idx < self.cards.len() - 1 {
                fmt.write_str(" ").unwrap();
            }
        }
        Ok(())
    }
}

#[cfg(test)]
pub fn cards_str(cards: &[&Card]) -> String {
    let mut out = String::new();
    for (idx, card) in cards.iter().enumerate() {
        out.push_str(&format!("{}", card));
        if idx < cards.len() - 1 {
            out.push(' ');
        }
    }
    out
}

#[test]
fn test_find_runs() {
    let card_1 = Card { number: 1, suit: Suit::Diamonds };
    let card_2a = Card { number: 2, suit: Suit::Diamonds };
    let card_2b = Card { number: 2, suit: Suit::Spades };
    let card_3a = Card { number: 3, suit: Suit::Diamonds };
    let card_3b = Card { number: 3, suit: Suit::Spades };
    let cards = vec![card_1, card_2a, card_2b, card_3a, card_3b];
    let hand = Hand::new(&cards);
    let mut runs = vec![];
    hand.find_runs(&mut runs);
    assert_eq!(runs.len(), 4);
    assert_eq!(cards_str(&runs[0].cards), "AD 2D 3D");
    assert_eq!(cards_str(&runs[1].cards), "AD 2S 3D");
    assert_eq!(cards_str(&runs[2].cards), "AD 2D 3S");
    assert_eq!(cards_str(&runs[3].cards), "AD 2S 3S");


}
