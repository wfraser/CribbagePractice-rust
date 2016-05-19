use std::fmt::Write;

use pancurses::*;

use super::card::{Card, Suit};
use super::combo::Combo;
use super::hand::Hand;
use super::ui::{UserInterface, Guess};

pub struct CursesUI {
    main_window: Window,
    card_windows: Vec<Window>,
}

const CARD_HEIGHT: usize = 11;
const CARD_WIDTH: usize = 11;

impl CursesUI {
    #[cfg(windows)]
    fn hide_win32_console() {
        // HACK HACK HACK
        // Until Cargo supports specifying the win32 subsystem option to the linker, the program
        // will always be built as a console program, so we have to do this dumb hack to get rid
        // of the unwanted console window that is created when launching the program.
        // Unfortunately, it will still briefly flash in before we are able to call this...
        extern crate kernel32;
        unsafe { kernel32::FreeConsole(); }
    }

    #[cfg(not(windows))]
    fn hide_win32_console() {
    }

    pub fn new(hand_size: usize) -> CursesUI {
        extern crate libc;
        use std::mem;
        unsafe { libc::setlocale(libc::LC_ALL, mem::transmute(b"\0")) };

        Self::hide_win32_console();
        let main_window = initscr();

        curs_set(0); // hide the cursor
        start_color(); // set up color mode
        main_window.nodelay(false); // use blocking getch
        cbreak(); // raw input

        let mut card_windows: Vec<Window> = Vec::with_capacity(hand_size);

        for i in 0 .. hand_size {
            let win = main_window.derwin(CARD_HEIGHT as i32,
                                         CARD_WIDTH as i32,
                                         0,
                                         (i * (CARD_WIDTH + 1)) as i32).unwrap();
            card_windows.push(win);
        }

        CursesUI {
            main_window: main_window,
            card_windows: card_windows,
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
}

impl Drop for CursesUI {
    fn drop(&mut self) {
        endwin();
    }
}

impl UserInterface for CursesUI {
    fn display_hand(&mut self, hand: &Hand) {
        for (i, card) in hand.cards.iter().enumerate() {
            let rendered = Self::render_card(card);
            self.card_windows[i].addstr(&rendered);
            self.card_windows[i].draw_box('|', '-');
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
        // TODO
        self.main_window.getch();
        None
    }
}
