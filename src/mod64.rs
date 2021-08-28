
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Mod64 {
    value: u64,
    modulo: u64,
}

impl Mod64 {
    pub fn new(value: u64, modulo: u64) -> Self {
        Self {
            value: value % modulo,
            modulo: modulo,
        }
    }

    // 要素が0個の場合の対応が難しいので、iter.product() メソッドの実装が厳しい
        // Mod64(1) を返すべきなのだが、要素が0個なので modulo がわからない
        // iter.product に引数を追加することはできない
    // しょうがないのでここで提供
    // まあ、代数的データ型を使って Mod64::One の四則演算を実装したらなんとかなるんだけど、めんどくさすぎる
    pub fn product<I>(iter: I, modulo: u64) -> Self 
    where I: Iterator<Item = Self> {
        iter.fold(Mod64::new(1, modulo), |a, b| a * b)
    }
}

impl std::ops::Add for Mod64 {
    type Output = Mod64;
    fn add(self, other: Mod64) -> Mod64 {
        if cfg!(debug_assertions) {
            assert_eq!(self.modulo, other.modulo, "Mod64: different modulo");
        }
        Mod64::new(self.value + other.value, self.modulo)
    }
}

impl std::ops::Neg for Mod64 {
    type Output = Mod64;

    fn neg(self) -> Mod64 {
        Mod64::new(self.modulo - self.value, self.modulo)
    }
}
 
impl std::ops::Sub for Mod64 {
    type Output = Mod64;

    fn sub(self, other: Mod64) -> Mod64 {
        self + (-other)
    }
}

impl std::ops::Mul for Mod64 {
    type Output = Mod64;

    fn mul(self, other: Mod64) -> Mod64 {
        if cfg!(debug_assertions) {
            assert_eq!(self.modulo, other.modulo, "Mod64: different modulo");
        }
        Mod64::new(self.value * other.value, self.modulo)
    }
}

#[cfg(test)]
mod tests_mod64 {
    use super::*;

    #[test]
    fn test_product() {
        assert_eq!(Mod64::product((1..=5).map(|i| Mod64::new(i, 7)), 7), Mod64::new(120, 7));
    }
}
