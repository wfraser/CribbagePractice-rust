use std::fmt::Write;

use pancurses::*;

use super::card::{Card, Suit};
use super::combo::Combo;
use super::hand::Hand;
use super::ui::{UserInterface, Guess};

pub struct CursesUI {
    main_window: Window,
    card_windows: Vec<Window>,
    arrow_windows: Vec<Window>,
    text_window: Window,
}

const CARD_HEIGHT: usize = 11;
const CARD_WIDTH: usize = 11;

// color pairs
const WHITE_ON_BLACK: i16 = 0;
const RED_ON_BLACK: i16 = 1;

impl CursesUI {
    #[cfg(windows)]
    fn platform_specific_init() {
        // HACK HACK HACK
        // Until Cargo supports specifying the win32 subsystem option to the linker, the program
        // will always be built as a console program, so we have to do this dumb hack to get rid
        // of the unwanted console window that is created when launching the program.
        // Unfortunately, it will still briefly flash in before we are able to call this...
        extern crate kernel32;
        unsafe { kernel32::FreeConsole(); }
    }

    #[cfg(unix)]
    fn platform_specific_init() {
        // Assuming the environment is reasonably modern, this should allow outputting UTF-8 text.
        extern crate libc;
        use std::mem;
        unsafe { libc::setlocale(libc::LC_ALL, mem::transmute(b"\0")) };
    }

    pub fn new(hand_size: usize) -> CursesUI {
        Self::platform_specific_init();
        let main_window = initscr();

        curs_set(0); // hide the cursor
        start_color(); // set up color mode
        main_window.nodelay(false); // use blocking getch
        main_window.keypad(true); // enable getting arrow keys and other special keys
        cbreak(); // raw input
        noecho();

        init_pair(WHITE_ON_BLACK, COLOR_WHITE, COLOR_BLACK);
        init_pair(RED_ON_BLACK, COLOR_RED, COLOR_BLACK);

        let mut card_windows: Vec<Window> = Vec::with_capacity(hand_size);
        let mut arrow_windows: Vec<Window> = Vec::with_capacity(hand_size);

        for i in 0 .. hand_size {
            let cwin = main_window.derwin(CARD_HEIGHT as i32,
                                          CARD_WIDTH as i32,
                                          0,
                                          (i * (CARD_WIDTH + 1)) as i32).unwrap();
            card_windows.push(cwin);

            let awin = main_window.derwin(1,
                                          CARD_WIDTH as i32,
                                          CARD_HEIGHT as i32 + 1,
                                          (i * (CARD_WIDTH + 1)) as i32).unwrap();
            arrow_windows.push(awin);
        }

        let text_window = main_window.derwin(main_window.get_max_y() - CARD_HEIGHT as i32 - 3,
                                             main_window.get_max_x(),
                                             CARD_HEIGHT as i32 + 3,
                                             0).unwrap();

        CursesUI {
            main_window: main_window,
            card_windows: card_windows,
            arrow_windows: arrow_windows,
            text_window: text_window,
        }
    }

    fn render_card(card: &Card) -> String {
        let mut out = String::new();

        let id = match card.number {
            1 => "A".to_owned(),
            11 => "J".to_owned(),
            12 => "Q".to_owned(),
            13 => "K".to_owned(),
            n => format!("{}", n),
        };

        write!(&mut out, "\n {: <2}\n", id).unwrap();

        match card.number {
            1 =>    write!(&mut out, concat!("           ",
                                             "           ",
                                             "           ",
                                             "     X     ",
                                             "           ",
                                             "           ",
                                             "           ")).unwrap(),

            2 =>    write!(&mut out, concat!("           ",
                                             "     X     ",
                                             "           ",
                                             "           ",
                                             "           ",
                                             "     X     ",
                                             "           ")).unwrap(),

            3 =>    write!(&mut out, concat!("           ",
                                             "     X     ",
                                             "           ",
                                             "     X     ",
                                             "           ",
                                             "     X     ",
                                             "           ")).unwrap(),

            4 =>    write!(&mut out, concat!("           ",
                                             "   X   X   ",
                                             "           ",
                                             "           ",
                                             "           ",
                                             "   X   X   ",
                                             "           ")).unwrap(),

            5 =>    write!(&mut out, concat!("           ",
                                             "   X   X   ",
                                             "           ",
                                             "     X     ",
                                             "           ",
                                             "   X   X   ",
                                             "           ")).unwrap(),

            6 =>    write!(&mut out, concat!("           ",
                                             "   X   X   ",
                                             "           ",
                                             "   X   X   ",
                                             "           ",
                                             "   X   X   ",
                                             "           ")).unwrap(),

            7 =>    write!(&mut out, concat!("           ",
                                             "   X   X   ",
                                             "     X     ",
                                             "   X   X   ",
                                             "           ",
                                             "   X   X   ",
                                             "           ")).unwrap(),

            8 =>    write!(&mut out, concat!("           ",
                                             "   X   X   ",
                                             "     X     ",
                                             "   X   X   ",
                                             "     X     ",
                                             "   X   X   ",
                                             "           ")).unwrap(),

            9 =>    write!(&mut out, concat!("   X   X   ",
                                             "           ",
                                             "   X   X   ",
                                             "     X     ",
                                             "   X   X   ",
                                             "           ",
                                             "   X   X   ")).unwrap(),

            10 =>   write!(&mut out, concat!("   X   X   ",
                                             "     X     ",
                                             "   X   X   ",
                                             "           ",
                                             "   X   X   ",
                                             "     X     ",
                                             "   X   X   ")).unwrap(),

            11 =>   write!(&mut out, concat!("      XXX  ",
                                             "       XX  ",
                                             "       XX  ",
                                             "       XX  ",
                                             "       XX  ",
                                             "  XX   XX  ",
                                             "  XXXXXXX  ")).unwrap(),

            12 =>   write!(&mut out, concat!("  XXXXXXX  ",
                                             " XX     XX ",
                                             " XX     XX ",
                                             " XX     XX ",
                                             " XX  XX XX ",
                                             " XX   XXXX ",
                                             "  XXXXXXX  ")).unwrap(),

            13 =>   write!(&mut out, concat!("  XX   XX  ",
                                             "  XX  XX   ",
                                             "  XX XX    ",
                                             "  XXX      ",
                                             "  XX XX    ",
                                             "  XX  XX   ",
                                             "  XX   XX  ")).unwrap(),
            _ => unreachable!(),
        };

        write!(&mut out, "        {: >2}", id).unwrap();

        let suit = match card.suit {
            Suit::Spades    => "♠",
            Suit::Clubs     => "♣",
            Suit::Hearts    => "♥",
            Suit::Diamonds  => "♦",
        };
        out.replace('X', suit)
    }

