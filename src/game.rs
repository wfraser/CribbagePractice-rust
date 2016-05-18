// Game :: main game loop
//
// Copyright (c) 2016 by William R. Fraser
//

use std::cell::RefCell;

use super::deck::Deck;
use super::ui::UserInterface;

const SCORE_BAD_GUESS_WRONG_SCORE: i32 = 1;
const SCORE_BAD_GUESS_INVALID_COMBO: i32 = 2;

pub struct Game<'a, UI: UserInterface + 'a> {
    ui: &'a RefCell<UI>,
}

impl<'a, UI: UserInterface> Game<'a, UI> {
    pub fn new(rc_ui: &'a RefCell<UI>) -> Game<'a, UI> {
        Game {
            ui: rc_ui,
        }
    }

    pub fn play(&mut self, deck: &Deck) {
        let mut ui = self.ui.borrow_mut();

        let hand = deck.deal_hand(5);
        ui.display_hand(&hand);

        let mut combos = hand.find_all_combos();

        let mut player_score = 0;
        loop {
            match ui.get_guess(&hand) {
                Some(guess) => {
                    if let Some(index) = combos.iter().position(|x| x == &guess.cards[..]) {
                        {
                            let combo = &combos[index];
                            if combo.score == guess.score {
                                ui.display_correct_guess(&combo);
                                player_score += combo.score as i32;
                            } else {
                                ui.display_bad_guess_wrong_score(&combo);
                                ui.add_score_cpu(SCORE_BAD_GUESS_WRONG_SCORE);
                            }
                        }
                        combos.remove(index);
                    } else {
                        ui.display_bad_guess_invalid_combo();
                        ui.add_score_cpu(SCORE_BAD_GUESS_INVALID_COMBO);
                    }
                },
                None => {
                    if combos.len() == 0 {
                        ui.display_win_message(player_score);
                        ui.add_score_player(player_score);
                    } else {
                        ui.display_missed_combos(&combos);
                        let score = combos.iter().fold(0i32, |score, combo| score + combo.score as i32);
                        ui.display_lose_message(score);
                        ui.add_score_cpu(score);
                    }

                    break;
                }
            }
        }
    }
}
