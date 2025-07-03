mod common;
mod reader;
mod trigram;

use common::parse;
use reader::file_to_words;
use trigram::Trigrams;

use std::io;

fn main() {
    const THRESHOLD: f32 = 0.3;
    let vocab: Vec<String> = file_to_words("./src/data/words.txt");

    let mut trigrams: Trigrams = Trigrams::new(THRESHOLD);
    trigrams.add_vocab(vocab.clone());

    trigrams.print_cache();

    loop {
        println!("Enter word: ");
        let mut word: String = String::new();
        io::stdin().read_line(&mut word).expect("error bruh");
        parse(&mut word);

        if word == "stop" {
            break;
        }

        trigrams.get_scores(&word);
    }
}
