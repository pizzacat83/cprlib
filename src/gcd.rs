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
