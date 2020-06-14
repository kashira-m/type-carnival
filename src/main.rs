extern crate termion;


use termion::{clear, cursor, color};
use termion::screen::AlternateScreen;
use termion::event::Key;
use termion::cursor::Goto;
use termion::raw::IntoRawMode;
use std::io::{stdin, stdout, Read, Write};
use termion::input::TermRead;

use device_query::{DeviceQuery, DeviceState, Keycode};

#[test]
fn device_query_works() {
    // device_query
    let device_state = DeviceState::new();
    let keys: Vec<Keycode> = device_state.get_keys();
    println!("Is A pressed? {}", keys.contains(&Keycode::A));
}

fn main() {

    // device_query
    let device_state = DeviceState::new();
    let keys: Vec<Keycode> = device_state.get_keys();
    println!("Is A pressed? {}", keys.contains(&Keycode::A));

    // hard coding
    let mut word_list = ["monji", "nakajima", "rust"];

    // initialize screen
    let mut screen = AlternateScreen::from(stdout().into_raw_mode().unwrap());
    write!(screen, "{}{}", clear::All, termion::cursor::Hide);
    screen.flush().unwrap();

    let stdin = stdin();

    // amek object for  test
    let mut textbox = TextBox::new();

    textbox.draw(&mut screen);
    for c in stdin.keys() {
        match c.unwrap() {
            Key::Ctrl('q') => break,
            Key::Char(' ') => textbox.reset(&mut screen),
            Key::Char(c) => textbox.get_char(c, &mut screen),
            Key::Backspace => textbox.delete_char(&mut screen),
            _ => {},
        }
    }

}


pub trait Drawable {
    fn draw<W: Write>(&mut self, screen: W);
    fn update<W: Write>(&mut self, screen: W);
}

struct TextBox {
    box_shape: Vec<String>,
    inputs: String,
    x: u16,
    y: u16,
}

impl TextBox {
    fn new() -> TextBox {
        let box1 = String::from("╔═════════════════╗");
        let input = String::from("");
        let box2 = String::from("╚═════════════════╝");

        TextBox {
            box_shape: vec![box1, box2],
            inputs: input,
            x: 1,
            y: 1,
        }
    }

    fn get_char<W: Write>(&mut self, c: char, screen: W) {
        self.inputs += &c.to_string();
        self.update(screen);
    }

    fn delete_char<W: Write>(&mut self, screen: W) {
        self.inputs.pop();
        self.update(screen);
    }

    fn reset<W: Write>(&mut self, screen: W) {
        let typed_word = &self.inputs;
        self.inputs = String::from("");
        self.update(screen);
    }

}

impl Drawable for TextBox {
    fn draw<W: Write>(&mut self, mut screen: W) {
            write!(screen, "{}{}", Goto(self.x, self.y), self.box_shape[0]).unwrap();
            write!(screen, "{}{}", Goto(self.x, self.y + 1), self.inputs).unwrap();
            write!(screen, "{}{}", Goto(self.x, self.y + 2), self.box_shape[1]).unwrap();
            screen.flush().unwrap();
    }

    fn update<W: Write>(&mut self, mut screen: W) {
        write!(screen, "{}{}", Goto(self.x, self.y), self.box_shape[0]).unwrap();
        write!(screen, "{}{}{}", Goto(self.x, self.y + 1), clear::CurrentLine ,self.inputs).unwrap();
        write!(screen, "{}{}", Goto(self.x, self.y + 2), self.box_shape[1]).unwrap();
        screen.flush().unwrap();
    }
}


struct Word {
    word: String,
    selected: bool,
    hitpoint: i8,
    deleted: bool,
}

impl Word {
    fn new(&mut self, word: String) -> Word {
        Word {
            word,
            selected: true,
            hitpoint: 2,
            deleted: false
        }
    }

    fn typed<W: Write>(&mut self, mut screen: W) {
        self.hitpoint -= 1;
        if self.hitpoint <= 0 {
            self.deleted = true;
        } else {
            self.draw(screen);
        }

    }
}

impl Drawable for Word {
    fn draw<W: Write>(&mut self, mut screen: W) {
        if !self.deleted {
            match self.hitpoint {
                2 => write!(screen, "{}{}{}{}",
                            clear::All,
                            cursor::Goto(1,1),
                            color::Fg(color::Blue),
                            self.word).unwrap(),
                1 => write!(screen, "{}{}{}{}",
                            clear::All,
                            cursor::Goto(1,1),
                            color::Fg(color::Green),
                            self.word).unwrap(),
                _ => {},
            }
        }
    }

    fn update<W: Write>(&mut self, mut screen: W) {
        if !self.deleted {
            match self.hitpoint {
                2 => write!(screen, "{}{}{}{}",
                            clear::All,
                            cursor::Goto(1,1),
                            color::Fg(color::Blue),
                            self.word).unwrap(),
                1 => write!(screen, "{}{}{}{}",
                            clear::All,
                            cursor::Goto(1,1),
                            color::Fg(color::Green),
                            self.word).unwrap(),
                _ => {},
            }
        }
    }
}

fn write_word<W: Write>(mut word: Word, mut screen: W) {
    word.draw(screen);
}