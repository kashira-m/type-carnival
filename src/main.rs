extern crate termion;
extern crate rand;

// terminal controll crates
use termion::screen::*;
use termion::raw::IntoRawMode;
use std::io::{Read, Write, stdout, stdin, BufReader, BufRead};
use termion::input::TermRead;
use termion::event::Key;
use termion::cursor::Goto;

use termion::clear;

// other crates
use std::thread;
use std::time::Duration;
// already imported std::io::Read
use std::fs::File;

use rand::Rng;

struct Game<R, W> {
    /// The play area width and height
    width: usize,
    height: usize,
    /// standard input
    stdin: R,
    /// standard output
    screen: W,
    /// word objects
    wordholder: WordHolder,
    /// Speed
    speed: u64,
    /// Score
    score: i32,
    /// The randomizer
    rng: rand::rngs::ThreadRng,
}

impl<R: Read, W: Write> Game<R, W> {
    fn startgame() {

    }

    fn draw_object() {}

    fn make_word(&mut self) {
        let mut word = self.wordholder.make_object(
            self.rng.gen_range(0, self.wordholder.words.len()),
            self.width as u16,
            self.rng.gen_range(1, self.height) as u16);
    }
}

struct WordHolder {
    /// word object holder
    w_objects: Vec<WordObj>,
    /// length of w_object
    obj_len: usize,
    /// max size of word holder(words)
    max_size: u32,
    /// available word list
    words: Vec<String>,
}

impl WordHolder {
    /// Get words from file
    fn get_word(&mut self) {
        let filename = "word.txt";

        let reader = BufReader::new(File::open(filename).expect("cannot open word collection"));
        for line in reader.lines() {
            for word in line.unwrap().split_whitespace() {
                self.words.push(String::from(word));   
            }
        }
    }

    /// Make word onject
    fn make_object(&mut self, num: usize, x: u16, y:u16) {
        self.w_objects.push(WordObj {
            word: self.words[num].clone(),
            x,
            y,
        });
        self.obj_len += 1;
    }
}

struct WordObj {
    word: String,
    x: u16,
    y: u16,
}

impl WordObj {
    fn new(word: String, x: u16, y: u16) -> WordObj {
        WordObj {
            word, x, y,
        }
    }

    fn draw<W: Write>(&mut self, screen: &mut W) {
        write!(screen, "{}{}", Goto(self.x, self.y), self.word);
        self.x -=1;
    }
}

fn main() {
    let mut game = initgame(80, 40); 
}

fn initgame(width: usize, height: usize) {

    // initialize game window
    // get in console raw mode
    let stdin = stdin();
    let mut screen = AlternateScreen::from(stdout().into_raw_mode().unwrap());
    write!(screen, "{}{}", clear::All, termion::cursor::Hide);
    screen.flush().unwrap();

    let mut wordholder = WordHolder {
        w_objects: vec![],
        obj_len: 0,
        words: vec![],
        max_size: 10,

    };

    let mut game = Game {
        width: 40,
        height: 70,
        stdin: stdin,
        screen: screen,
        wordholder,
        speed: 1,
        score: 0,
        rng: rand::thread_rng(),
    };
}

// not used
fn words_by_file(filename: &str) -> Vec<String> {
    let reader = BufReader::new(File::open(filename).expect("cannot open word collection"));
    let mut words: Vec<String> = vec![];
    for line in reader.lines() {
        for word in line.unwrap().split_whitespace() {
            words.push(String::from(word));   
        }
    }
    words
} 

#[test]
fn word_from_file() {
    let filename = "word.txt";
    let reader = BufReader::new(File::open("word.txt").expect("Cannot open file.txt"));

    for line in reader.lines() {
    for word in line.unwrap().split_whitespace() {
        println!("word '{}'", word);
    }
}

}