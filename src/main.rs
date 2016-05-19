// Main program entry point and user interface functions.
//
// Copyright (c) 2016 by William R. Fraser
//

use std::cell::RefCell;
use std::io::{self, Write};
use std::str::FromStr;

extern crate pancurses;
extern crate rand;

mod card;
mod combo;
mod curses_ui;
mod deck;
mod game;
mod hand;
mod ui;
mod util;

use card::{Card, Suit};
use combo::Combo;
use curses_ui::CursesUI;
use deck::Deck;
use game::Game;
use hand::Hand;
use ui::{UserInterface, Guess};

struct ConsoleUI {
    pub player_score: i32,
    pub cpu_score: i32,
}

impl UserInterface for ConsoleUI {
    fn display_hand(&mut self, hand: &Hand) {
        println!("{}", hand);
    }

    fn display_missed_combos(&mut self, combos: &[Combo]) {
        println!("You missed some:");
        for combo in combos {
            for (idx, card) in combo.cards.iter().enumerate() {
                print!("{}", card);
                if idx < combo.cards.len() - 1 {
                    print!(" ");
                }
            }
            println!(": {} points for a {}", combo.score, combo.text);
        }
    }

    fn display_win_message(&mut self, score: i32) {
        println!("Aww yiss! {} points for you!", score);
    }

    fn display_lose_message(&mut self, score: i32) {
        println!("Computer gets muggins of {} points.", score);
    }

    fn display_bad_guess_wrong_score(&mut self, actual_combo: &Combo) {
        println!("Nope, score is {} for a {}.", actual_combo.score, actual_combo.text);
    }

    fn display_bad_guess_invalid_combo(&mut self) {
        println!("Nope! That's nothing.");
    }

    fn display_correct_guess(&mut self, combo: &Combo) {
        println!("Correct! {} points for a {}.", combo.score, combo.text);
    }

    fn add_score_player(&mut self, score: i32) {
        self.player_score += score;
    }

    fn add_score_cpu(&mut self, score: i32) {
        self.cpu_score += score;
    }

    fn get_guess(&mut self, hand: &Hand) -> Option<Guess> {
        let mut guess = Guess {
            cards: vec![],
            score: 0,
        };

        let mut line = String::new();
        loop {
            line.clear();
            io::stdin().read_line(&mut line).unwrap();

            let trimmed = line.trim();
            if trimmed.is_empty() {
                break;
            }

            let mut parts: Vec<&str> = trimmed.split(' ').collect();
            if let Ok(score) = parts.iter().last().unwrap().parse::<i8>() {
                guess.score = score;
                parts.pop();
            } else {
                print!("Score? ");
                io::stdout().flush().unwrap();
                let mut score_line = String::new();
                io::stdin().read_line(&mut score_line).unwrap();
                if let Ok(score) = score_line.trim().parse::<i8>() {
                    guess.score = score;
                } else {
                    println!("invalid number");
                    continue;
                }
            }

            guess.cards.clear();
            let mut valid_cards = true;
            for x in parts {
                match Card::from_str(x) {
                    Ok(card) => {
                        if guess.cards.contains(&card) {
                            println!("You typed {} twice!", card);
                            valid_cards = false;
                            break;
                        }
                        if !hand.cards.contains(&card) {
                            println!("That card isn't in your hand!");
                            valid_cards = false;
                            break;
                        }
                        guess.cards.push(card);
                    },
                    Err(e) => {
                        println!("{}", e);
                        valid_cards = false;
                        break;
                    }
                }
            }

            if !valid_cards {
                continue;
            }

            break;
        }

        if guess.cards.is_empty() {
            None
        } else {
            Some(guess)
        }
    }
}

fn main() {
    let ui = RefCell::new(ConsoleUI {
        player_score: 0,
        cpu_score: 0
    });

    let curses_ui = CursesUI::new(5);

    let mut game = Game::new(&ui);

    let mut deck = Deck::new();
    loop {
        println!("");

        deck.shuffle();
        game.play(&deck);

        let ui = ui.borrow();
        println!("Score total: You: {}", ui.player_score);
        println!("        Computer: {}", ui.cpu_score);
        print!("Play again? [y/n] ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        if !line.to_lowercase().starts_with("y") {
            break;
        }
    }
}
