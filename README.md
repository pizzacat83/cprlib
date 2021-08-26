# cprlib

[![verify](https://github.com/pizzacat83/cprlib/actions/workflows/verify.yml/badge.svg)](https://github.com/pizzacat83/cprlib/actions/workflows/verify.yml)

## 使い方
そのまま貼り付けても動くが、ライブラリ同士の名前衝突を防ぐために `mod` で囲んでおくと良いかも。

```rust
mod gcd {
    // ここにコピペ
}

fn solve() {
    let x = gcd::calc_gcd(2, 3);
}
```
