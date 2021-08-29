use crate::mod64::Mod64;

// 二項係数というより、Mod64 周りのユーティリティ集かもしれない。

pub struct BinomTable {
    modulo: u64,
    len: usize,
    inv_table: Vec<Mod64>,
    fact_table: Vec<Mod64>,
    fact_inv_table: Vec<Mod64>,
}

impl BinomTable {
    pub fn new(modulo: u64) -> Self {
        let table = Self {
            modulo,
            len: 2,
            inv_table: vec![Mod64::new(1, modulo); 2],
            fact_table: vec![Mod64::new(1, modulo); 2],
            fact_inv_table: vec![Mod64::new(1, modulo); 2],
        };
        table.check_sanity();
        table
    }

    /**
     * 追加で additional 個の要素が入るように reserve
     * 
     * 各種関数はいい感じに reserve をするので、
     * あんまり神経質に手動 reserve をする必要はない
     */
    pub fn reserve(&mut self, additional: usize) {
        self.inv_table.reserve(additional);
        self.fact_table.reserve(additional);
        self.fact_inv_table.reserve(additional);
        self.check_sanity();
    }

    /**
     * 現在の要素数に関係なく、total 個の要素が入るように reserve
     * 
     * 各種関数はいい感じに reserve をするので、
     * あんまり神経質に手動 reserve をする必要はない
     */
    pub fn reserve_total(&mut self, total: usize) {
        if total > self.len {
            self.reserve(total - self.len);
        }
    }

    fn fill_table(&mut self, k: u64) {
        if self.len < k as usize + 1 {
            self.reserve_total(k as usize + 1);
            for i in self.len as u64..=k {
                let inv = -self.inv_table[(self.modulo % i) as usize] * self.to_mod64(self.modulo / i);
                self.inv_table.push(inv);
                self.fact_table.push(self.fact_table[(i - 1) as usize] * self.to_mod64(i));
                self.fact_inv_table.push(self.fact_inv_table[(i - 1) as usize] * inv);
            }
            self.len = self.inv_table.len();
        }
        self.check_sanity();
    }

    fn to_mod64(&self, x: u64) -> Mod64 {
        Mod64::new(x, self.modulo)
    }

    pub fn inv(&mut self, k: Mod64) -> Mod64 {
        if k.value == 0 {
            panic!("BinomTable: 0 is not invertible");
        }
        self.fill_table(k.value);
        let inv = self.inv_table[k.value as usize];

        debug_assert_eq!(inv * k, self.to_mod64(1));

        inv
    }

    // Z/nZ の上で well-defined ではないので、引数は u64
    pub fn fact(&mut self, k: u64) -> Mod64 {
        self.fill_table(k);
        return self.fact_table[k as usize];
    }

    // Z/nZ の上で well-defined ではないので、引数は u64
    pub fn fact_inv(&mut self, k: u64) -> Mod64 {
        self.fill_table(k);
        return self.fact_inv_table[k as usize];
    }

    /// O(k) で nCk を計算
    // nCk は Z/nZ の上で well-defined ではないので、引数は u64
    pub fn binom_linear(&mut self, n: u64, k: u64) -> Mod64 {
        let res =
            Mod64::product(
                (n-k+1..=n).map(|i| self.to_mod64(i)),
                self.modulo,
            )
            * self.fact_inv(k);
        self.check_sanity();
        return res;
    }

    /**
     * O(1) で nCk を計算  
     * ただし前計算 O(n)
     */
    // nCk は Z/nZ の上で well-defined ではないので、引数は u64
    pub fn binom_const(&mut self, n: u64, k: u64) -> Mod64 {
        let res = self.fact(n) * self.fact_inv(n-k) * self.fact_inv(k);
        self.check_sanity();
        return res;
    }

    fn check_sanity(&self) {
        debug_assert_eq!(self.len, self.inv_table.len());
        debug_assert_eq!(self.len, self.fact_table.len());
        debug_assert_eq!(self.len, self.fact_inv_table.len());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inv() {
        let mut table = BinomTable::new(5);
        assert_eq!(table.inv(Mod64::new(3, 5)), Mod64::new(2, 5));
    }
}

