use proconio::input;
#[allow(unused_imports)]
use proconio::marker::{Chars, Bytes};

fn main() {
    input! {
        mut x: i64,
        k: i64,
        d: i64
    }

    let ans;

    if x < 0{
        x = -x
    }

    let i = x / d;

    if i >= k {
        ans = x - k*d;
        println!("{}", ans);
        return
    }

    if (k - i) % 2 == 0 {
        ans = x - i * d
    }
    else {
        ans = -(x - (i + 1) * d)
    }
    // dbg!(x - i * d);
    // dbg!(x - (i + 1) * d);

    println!("{}", ans);
}