    fn clear(&mut self) {
        for win in &self.card_windows {
            win.draw_box('|', '-');
            win.refresh();
        }
        for win in &self.arrow_windows {
            win.bkgd(' '.to_chtype());
            win.refresh();
        }
        self.text_window.erase();
        self.text_window.refresh();
    }
}

impl Drop for CursesUI {
    fn drop(&mut self) {
        endwin();
    }
}

// present in pancurses master but not 0.3:

#[cfg(windows)]
const A_DIM: chtype = 0x8000_0000;

#[cfg(unix)]
const A_DIM: chtype = 0x0010_0000;

impl UserInterface for CursesUI {
    fn display_hand(&mut self, hand: &Hand) {
        for (i, card) in hand.cards.iter().enumerate() {
            let rendered = Self::render_card(card);
            let win = &self.card_windows[i];

            match card.suit {
                Suit::Hearts | Suit::Diamonds => {
                    win.attron(COLOR_PAIR(RED_ON_BLACK as chtype));
                },
                _ => {
                    win.attron(COLOR_PAIR(WHITE_ON_BLACK as chtype));
                },
            }

            win.addstr(&rendered);

            win.attrset(COLOR_PAIR(WHITE_ON_BLACK as chtype) | A_DIM);
            win.draw_box('|', '-');

            self.arrow_windows[i].bkgd(' '.to_chtype());
            self.arrow_windows[i].refresh();
        }
    }

    fn display_missed_combos(&mut self, combos: &[Combo]) {
        // TODO
    }

    fn display_win_message(&mut self, score: i32) {
        // TODO
    }

    fn display_lose_message(&mut self, score: i32) {
        // TODO
    }

    fn display_bad_guess_wrong_score(&mut self, actual_combo: &Combo) {
        // TODO
    }

    fn display_bad_guess_invalid_combo(&mut self) {
        // TODO
    }

    fn display_correct_guess(&mut self, combo: &Combo) {
        // TODO
    }

    fn add_score_player(&mut self, score: i32) {
        // TODO
    }

    fn add_score_cpu(&mut self, score: i32) {
        // TODO
    }

    fn get_guess(&mut self, hand: &Hand) -> Option<Guess> {

        self.clear();
        
        self.text_window.printw("Use arrow keys to select cards. Press enter to mark selected card as part of a combo.\n");
        self.text_window.refresh();

        let done_text = "  Done.  ";
        let done_win = self.text_window.derwin(3, done_text.len() as i32, 2, 0).unwrap();
        done_win.mvaddstr(1, 0, done_text);
        done_win.draw_box('#', '#'); // initially selected
        done_win.refresh();

        let mut card_idx: i32 = -1;
        let mut cards: Vec<Card> = vec![];

        loop {
            let input = self.main_window.getch();

            {
                let win = if card_idx == -1 {
                    &done_win
                } else {
                    &self.card_windows[card_idx as usize]
                };
                
                win.draw_box('|', '-');
                win.refresh();
            }

            match input {
                Some(Input::KeyLeft) => { card_idx -= 1; },
                Some(Input::KeyRight) => { card_idx += 1; },
                Some(Input::Character('\n')) => {
                    if card_idx == -1 {
                        break;
                    } else {
                        let ref card = hand.cards[card_idx as usize];
                        let ref arrow_win = self.arrow_windows[card_idx as usize];
                        if let Some(pos) = cards.iter().position(|x| x == card) {
                            cards.remove(pos);
                            arrow_win.bkgd(' '.to_chtype());
                            self.text_window.printw("erase ");
                        } else {
                            cards.push(card.clone());
                            arrow_win.bkgd('^'.to_chtype());
                        }
                        arrow_win.refresh();
                    }
                },
                _ => (),
            };
            if card_idx == -2 {
                card_idx = hand.cards.len() as i32 - 1;
            } else if card_idx == hand.cards.len() as i32 {
                card_idx = -1;
            }

            {
                let win = if card_idx == -1 {
                    &done_win
                } else {
                    &self.card_windows[card_idx as usize]
                };

                win.draw_box('#', '#');
                win.refresh();
            }
            self.text_window.refresh();
            self.main_window.refresh();
        }

        delwin(done_win);

        if cards.is_empty() {
            None
        } else {
            Some(Guess {
                cards: cards,
                score: 0,
            })
        }
    }
}
