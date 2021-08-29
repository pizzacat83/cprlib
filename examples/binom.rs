// verification-helper: PROBLEM https://judge.yosupo.jp/problem/binomial_coefficient


use proconio::input;
use cprlib::binom::BinomTable;

fn main() {
    input! {
        queries_len: usize,
        modular: u64,
        queries: [(u64, u64); queries_len],
    }
    
    let mut binom = BinomTable::new(modular);

    for &(n, k) in queries.iter() {
        println!("{}", binom.binom_linear(n, k).value);
    }
}
