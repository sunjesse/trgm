use std::collections::HashSet;

/*
Implementation of postgresql trigram implementation.
It pads the head with 2 empty spaces, and tail with
1 empty space.
*/

pub fn get_trgm(s: &String) -> Vec<String> {
    /* naive implementation */
    let mut ans: Vec<String> = Vec::with_capacity((s.len() + 4) - 2);
    let mut _s: String = "  ".to_string();
    _s.push_str(s);
    _s.push_str(" ");
    for i in 0..=(_s.len() - 3) {
        unsafe {
            ans.push(_s.get_unchecked(i..(i + 3)).to_string());
        }
    }
    ans
}

#[inline(always)]
fn hash(c: &String) -> u32 {
    /* we know it's a trigram so there will be 3 chars always */
    let b: &[u8] = c.as_bytes();
    ((b[0] as u32) << 16) | ((b[1] as u32) << 8) | (b[2] as u32)
}

pub fn similarity(x: &Vec<String>, y: &Vec<String>) -> f32 {
    let mut set: HashSet<u32> = HashSet::new();
    let mut u: usize = 0;
    let mut v: usize = x.len();
    for c in x.iter() {
        let key: u32 = hash(c);
        set.insert(key);
    }
    for c in y.iter() {
        let key: u32 = hash(c);
        if let Some(_) = set.get(&key) {
            u += 1;
        } else {
            v += 1;
        }
    }

    u as f32 / (v as f32)
}
