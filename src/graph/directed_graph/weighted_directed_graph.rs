use itertools::Itertools;

const INF: isize = std::isize::MAX / 2;

#[derive(Clone)]
#[allow(unused)]
struct WeightedDirectedGraph {
    n: usize,
    ew: Vec<(usize, usize, isize)>,
}

#[allow(unused)]
impl WeightedDirectedGraph {

    pub fn build(n: usize, ew: Vec<(usize, usize, isize)>) -> Self {
        WeightedDirectedGraph {
            n: n,
            ew: ew,
        }
    }

    pub fn adjoint_list(&self) -> Vec<Vec<(usize, isize)>> {
        let mut res = vec![vec![]];
        for &(u, v, w) in &self.ew {
            res[u].push((v, w));
        }
        res
    }

    pub fn bellman_ford_min(&self, from: usize) -> Vec<isize> {
        let mut res = vec![INF ; self.n];
        res[from] = 0;

        for i in 0..2 * self.n {
            for &(u, v, w) in &self.ew {
                if res[u] + w < res[v] && res[u] != INF {
                    if i < self.n {
                        res[v] = res[u] + w;
                    } else {
                        res[v] = -INF;
                    }
                }
            }
        }
        res
    }

    pub fn bellman_ford_max(&self, from: usize) -> Vec<isize> {
        let mut res = vec![INF; self.n];
        res[from] = 0;

        let ew = self.ew.iter().map(
            |(u, v, w)| (u, v, -w)
        ).collect_vec();

        for i in 0..2 * self.n {
            for &(u, v, w) in &ew {
                if res[*u] + w < res[*v] && res[*u] != INF {
                    if i < self.n {
                        res[*v] = res[*u] + w;
                    } else {
                        res[*v] = -INF;
                    }
                }
            }
        }
        for i in 0..self.n {
            res[i] *= -1;
        }
        res
    }

    pub fn floyd_warshall(&self) -> Vec<Vec<isize>> {
        let mut res = vec![vec![INF; self.n];  self.n];
        for &(u, v, w) in &self.ew {
            res[u][v] = w;
        }
        for i in 0..self.n {
            res[i][i] = 0;
        }

        for k in 0..self.n {
            for u in 0..self.n {
                for v in 0..self.n {
                    res[u][v] = res[u][v].min(res[u][k] + res[k][v]);
                }
            }
        }
        res
    }
}