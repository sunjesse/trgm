use std::collections::{HashMap, HashSet};

/*
Implementation of postgresql trigram implementation.
It pads the head with 2 empty spaces, and tail with
1 empty space.
*/

#[derive(Debug)]
pub struct Trigrams {
    cache: HashMap<String, HashSet<u32>>,
    vocab: Vec<String>,
    threshold: f32,
}

impl Trigrams {
    pub fn new(threshold: f32) -> Self {
        Trigrams {
            cache: HashMap::new(),
            vocab: Vec::new(),
            threshold: threshold,
        }
    }

    pub fn add_vocab(&mut self, vocab: Vec<String>) {
        let _: Vec<_> = vocab.iter().map(|x| self.get_trgm(x)).collect();
        self.vocab = vocab;
    }

    pub fn get_trgm(&mut self, s: &String) -> HashSet<u32> {
        if let Some(val) = self.cache.get(s) {
            return val.clone(); // bad
        }

        let mut ans: HashSet<u32> = HashSet::with_capacity((s.len() + 4) - 2);

        if s.len() == 0 {
            return ans;
        }

        let bytes: &[u8] = (*s).as_bytes();
        let ub: usize = bytes.len() - 1;

        let mut v: u32 = 32u32 << 16 | 32u32 << 8 | (bytes[0] as u32);

        ans.insert(v);

        for i in 0..=ub {
            /* zeros out top 8 bits, and then shift 8 bits left from original position */
            v <<= 16;
            v >>= 8;
            if i < ub {
                v |= bytes[1 + i] as u32;
            } else {
                v |= 32u32; // space
            }
            ans.insert(v);
        }
        self.cache.insert(s.clone(), ans.clone()); // bad
        ans
    }

    pub fn get_scores(&mut self, word: &String) -> Vec<(usize, f32)> {
        let word_trgm: HashSet<u32> = self.get_trgm(word);

        let keys: Vec<&String> = self.vocab.iter().collect();

        let mut scores: Vec<(usize, f32)> = keys
            .iter()
            .enumerate()
            .map(|(i, k)| (i, self.similarity(&word_trgm, self.cache.get(*k).unwrap()))) // k is ref to ref
            .filter(|(_, score)| *score > self.threshold)
            .collect();

        scores.sort_by(|a, b| (-a.1).partial_cmp(&-b.1).unwrap());

        for (i, v) in scores.iter() {
            println!("{:}: {:}", self.vocab[*i], v);
        }

        scores
    }

    pub fn print_cache(&self) {
        /* helper, get rid of soon */
        println!("cache is {:?}", self.cache);
    }

    pub fn similarity(&self, x: &HashSet<u32>, y: &HashSet<u32>) -> f32 {
        let mut u: usize = 0;
        let mut v: usize = x.len();
        for c in y.iter() {
            if let Some(_) = x.get(c) {
                u += 1;
            } else {
                v += 1;
            }
        }
        u as f32 / (v as f32)
    }
}
