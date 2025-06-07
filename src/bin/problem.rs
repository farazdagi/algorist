use std::io::{self, Write};

use algorist::io::{Scanner, wln};

fn main() {
    let mut scan = Scanner::new(io::stdin().lock());
    let mut w = io::BufWriter::new(io::stdout().lock());

    scan.test_cases(&mut |scan| {
        let n = scan.u();
        let vals: Vec<i32> = scan.vec(n);

        let ans = vals.len();
        wln!(w, "{}", ans);
    });
}
