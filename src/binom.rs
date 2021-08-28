use crate::mod64::Mod64;

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

    pub fn reserve(&mut self, size: usize) {
        self.inv_table.reserve(size);
        self.fact_table.reserve(size);
        self.fact_inv_table.reserve(size);
        self.check_sanity();
    }

    fn fill_table(&mut self, k: u64) {
        if self.len < k as usize + 1 {
            self.reserve(k as usize + 1 - self.len);
            for i in self.len as u64..=k {
                let inv = -self.inv_table[(self.modulo % i) as usize] * Mod64::new(self.modulo / i, self.modulo);
                self.inv_table.push(inv);
                self.fact_table.push(self.fact_table[(i - 1) as usize] * Mod64::new(i, self.modulo));
                self.fact_inv_table.push(self.fact_inv_table[(i - 1) as usize] * inv);
            }
            self.len = self.inv_table.len();
        }
        self.check_sanity();
    }

    pub fn fact_inv(&mut self, k: u64) -> Mod64 {
        self.fill_table(k);
        return self.fact_inv_table[k as usize];
    }

    pub fn binom(&mut self, n: u64, k: u64) -> Mod64 {
        let res =
            Mod64::product(
                (n-k+1..=n).map(|i| Mod64::new(i, self.modulo)),
                self.modulo,
            )
            * self.fact_inv(k);
        self.check_sanity();
        return res;
    }

    fn check_sanity(&self) {
        if cfg!(debug_assertions) {
            assert_eq!(self.len, self.inv_table.len());
            assert_eq!(self.len, self.fact_table.len());
            assert_eq!(self.len, self.fact_inv_table.len());
        }
    }
}
