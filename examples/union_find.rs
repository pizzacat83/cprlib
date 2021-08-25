// verification-helper: PROBLEM https://judge.yosupo.jp/problem/unionfind

use proconio::input;

fn main() {
    input! {
        vertices_len: usize,
        queries_len: usize,
    }
    let mut uf = cprlib::union_find::UnionFind::new(vertices_len);

    input! {
        queries: [(u64, usize, usize); queries_len],
    }

    for &(is_same, x, y) in queries.iter() {
        if is_same == 1 {
            println!(
                "{}",
                match uf.same(x, y) {
                    true => "1",
                    false => "0",
                }
            );
            
        } else {
            uf.unite(x, y);
        }
    }
}
