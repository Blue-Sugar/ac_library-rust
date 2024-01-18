use super::*;

#[allow(unused)]
struct LCA {
    tree: Tree,
    parent: Vec<Vec<isize>>,
    dist: Vec<usize>,
}

#[allow(unused)]
impl LCA {
    fn build(tree: Tree) -> Self {
        let mut parent = vec![tree.parent()];
        let mut k = 1;
        while (1 << k) < tree.n {
            let mut p = vec![-1; tree.n];
            for i in 0..tree.n {
                let mut tmp = parent[k - 1][i];
                if tmp < 0 {
                    continue;
                }
                tmp = parent[k - 1][tmp as usize];
                p[i] = tmp;
            }
            parent.push(p);
            k += 1;
        }
        let dist = tree.distance_from_root();
        LCA {
            tree: tree,
            parent: parent,
            dist: dist,
        }
    }

    fn lowest_common_ancestor(&self, mut u: usize, mut v: usize) -> usize {
        if self.dist[u] < self.dist[v] {
            std::mem::swap(&mut u, &mut v);
        }
        let du = self.dist[u] - self.dist[v];
        for i in 0..self.parent.len() {
            if ((du >> i) & 1) == 1 {
                u = self.parent[i][u] as usize;
            }
        }
        if u == v {return u;}
        for i in (0..self.parent.len()).rev() {
            if self.parent[i][u] == -1 || self.parent[i][v] == -1 {
                continue;
            }
            if self.parent[i][u] == self.parent[i][v] {
                continue;
            }
            u = self.parent[i][u] as usize;
            v = self.parent[i][v] as usize;
        }
        self.parent[0][u] as usize
    }

    fn get_dist(&self, u: usize, v: usize) -> usize {
        let lca = self.lowest_common_ancestor(u, v);
        self.dist[u] + self.dist[v] - 2 * self.dist[lca]
    }

    fn is_on_path(&self, a: usize, u: usize, v: usize) -> bool {
        self.get_dist(u, a) + self.get_dist(a, v) == self.get_dist(u, v)
    }
}