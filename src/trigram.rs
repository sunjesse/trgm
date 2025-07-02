use std::collections::HashSet;

/*
Implementation of postgresql trigram implementation.
It pads the head with 2 empty spaces, and tail with
1 empty space.
*/

pub fn get_trgm(s: &String) -> HashSet<u32> {
    /* naive implementation */
    let mut ans: HashSet<u32> = HashSet::with_capacity((s.len() + 4) - 2);
    let mut _s: String = "  ".to_string();
    _s.push_str(s);
    _s.push_str(" ");
    for i in 0..=(_s.len() - 3) {
        unsafe {
            ans.insert(hash(_s.get_unchecked(i..(i + 3)).to_string()));
        }
    }
    ans
}

#[inline(always)]
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
        b32, b97, b98, b99, b32
    */
    let b: &[u8] = c.as_bytes();
    ((b[0] as u32) << 16) | ((b[1] as u32) << 8) | (b[2] as u32)
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

