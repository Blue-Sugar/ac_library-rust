#[allow(unused)]
struct UndirectedGraph {
    n: usize,
    e: Vec<(usize, usize)>,
}

#[allow(unused)]
impl UndirectedGraph {
    pub fn build(n: usize, e: &Vec<(usize, usize)>) -> Self {
        let e = e.clone();
        UndirectedGraph {
            n: n,
            e: e,
        }
    }

    fn adjoint_list(&self) -> Vec<Vec<usize>> {
        let mut res = vec![vec![]; self.n];
        for &(u, v) in &self.e {
            res[u].push(v);
            res[v].push(u);
        }
        res
    }

    pub fn has_hamiltonian_path(&self) -> bool {
        let al = self.adjoint_list();
        let mut dp = vec![vec![false; self.n]; 1 << self.n];
        for i in 0..self.n {
            dp[1 << i][i] = true;
        }
        for k in 0..1 << self.n {
            for i in 0..self.n {
                if dp[k][i] {
                    for &j in &al[i] {
                        if k & (1 << j) == 0 {
                            dp[k | (1 << j)][j] = true;
                        }
                    }
                }
            }
        }
        dp[(1 << self.n) - 1].iter().any(|b| *b)
    }
}