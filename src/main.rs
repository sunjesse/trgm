mod common;
mod reader;
mod trigram;

use common::parse;
use reader::file_to_words;
use std::collections::HashSet;
use trigram::{similarity, Trigrams};

use std::io;

fn main() {
    const THRESHOLD: f32 = 0.3;
    let vocab: Vec<String> = file_to_words("./src/data/words.txt");

    let mut trigrams: Trigrams = Trigrams::new();
    trigrams.add_vocab(vocab.clone());

    trigrams.print_cache();

    let vocab_trgm: Vec<HashSet<u32>> = vocab.iter().map(|x| trigrams.get_trgm(x)).collect();

    loop {
        println!("Enter word: ");
        let mut word: String = String::new();
        io::stdin().read_line(&mut word).expect("error bruh");
        parse(&mut word);

        if word == "stop" {
            break;
        }

        /*
        naive implementation, we can do a lot to improve runtime.
        */
        let mut scores: Vec<(usize, f32)> = vocab_trgm
            .iter()
            .enumerate()
            .map(|(i, x)| (i, similarity(&trigrams.get_trgm(&word), x)))
            .filter(|(_, v)| *v > THRESHOLD)
            .collect();

        scores.sort_by(|a, b| (-a.1).partial_cmp(&-b.1).unwrap());
        println!("SIMILAR (threshold := {:})", THRESHOLD);
        for (i, v) in scores.iter() {
            println!("{:}: {:}", vocab[*i], v);
        }
    }
}
