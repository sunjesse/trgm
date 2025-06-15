mod trigram;
mod reader;

use trigram::{get_trgm, similarity};
use reader::file_to_words;

use std::io;

#[inline(always)]
fn parse(x: &mut String) {
    let xarr: Vec<char> = x.chars().collect();
    let mut i: usize = xarr.len() - 1;
    while matches!(xarr[i], '\n') {
        x.pop();
        if i == 0 { break; }
        i -= 1;
    }
}

fn main() {
    const THRESHOLD: f32 = 0.3;
    let vocab: Vec<String> = file_to_words("./src/data/words.txt");

    let vocab_trgm: Vec<Vec<String>> = vocab.iter().map(|x| get_trgm(x)).collect();

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
        let mut scores: Vec<(usize, f32)> = vocab_trgm.iter().enumerate()
                                    .map(|(i, x)| 
                                        (i, similarity(&get_trgm(&word), x))
                                    )
                                    .filter(|(_, v)| *v > THRESHOLD)
                                    .collect();

        scores.sort_by(|a, b| (-a.1).partial_cmp(&-b.1).unwrap());
        println!("SIMILAR (threshold := {:})", THRESHOLD);
        for (i, v) in scores.iter() {
            println!("{:}: {:}", vocab[*i], v);
            
        }
    }
    
}
