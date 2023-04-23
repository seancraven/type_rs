use console::{style, Key, Term};
use std::collections::LinkedList;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines, Write};
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

use clap::Parser;

fn main() {
    let args = Args::parse();
    let test_string = "Tring to see what I should be typin string";
    let term = Term::stdout();
    let file = File::open(args.file_name).expect("Failed to open file");
    term.write_line("Press any key to start")
        .expect("Failed to write to terminal");
    term.read_char().expect("Failed to read input");
    let mut errors = 0;
    for window in FileLinesGenerator::new(file, args.lines) {
        term.clear_screen().expect("Failed to clear screen");
        let line_to_type = window.iter().next().unwrap();
        term.write_line(line_to_type)
            .expect("Failed to write to terminal");
        window.iter().skip(1).for_each(|line| {
            term.write_line(line).expect("Failed to write to terminal");
        });
        errors += type_line(&line_to_type, &term, args.lines).expect("Failed to type line");
    }
    term.clear_screen().expect("Failed to clear screen");
    term.write_line(&format!("You made {} errors", errors))
        .expect("Failed to write to terminal");
}

fn type_line(line: &String, term: &Term, window_size: usize) -> Result<u64, io::Error> {
    let mut errors = 0;
    let mut char_iter = line.chars().skip(0);
    term.move_cursor_up(window_size)?;
    let mut idx = 0;
    while let Some(char) = char_iter.next() {
        let user_char = match term.read_key()? {
            Key::Char(c) => Some(c),
            Key::Backspace => {
                term.move_cursor_left(1)?;
                char_iter = line.chars().skip(idx - 1);
                None
            }
            _ => None,
        };
        if let Some(user_char) = user_char {
            if user_char == char {
                term.write_str(&format!("{}", style(char).green()))?;
            } else {
                term.write_str(&format!("{}", style(user_char).red()))?;
            }
            idx += 1;
        }
    }
    while term.read_key().unwrap() != Key::Enter {
        errors += 1;
    }
    return Ok(errors);
}

/// Generates a window of lines from a file.
#[derive(Debug)]
struct FileLinesGenerator {
    //
    current_window: LinkedList<String>,
    lines_iter: Lines<BufReader<File>>,
}
impl FileLinesGenerator {
    /// Create a new FileLinesGenerator. With the first window
    /// populate.
    ///
    /// # Arguments
    /// * `file` - File to read from
    /// * `window_size` - Number of lines to show to the user
    ///
    /// # Returns
    /// * `FileLinesGenerator` - Struct to read lines from file
    fn new(file: File, window_size: usize) -> Self {
        let mut lines_iter = BufReader::new(file).lines();
        let mut current_window = LinkedList::new();
        for _ in 0..window_size {
            match lines_iter.next() {
                Some(line) => {
                    if let Ok(line) = line {
                        current_window.push_back(line);
                    }
                }
                None => break,
            }
        }
        return Self {
            current_window,
            lines_iter,
        };
    }
}
impl Iterator for FileLinesGenerator {
    type Item = LinkedList<String>;
    fn next(&mut self) -> Option<Self::Item> {
        // returns slice of current_chunk.
        // If current_chunk will be_empty, load next chunk before
        // returning.
        self.current_window.pop_front();
        match self.lines_iter.next() {
            Some(line) => {
                if let Ok(line) = line {
                    self.current_window.push_back(line);
                    return Some(self.current_window.clone());
                } else {
                    return Some(self.current_window.clone());
                }
            }
            None => {
                if self.current_window.len() > 0 {
                    return Some(self.current_window.clone());
                } else {
                    return None;
                }
            }
        }
    }
}

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    file_name: PathBuf,

    #[arg(short, long, default_value = "3")]
    lines: usize,
}
