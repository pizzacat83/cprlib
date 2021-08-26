use num::{Unsigned, PrimInt};
pub trait UPrimInt: Unsigned + PrimInt {}
impl<T> UPrimInt for T where T: Unsigned + PrimInt {}

pub fn calc_gcd<U: UPrimInt>(a: U, b: U) -> U {
    let (a, b) = if a < b { (a, b) } else { (b, a) };
    calc_gcd_rec(a, b)
}

fn calc_gcd_rec<U: UPrimInt>(a: U, b: U) -> U {
    // a < b を仮定
    if a == U::zero() {
        return b;
    }
    calc_gcd_rec(b % a, a)
}

pub fn calc_gcd_multi<U, Iter>(iter: Iter) -> U
    where U: UPrimInt,
    Iter: Iterator<Item = U>,
{
    iter.fold(U::zero(), calc_gcd)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_gcd() {
        test_calc_gcd_helper(24, 39, 3);
        test_calc_gcd_helper(35, 55, 5);
        test_calc_gcd_helper(6, 35, 1);
    }

    fn test_calc_gcd_helper(a: u64, b: u64, expected: u64) {
        assert_eq!(
            calc_gcd(a, b),
            expected,
            "calc_gcd({}, {}) != {}",
            a,
            b,
            expected,
        );
    }

    #[test]
    fn test_calc_gcd_multi() {
        test_calc_gcd_multi_helper(vec![24, 54, 39], 3);
        test_calc_gcd_multi_helper(vec![20, 30, 45], 5);
        test_calc_gcd_multi_helper(vec![6], 6);
    }

    fn test_calc_gcd_multi_helper(nums: Vec<u64>, expected: u64) {
        assert_eq!(
            calc_gcd_multi(nums.iter().map(|&a| a)),
            expected,
            "calc_gcd_multi({:?}) != {}",
            nums,
            expected,
        );
    }
}
