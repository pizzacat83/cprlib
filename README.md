# cprlib

[![cargo test](https://github.com/pizzacat83/cprlib/actions/workflows/cargo-test.yml/badge.svg)](https://github.com/pizzacat83/cprlib/actions/workflows/cargo-test.yml) [![verify](https://github.com/pizzacat83/cprlib/actions/workflows/oj-verify.yml/badge.svg)](https://github.com/pizzacat83/cprlib/actions/workflows/oj-verify.yml)
## 使い方
Rust のモジュール構造を使っているので、コピペする際に `mod` で囲む必要がある。

```rust
mod gcd { // モジュール名は本リポジトリでの名前に合わせる
    // ここにコピペ
}

fn solve() {
    let x = gcd::calc_gcd(2, 3);
}
```

ファイルの先頭に `use crate::hoge;` みたいなのがある場合は、別のモジュールに依存しているのでそのモジュールも貼ってあげる。
