// Card :: a standard playing card
//
// Copyright (c) 2016 by William R. Fraser
//

use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Suit {
    Spades,
    Clubs,
    Hearts,
    Diamonds,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Card {
    pub suit: Suit,
    pub number: i8,
}

impl Card {
    /// Value of the card for scoring purposes.
    pub fn value(&self) -> i8 {
        if self.number > 10 {
            10
        } else {
            self.number
        }
    }
}

impl Display for Card {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        let mut out = String::new();

        match self.number {
            1  => out.push_str("A"),
            11 => out.push_str("J"),
            12 => out.push_str("Q"),
            13 => out.push_str("K"),
            _  => out.push_str(&format!("{}", self.number)),
        }

        match self.suit {
            Suit::Spades    => out.push_str("S"),
            Suit::Clubs     => out.push_str("C"),
            Suit::Hearts    => out.push_str("H"),
            Suit::Diamonds  => out.push_str("D"),
        }

        fmt.write_str(&out)
    }
}

impl Eq for Card {
}

#[derive(Debug)]
pub struct CardParseError {
    message: String,
}

impl CardParseError {
    fn new(s: String) -> CardParseError {
        CardParseError {
            message: s
        }
    }
}

impl Display for CardParseError {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt.write_str(self.description())
    }
}

impl Error for CardParseError {
    fn description(&self) -> &str {
        &self.message
    }
}

impl FromStr for Card {
    type Err = CardParseError;

    /// Parses strings of the form "<number><suit>" where "<number>" is 1-13 or A, J, Q, K; and "<suit>" is S, C, D, or H.
    fn from_str(s: &str) -> Result<Card, CardParseError> {
        let (suit_byte_index, suit_char) = try!(s.char_indices().last().ok_or_else(|| CardParseError::new(format!("invalid card {:?}", s))));
        let num_str = &s[0..suit_byte_index];

        let suit: Suit;
        match suit_char {
            's' | 'S' => suit = Suit::Spades,
            'c' | 'C' => suit = Suit::Clubs,
            'h' | 'H' => suit = Suit::Hearts,
            'd' | 'D' => suit = Suit::Diamonds,
            _ => return Err(CardParseError::new(format!("invalid card suit: {:?}", suit_char))),
        }

        let num: i8;
        match num_str {
            "a" | "A" => num = 1,
            "j" | "J" => num = 11,
            "q" | "Q" => num = 12,
            "k" | "K" => num = 13,
            _ => num = try!(num_str.parse().map_err(|e| CardParseError::new(format!("invalid card number: {} {:?}", e, num_str))))
        }

        if num < 1 || num > 13 {
            Err(CardParseError::new(format!("invalid card: number {:?} is out of range", num_str)))
        } else {
            Ok(Card {
                suit: suit,
                number: num,
            })
        }
    }
}

#[test]
fn test_card_parse() {
    assert_eq!(Card::from_str("ks").unwrap(), Card { number: 13, suit: Suit::Spades });
    assert_eq!(Card::from_str("KS").unwrap(), Card { number: 13, suit: Suit::Spades });
    assert_eq!(Card::from_str("13s").unwrap(), Card { number: 13, suit: Suit::Spades });
    assert!(Card::from_str("5X").is_err());
    assert!(Card::from_str("14s").is_err());
    assert!(Card::from_str("0s").is_err());
    assert!(Card::from_str("barf").is_err());
    assert!(Card::from_str("").is_err());
}
