mod trigram;

use trigram::{get_trgm, similarity};

use std::io;

#[inline(always)]
fn parse(x: &mut String) {
    /* i don't love the x.chars() call each time */
    while matches!(x.chars().last(), Some('\n')) {
        x.pop();
    }
}

fn main() {
    let mut fixed_wrd: String = String::new();

    println!("Enter fixed word: ");
    io::stdin().read_line(&mut fixed_wrd).expect("error xd");

    parse(&mut fixed_wrd);

    let fixed_wrd_trgm: Vec<String> = get_trgm(&fixed_wrd);

    loop {
        let mut x: String = String::new();
        io::stdin().read_line(&mut x).expect("error bruh");
        parse(&mut x);

        if x == "stop" {
            break;
        }

        println!("{:?}", similarity(&get_trgm(&x), &fixed_wrd_trgm));
    }
}
