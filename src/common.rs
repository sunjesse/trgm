/* util functions */

#[inline(always)]
pub fn parse(x: &mut String) {
    let xarr: Vec<char> = x.chars().collect();
    let mut i: usize = xarr.len() - 1;
    while matches!(xarr[i], '\n') {
        x.pop();
        if i == 0 { break; }
        i -= 1;
    }
}
