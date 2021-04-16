use proconio::input;
#[allow(unused_imports)]
use proconio::marker::{Chars, Bytes};

fn main() {
    input! {
        s: String
    }

    let mut ans = 0;

    if s.contains("RRR") {
        ans = 3
    }
    else if s.contains("RR"){
        ans = 2
    }
    else if s.contains("R"){
        ans = 1
    }

    println!("{}", ans);
}
