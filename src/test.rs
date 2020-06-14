extern crate termion;


use termion::{clear, cursor, color};
use termion::screen::AlternateScreen;
use termion::event::Key;
use termion::cursor::Goto;
use termion::raw::IntoRawMode;
use std::io::{BufWriter, BufReader, stdin, stdout, Read, Write};
use termion::input::TermRead;

fn main() {
    let buffer: Vec<Vec<char>> = vec![vec![]];

    let mut screen = AlternateScreen::from(stdout().into_raw_mode().unwrap());
    write!(screen, "{}{}", clear::All, termion::cursor::Hide);
    screen.flush().unwrap();
}