pub fn calc_gcd(a: u64, b: u64) -> u64 {
    let (a, b) = if a < b { (a, b) } else { (b, a) };
    calc_gcd_rec(a, b)
}

fn calc_gcd_rec(a: u64, b: u64) -> u64 {
    // a < b を仮定
    if a == 0 {
        return b;
    }
    calc_gcd_rec(b % a, a)
}

pub fn calc_gcd_multi<Iter: Iterator<Item = u64>>(iter: Iter) -> u64 {
    iter.fold(0, calc_gcd)
}
