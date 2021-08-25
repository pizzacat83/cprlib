#[derive(Debug)]
pub struct UnionFind {
    parent: Vec<Option<usize>>,
    rank: Vec<usize>,
}

impl UnionFind {
    pub fn new(size: usize) -> UnionFind {
        return UnionFind {
            parent: vec![None; size],
            rank: vec![0; size],
        }
    }

    pub fn unite(&mut self, x: usize, y: usize) {
        let x_root = self.find_root(x);
        let y_root = self.find_root(y);

        if x_root == y_root {
            return;
        }

        if self.rank[x_root] < self.rank[y_root] {
            self.parent[x_root] = Some(y_root);
        } else {
            self.parent[y_root] = Some(x_root);
            if self.rank[x_root] == self.rank[y_root] {
                self.rank[x_root] += 1;
            }
        }
    }
    
    fn find_root(&mut self, x: usize) -> usize {
        if let Some(parent) = self.parent[x] {
            let root = self.find_root(parent);
            self.parent[x] = Some(root);
            return root;
        } else {
            return x;
        }
    }
    
    pub fn same(&mut self, x: usize, y: usize) -> bool {
        return self.find_root(x) == self.find_root(y);
    }
}
