// pancurses_ext :: Things missing from pancurses.
//
// Copyright (c) 2016 by William R. Fraser
//

pub mod pancurses {
    pub use ::pancurses::*;

    #[cfg(windows)]
    use pdcurses as curses;

    #[cfg(unix)]
    use ncurses::ll as curses;

    pub fn echo() {
        unsafe { curses::echo() };
    }

    pub fn mvwgetnstr(w: &Window, y: i32, x: i32, n: usize) -> String {
        let (wy, wx) = match w.get_beg_yx() {
            (wy, wx) => (wy + y, wx + x),
        };
        let mut buf: Vec<u8> = Vec::with_capacity(n);
        buf.resize(n, 0);
        unsafe {
            curses::mvgetnstr(wy, wx, ::std::mem::transmute(buf.as_mut_ptr()), n as i32);
            String::from_utf8_unchecked(buf)
        }
    }
}
