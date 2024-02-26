#[allow(unused)]
struct DAG {
    n: usize,
    e: Vec<(usize, usize)>,
}

#[allow(unused)]
impl DAG {
    pub fn build(n: usize, e: Vec<(usize, usize)>) -> Self {
        let mut indegree = vec![0; n];
        let mut adjoint_list = vec![vec![]; n];
        for &(u, v) in &e {
            indegree[v] += 1;
            adjoint_list[u].push(v);
        }
        let mut check = 0;
        let mut q = std::collections::VecDeque::new();
        for (i, &deg) in indegree.iter().enumerate() {
            if deg == 0 {
                q.push_back(i);
                check += 1;
            }
        }
        while let Some(u) = q.pop_front() {
            for &v in &adjoint_list[u] {
                indegree[v] -= 1;
                if indegree[v] == 0 {
                    q.push_back(v);
                    check += 1;
                }
            }
        }
        assert!(check == n);
        DAG {
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

    pub fn topological_sort(&self) -> Vec<usize> {
        let mut indegree = self.indegree();
        let mut adjoint_list = self.adjoint_list();
        let mut q = std::collections::VecDeque::new();
        let mut res = vec![];
        for (i, &deg) in indegree.iter().enumerate() {
            if deg == 0 {
                q.push_back(i);
                res.push(i);
            }
        }
        while let Some(u) = q.pop_front() {
            for &v in &adjoint_list[u] {
                indegree[v] -= 1;
                if indegree[v] == 0 {
                    q.push_back(v);
                    res.push(v);
                }
            }
        }
        assert!(res.len() == self.n);
        res
    }

    pub fn longest_len_of_path_from_u_to_v(&self, u: usize, v: usize) -> Option<usize> {
        if u == v {
            return Some(0);
        }
        let adjoint_list = self.adjoint_list();
        let topological_sort = self.topological_sort();
        let mut res = vec![std::isize::MIN / 2; self.n];
        res[u] = 0;
        for &i in &topological_sort {
            for &j in &adjoint_list[i] {
                res[j] = res[j].max(res[i] + 1);
            }
        }
        if res[v] < 0 {
            return None;
        }
        Some(res[v] as usize)
    }
}