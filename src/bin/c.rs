#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        t: usize,
        t_list: [usize; n],
    }
    let mut ans = t;

    for i in 0..t_list.len() - 1 {
        if t_list[i] + t > t_list[i + 1] {
            ans += t_list[i + 1] - t_list[i];
        } else {
            ans += t;
        }
    }
    println!("{}", ans);
}
