mod weighted_directed_graph;

#[allow(unused)]
struct DirectedGraph {
    n: usize,
    e: Vec<(usize, usize)>,
}

#[allow(unused)]
impl DirectedGraph {
    pub fn new(n: usize, e: Vec<(usize, usize)>) -> Self {
        DirectedGraph {
            n: n,
            e: e,
        }
    }

    pub fn adjoint_list(&self) -> Vec<Vec<usize>> {
        let mut res = vec![vec![]; self.n];
        for &(u, v) in &self.e {
            res[u].push(v);
        }
        res
    }

    pub fn indegree(&self) -> Vec<usize> {
        let mut res = vec![0; self.n];
        for &(u, v) in &self.e {
            res[v] += 1;
        }
        res
    }

    pub fn longest_len_of_path(&self) -> usize {
        let mut indegree = self.indegree();
        let mut al = self.adjoint_list();
        let mut used = vec![-1isize; self.n];
        let mut q = std::collections::VecDeque::new();
        for i in 0..self.n {
            if indegree[i] == 0 {
                q.push_back(i);
                used[i] = 0;
            }
        }
        while let Some(u) = q.pop_front() {
            for &v in &al[u] {
                indegree[v] -= 1;
                if indegree[v] == 0 {
                    used[v] = used[u] + 1;
                    q.push_back(v);
                }
            }
        }
        used.into_iter().max().unwrap() as usize
    }

    pub fn has_cycle(&self) -> bool {
        let mut indegree = self.indegree();
        let mut al = self.adjoint_list();
        let mut res = vec![];
        let mut q = std::collections::VecDeque::new();
        for i in 0..self.n {
            if indegree[i] == 0 {
                q.push_back(i);
                res.push(i);
            }
        }
        while let Some(u) = q.pop_front() {
            for &v in &al[u] {
                indegree[v] -= 1;
                if indegree[v] == 0 {
                    res.push(v);
                    q.push_back(v);
                }
            }
        }
        res.len() != self.n
    }

}