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

pub fn calc_gcd_multi<'a, Iter: Iterator<Item = &'a u64>>(iter: Iter) -> u64 {
    iter.map(|&a| a).fold(0, calc_gcd)
}
