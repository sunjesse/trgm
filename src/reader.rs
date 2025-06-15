use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use crate::common::parse;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn file_to_words(path: &str) -> Vec<String> {
    let mut list_of_words: Vec<String> = Vec::new();
    if let Ok(lines) = read_lines(path) {
        for line in lines.map_while(Result::ok) {
            if !line.is_empty() {
                let split_line: Vec<String> = line
                    .split_whitespace()
                    .map(|x| {
                        let mut w = x.to_string();
                        parse(&mut w);
                        w
                    })
                    .collect::<Vec<String>>();

                for i in 0..split_line.len() {
                    list_of_words.push(split_line[i].clone());
                }
            }
        }
    }
    list_of_words
}
