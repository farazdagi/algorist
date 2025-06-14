use algorist::io::{test_cases, wln};

fn main() {
    test_cases(&mut |scan, w| {
        let n = scan.u();
        let vals: Vec<i32> = scan.vec(n);

        let ans = vals.len();
        wln!(w, "{}", ans);
    });
}
