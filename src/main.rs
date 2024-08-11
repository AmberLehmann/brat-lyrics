use std::io::{Write, stdout};
// use crossterm::style::Stylize;
use crossterm::terminal::{self, Clear, ClearType};
use crossterm::QueueableCommand;
use crossterm::cursor::{MoveTo, Hide, Show};
// use crossterm::event::{read, poll, Event};
use std::time::Duration;
use std::thread;
use std::fs;

fn print_word(word: String, delay: u64) -> () {
    let mut stdout = stdout();
    let (width, height) = terminal::size().unwrap();
    stdout.queue(Clear(ClearType::All)).unwrap();
    stdout.queue(MoveTo(width/2, height/2)).unwrap();
    stdout.queue(Hide).unwrap();
    stdout.write(word.as_bytes()).unwrap();
    stdout.flush().unwrap();
    thread::sleep(Duration::from_millis(delay));
}

fn finish() -> () {
    let mut stdout = stdout();
    stdout.queue(Show).unwrap();
}

fn main() {
    let raw_lyrics = fs::read_to_string("./b2b")
        .expect("Should have been able to read the file");
    let lyrics = raw_lyrics
        .split_whitespace()
        .map(|word| word.to_uppercase());

    let raw_word_delay = fs::read_to_string("./b2b_delay")
        .expect("Should have been able to read the file");
    let mut word_delay = raw_word_delay
        .split_whitespace()
        .map(|word| word.parse::<u64>().ok())
        .into_iter();
    let raw_ws_delay = fs::read_to_string("./b2b_whitespace_delay")
        .expect("Should have been able to read the file");
    let mut ws_delay = raw_ws_delay
        .split_whitespace()
        .map(|word| word.parse::<u64>().ok())
        .into_iter();

    for word in lyrics {
        print_word(" ".to_owned(), ws_delay.
        next().unwrap_or(Some(2)).unwrap_or(2));
        print_word(word, word_delay.
            next().unwrap_or(Some(2)).unwrap_or(2));
    }
    finish();
}
