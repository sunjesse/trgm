use std::collections::{HashMap, HashSet};

/*
Implementation of postgresql trigram implementation.
It pads the head with 2 empty spaces, and tail with
1 empty space.
*/

#[derive(Debug)]
pub struct Trigrams {
    cache: HashMap<String, HashSet<u32>>,
}

impl Trigrams {
    pub fn new() -> Self {
        Trigrams {
            cache: HashMap::new(),
        }
    }

    pub fn add_vocab(&mut self, vocab: Vec<String>) {
        let _: Vec<_> = vocab.iter().map(|x| self.get_trgm(x)).collect();
    }

    pub fn get_trgm(&mut self, s: &String) -> HashSet<u32> {
        if let Some(val) = self.cache.get(s) {
            return val.clone();
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
        self.cache.insert(s.clone(), ans.clone());
        ans
    }

    pub fn print_cache(&self) {
        /* helper, get rid of soon */
        println!("cache is {:?}", self.cache);
    }
}

pub fn similarity(x: &HashSet<u32>, y: &HashSet<u32>) -> f32 {
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

#[inline(always)]
#[allow(dead_code)]
fn hash(c: String) -> u32 {
    /*
    we know it's a trigram so there will be 3 chars always
    Further, we know that the key will be chunked into
    4 8bit segments. We have a sliding window of size 3
    to make up a bit representation of the form
    | 0 | b_1 | b_2 | b_3 |

    Ex:
        'abc' => [' ab', 'abc', 'bc ']
        b[0] = ' ', b[1] = 'a', b[2] = 'b', b[3] = 'c', b[4] = ' '
        32u8, 97u8, 98u8, 99u8, 32u8
    */
    let b: &[u8] = c.as_bytes();
    ((b[0] as u32) << 16) | ((b[1] as u32) << 8) | (b[2] as u32)
}
