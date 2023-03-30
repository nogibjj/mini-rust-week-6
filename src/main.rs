use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: word_counter <file_path>");
        return;
    }

    let file_path = &args[1];
    match count_words(file_path) {
        Ok(word_count) => println!("The file contains {} words.", word_count),
        Err(error) => println!("Error: {}", error),
    }
}

fn count_words<P: AsRef<Path>>(path: P) -> io::Result<usize> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut word_count = 0;
    for line in reader.lines() {
        let line = line?;
        word_count += line.split_whitespace().count();
    }

    Ok(word_count)
}
