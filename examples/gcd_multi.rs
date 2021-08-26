// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/problems/1009

fn main() {
    loop {
        let mut line = String::new();
        if std::io::stdin().read_line(&mut line).is_err() {
            break;
        }
        let nums = line
            .split_ascii_whitespace()
            .map(|s| s.parse::<u64>().unwrap())
            .collect::<Vec<_>>();
        println!("{}", solve(nums));
    }
}

fn solve(nums: Vec<u64>) -> u64 {
    cprlib::gcd::calc_gcd_multi(nums.into_iter())
}
