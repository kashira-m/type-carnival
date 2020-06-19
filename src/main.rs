#![feature(optin_builtin_traits)]


extern crate termion;

use termion::{clear, cursor, color, async_stdin};
use termion::screen::AlternateScreen;
use termion::event::Key;
use termion::cursor::Goto;
use termion::raw::IntoRawMode;
use std::io::{stdin, stdout, Read, Write, BufReader, BufRead};
use termion::input::TermRead;

use std::fs::File;

use rand::Rng;

use std::thread;
use std::time;
use std::sync::{Arc, Mutex, mpsc};

use futures::prelude::*;

struct Keys {
    key: termion::event::Key,
}


const top_indent: u16 = 3;
const bottom_indent: u16 = 4;

#[test]
fn key_code_test() {
    let mut stdin = stdin();
    let mut screen = AlternateScreen::from(stdout().into_raw_mode().unwrap());
    write!(screen, "{}{}{}", clear::All, termion::cursor::Hide, Goto(1,1));
    screen.flush().unwrap();
    let mut key_bytes = [0];
    loop {
        stdin.read(&mut key_bytes).unwrap();
        match key_bytes[0] {
            b' ' => break,
            10 => write!(screen, "{}", 10 as char).unwrap(),
            _ => write!(screen, "{}",key_bytes[0] as char).unwrap(),
        }
        screen.flush().unwrap();
    }
}

fn main() {
    // initialize screen
    let mut screen = AlternateScreen::from(stdout().into_raw_mode().unwrap());
    write!(screen, "{}{}", clear::All, termion::cursor::Hide);
    screen.flush().unwrap();


    let mut game = Game::new(screen);
    
    game.init_game();
}


struct Game<W> {
    inputbox: InputBox,
    wordholder: WordHolder,
    screen: W,
    termsize: (u16, u16),
    score: i32
}

impl<W: Write> Game<W> {
    fn new(screen: W) -> Game<W> {
        let termsize = termion::terminal_size().unwrap();
        Game {
            inputbox: InputBox::new(termsize),
            wordholder: WordHolder::new("word.txt"),
            screen,
            termsize,
            score: 0,
        }
    }

    fn init_game(mut self) {
        self.draw();
        self.game_start();
    }

    fn game_start(&mut self) {
        let mut stdin = stdin();
            for c in stdin.keys() {
                match c.unwrap() {
                    Key::Ctrl('q') => break,
                    Key::Char(' ') => self.compare_result(),
                    Key::Char(c) => self.get_input(c),
                    Key::Backspace => self.inputbox.delete_char(),
                    _ => {},
                }
            self.update();
        }
    }

    fn add_score(&mut self, score: i32) {
        self.score += score;
    }

    fn compare_result(&mut self) {
        if self.inputbox.inputs == self.wordholder.typable.word {
            self.wordholder.change_typable(self.termsize);
            self.add_score(2);
        } else {

        }
        self.inputbox.reset();
    }
    fn get_input(&mut self, c: char) {
        self.inputbox.get_char(c);
        self.wordholder.move_forward();
        if !self.wordholder.wordzone(self.termsize.0) {
            self.gameover();
        }
    }

    fn gameover(&mut self) {
        self.wordholder.typable = Word::new(String::from("Game Over"), self.termsize.1/2);
    }

    fn update(&mut self) {
        write!(self.screen, "{}", clear::All).unwrap();
        
        write!(self.screen, "{}{}{}", color::Fg(color::White), Goto(self.termsize.0 -1, 2), self.score).unwrap();
        for i in 1..self.termsize.0 {
            write!(self.screen, "{}{}", Goto(i, top_indent), "_").unwrap();
            write!(self.screen, "{}{}", Goto(i, self.termsize.1 - bottom_indent), "_").unwrap()
        }

        self.inputbox.update(&mut self.screen);
        self.wordholder.typable.update(&mut self.screen);

        self.screen.flush().unwrap();
    }

    fn draw(&mut self) {
        write!(self.screen, "{}{}", Goto(self.termsize.0 -1, 2), self.score).unwrap();
        for i in 1..self.termsize.0 {
            write!(self.screen, "{}{}", Goto(i, top_indent), "_").unwrap();
            write!(self.screen, "{}{}", Goto(i, self.termsize.1 - bottom_indent), "_").unwrap()
        }
        self.inputbox.draw(&mut self.screen);
        self.wordholder.typable.draw(&mut self.screen);

        self.screen.flush().unwrap();
    }
}

struct WordHolder {
    /// usable word list from file
    word_list: Vec<String>,
    ///
    typable: Word,
    /// random number generator for word place
    rng: rand::rngs::ThreadRng,
}

