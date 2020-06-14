extern crate termion;
extern crate rand;

// terminal controll crates
use termion::screen::*;
use termion::raw::IntoRawMode;
use std::io::{Read, Write, stdout, stdin, BufReader, BufRead};
use termion::input::TermRead;
use termion::event::Key;
use termion::cursor::Goto;
use termion::async_stdin;

use termion::clear;

// other crates
use std::thread;
use std::time;
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
    fn initgame(&mut self) {
        self.wordholder.get_word();
        self.make_word(3);
        
        self.startgame();
    }

    fn startgame(&mut self) {
        loop {
            println!("{}", clear::All);
            self.draw_object();
            self.update_object();
            self.screen.flush().unwrap();
            thread::sleep(time::Duration::from_secs(1));
        }
    }

    fn draw_object(&mut self) {
        self.wordholder.draw_object(&mut self.screen);
    }

    fn update_object(&mut self) {
        self.wordholder.update_object(&mut self.screen);
    }


    /// make word object {num} times
    fn make_word(&mut self, num: u16) {
        for _ in 0..num {
            let mut word = self.wordholder.make_object(
                self.rng.gen_range(0, self.wordholder.words.len()),
                self.width as u16,
                self.rng.gen_range(1, self.height) as u16);
        }
    }
}

struct WordHolder {
    /// word object holder
    draw_queue: Vec<WordObj>,
    ///
    update_queue: Vec<WordObj>,
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
        self.draw_queue.push(WordObj {
            word: self.words[1].clone(),
            x,
            y,
        });
        self.obj_len += 1;
    }

    fn draw_object<W: Write>(&mut self, screen:&mut W) {
        for _ in 0..self.draw_queue.len() {
            let mut obj: WordObj = self.draw_queue.pop().unwrap();
            obj.draw(screen);
            self.update_queue.push(obj);
        }
    }

    fn update_object<W: Write>(&mut self, screen:&mut W) {
        for index in 0..self.update_queue.len() {
            self.update_queue[index].update(screen);
        }
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
        // write!(screen, "{}{}", Goto(self.x, self.y), self.word);
        self.x -= self.word.len() as u16;
    }

    fn update<W: Write>(&mut self, screen: &mut W) {
        write!(screen, "{}{}", Goto(self.x, self.y), self.word);
        self.x -=1;
    }
}

fn main() {
    initgame(80, 40); 
}

fn initgame(width: usize, height: usize) {

    // initialize game window
    // get in console raw mode
    // let stdin = stdin();
    let mut screen = AlternateScreen::from(stdout().into_raw_mode().unwrap());
    write!(screen, "{}{}", clear::All, termion::cursor::Hide);
    screen.flush().unwrap();

    let mut wordholder = WordHolder {
        draw_queue: vec![],
        update_queue: vec![],
        obj_len: 0,
        words: vec![],
        max_size: 10,

    };

    let mut game = Game {
        width,
        height,
        stdin: async_stdin(),
        screen: screen,
        wordholder,
        speed: 1,
        score: 0,
        rng: rand::thread_rng(),
    };
    game.initgame();
}