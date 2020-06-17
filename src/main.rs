extern crate termion;


use termion::{clear, cursor, color};
use termion::screen::AlternateScreen;
use termion::event::Key;
use termion::cursor::Goto;
use termion::raw::IntoRawMode;
use std::io::{stdin, stdout, Read, Write, BufReader, BufRead};
use termion::input::TermRead;

use std::fs::File;

use device_query::{DeviceQuery, DeviceState, Keycode};


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
}

impl<W: Write> Game<W> {
    fn new(screen: W) -> Game<W> {
        Game {
            inputbox: InputBox::new(),
            wordholder: WordHolder::new("word.txt"),
            screen,
        }
    }

    fn init_game(mut self) {
        self.draw();
        self.game_start();
    }

    fn game_start(&mut self) {
        use std::io::stdin;
        let stdin = stdin();
        for c in stdin.keys() {
            match c.unwrap() {
                Key::Ctrl('q') => break,
                Key::Char(' ') => self.inputbox.reset(),
                Key::Char(c) => self.inputbox.get_char(c),
                Key::Backspace => self.inputbox.delete_char(),
                _ => {},
            }
            self.draw();
            self.screen.flush().unwrap();
        }
    }

    fn update(&mut self) -> bool {
        
        let mut key_bytes = [0];

        match  key_bytes[0] {
            b' ' => self.inputbox.reset(),
            _ => {},
        }
    
        true
    }

    fn draw(&mut self) {
        self.inputbox.draw(&mut self.screen);
        self.wordholder.typable.draw(&mut self.screen);
    }
}

struct WordHolder {
    /// usable word list from file
    word_list: Vec<String>,
    ///
    typable: Word,
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

        let mut typable = Word::new(String::from(words.pop().unwrap()));
        WordHolder {
            word_list: words,
            typable,
        }
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
    fn new() -> InputBox {
        let box1 = String::from("╔═════════════════╗");
        let input = String::from("");
        let box2 = String::from("╚═════════════════╝");

        InputBox {
            box_shape: vec![box1, box2],
            inputs: input,
            x: 1,
            y: 1,
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
    fn new(word: String) -> Word {
        Word {
            word,
            selected: true,
            hitpoint: 2,
            deleted: false
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

#[test]
fn device_query_works() {
    // device_query
    let device_state = DeviceState::new();
    let keys: Vec<Keycode> = device_state.get_keys();
    println!("Is A pressed? {}", keys.contains(&Keycode::A));
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