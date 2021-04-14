use proconio::input;
#[allow(unused_imports)]
use proconio::marker::{Chars, Bytes};

fn main() {
    input! {
        n: usize,
        mut l: [i64; n]
    }

    if n < 3{
        println!("0");
        return
    }

    l.sort();

    // dbg!(&l);

    let mut ans = 0;
    for i in 0..n-2{
        for j in i+1..n-1{
            for k in j+1..n{
                if l[i] == l[j] || l[j] == l[k] || l[k] == l[i]{
                    continue
                }
                if l[k] < l[i] + l[j]{
                    ans += 1;
                }
            }
        }
    }

    println!("{}", ans);
}