impl WordHolder {
    fn new(filepath: &str) -> WordHolder {
        let mut words: Vec<String> = vec![];
        let reader = BufReader::new(File::open(filepath).expect("cannot open word collection"));
        for line in reader.lines() {
            for word in line.unwrap().split_whitespace() {
                words.push(String::from(word));   
            }
        }

        let termsize = termion::terminal_size().unwrap();
        let mut rng = rand::thread_rng();


        let mut typable = Word::new(String::from(words.pop().unwrap()), rng.gen_range(top_indent + 1,termsize.1 - bottom_indent));
        WordHolder {
            word_list: words,
            typable,
            rng,
        }
    }

    fn change_typable(&mut self, termsize: (u16, u16)) {
        self.typable = Word::new(self.word_list.pop().unwrap() ,self.rng.gen_range(top_indent + 1,termsize.1 - bottom_indent));
    }

    fn move_forward(&mut self) {
        self.typable.x += 1;
    }

    fn wordzone(&mut self, max_x:u16) -> bool {
        if self.typable.x + self.typable.word.len() as u16 > max_x {
            return false
        }
        true
    }
}

pub trait Drawable {
    fn draw<W: Write>(&mut self, screen: W);
    fn update<W: Write>(&mut self, screen: W);
}

struct InputBox {
    box_shape: Vec<String>,
    inputs: String,
    x: u16,
    y: u16,
}

impl InputBox {
    fn new(termsize: (u16, u16)) -> InputBox {
        let box1 = String::from("╔══════════════════════════════════╗");
        let input = String::from("");
        let box2 = String::from("╚══════════════════════════════════╝");
        let x = termsize.0 - 35;
        let y = termsize.1 - 3;
        InputBox {
            box_shape: vec![box1, box2],
            inputs: input,
            x,
            y,
        }
    }

    fn get_char(&mut self, c: char) {
        self.inputs += &c.to_string();
        //self.update(screen);
    }

    fn delete_char(&mut self) {
        self.inputs.pop();
        //self.update(screen);
    }

    fn reset(&mut self) {
        let typed_word = &self.inputs;
        self.inputs = String::from("");
    }

}

impl Drawable for InputBox {
    fn draw<W: Write>(&mut self, mut screen: W) {
        write!(screen, "{}", color::Fg(color::White));
        write!(screen, "{}{}", Goto(self.x, self.y), self.box_shape[0]).unwrap();
        write!(screen, "{}{}", Goto(self.x, self.y + 1), self.inputs).unwrap();
        write!(screen, "{}{}", Goto(self.x, self.y + 2), self.box_shape[1]).unwrap();
    }

    fn update<W: Write>(&mut self, mut screen: W) {
        write!(screen, "{}", color::Fg(color::White));
        write!(screen, "{}{}", Goto(self.x, self.y), self.box_shape[0]).unwrap();
        write!(screen, "{}{}{}", Goto(self.x, self.y + 1), clear::CurrentLine ,self.inputs).unwrap();
        write!(screen, "{}{}", Goto(self.x, self.y + 2), self.box_shape[1]).unwrap();

    }
}


struct Word {
    word: String,
    selected: bool,
    hitpoint: i8,
    deleted: bool,
    x: u16,
    y: u16,
}

impl Word {
    fn new(word: String, y: u16) -> Word {
        
        Word {
            word,
            selected: true,
            hitpoint: 2,
            deleted: false,
            x:1,
            y,
        }
    }

    fn typed(&mut self) {
        self.hitpoint -= 1;
        if self.hitpoint <= 0 {
            self.deleted = true;
        }
    }
}

impl Drawable for Word {
    fn draw<W: Write>(&mut self, mut screen: W) {
        if !self.deleted {
            match self.hitpoint {
                2 => write!(screen, "{}{}{}",
                            cursor::Goto(self.x,self.y),
                            color::Fg(color::Cyan),
                            self.word).unwrap(),
                1 => write!(screen, "{}{}{}",
                            cursor::Goto(self.x,self.y),
                            color::Fg(color::Green),
                            self.word).unwrap(),
                _ => {},
            }
        }
    }

    fn update<W: Write>(&mut self, mut screen: W) {
        if !self.deleted {
            match self.hitpoint {
                2 => write!(screen, "{}{}{}",
                            cursor::Goto(self.x,self.y),
                            color::Fg(color::Yellow),
                            self.word).unwrap(),
                1 => write!(screen, "{}{}{}",
                            cursor::Goto(self.x,self.y),
                            color::Fg(color::Green),
                            self.word).unwrap(),
                _ => {},
            }
        }
    }
}

#[test]
fn byte_read_test() {
    let mut key_bytes = [0];
    let mut stdin = stdin();

    stdin.read(&mut key_bytes).unwrap();
    let mut key = "";
    match key_bytes[0] {
        b'k' | b'w' => {key = "k"},
        b'j' | b's' => {key = "j"},
        b'h' | b'a' => {key = "h"},
        b'l' | b'd' => {key = "l"},
        _ => {},
    }
    println!("{}", key);

}