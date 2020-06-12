extern crate termion;

use termion::{color, clear};
use std::time::Duration;
use std::thread;

fn main() {
    println!("{red}more red than any comrade{reset}",
             red   = color::Fg(color::Red),
             reset = color::Fg(color::Reset));
    // Sleep for a short period of time.
    thread::sleep(Duration::from_millis(300));
    // Go back;
    println!("\r");
    // Clear the line and print some new stuff
    print!("{clear}{red}g{blue}a{green}y{red} space communism{reset}",
            clear = clear::CurrentLine,
            red   = color::Fg(color::Red),
            blue  = color::Fg(color::Blue),
            green = color::Fg(color::Green),
            reset = color::Fg(color::Reset));
}