mod binary_index_tree;
mod lowest_common_ancestor;
mod segment_tree;


#[allow(unused)]
struct Tree {
    root: usize,
    n: usize,
    e: Vec<(usize, usize)>,
    pub adjoint_list: Vec<Vec<usize>>,
}

#[allow(unused)]
impl Tree {
    pub fn build(root: usize, n: usize, e: Vec<(usize, usize)>) -> Self {
        let mut adjoint_list = vec![vec![]; n];
        for &(u, v) in &e {
            adjoint_list[u].push(v);
            adjoint_list[v].push(u);
        }

        Tree {
            root: root,
            n: n,
            e: e,
            adjoint_list: adjoint_list,
        }
    }

    pub fn parent(&self) -> Vec<isize> {
        let mut res = vec![self.n as isize; self.n];
        let mut q = std::collections::VecDeque::new();
        q.push_back(self.root);
        res[self.root] = -1;
        while let Some(u) = q.pop_front() {
            for &v in &self.adjoint_list[u] {
                if res[v] != self.n as isize {
                    continue;
                }
                res[v] = u as isize;
                q.push_back(v);
            }
        }
        res
    }

    pub fn children(&self) -> Vec<Vec<usize>> {
        let mut res = vec![vec![]; self.n];
        for (i, &p) in self.parent().iter().enumerate() {
            if p == -1 {
                continue;
            }
            res[p as usize].push(i);
        }
        res
    }
 
    pub fn distance_from_root(&self) -> Vec<usize> {
        let mut res = vec![std::usize::MAX; self.n];
        let mut q = std::collections::VecDeque::new();
        q.push_back(self.root);
        res[self.root] = 0;

        while let Some(u) = q.pop_front() {
            for &v in &self.adjoint_list[u] {
                if res[v] == std::usize::MAX {
                    res[v] = res[u] + 1;
                    q.push_back(v);
                }
            }
        }
        res
    }

    pub fn diameter(&self) -> usize {
        let mut used = vec![-1; self.n];
        let mut q = std::collections::VecDeque::new();
        q.push_back(0);
        used[0] = 0;
        let mut memo = 0;
        while let Some(i) = q.pop_front() {
            memo = i;
            for &j in &self.adjoint_list[i] {
                if used[j] >= 0 {
                    continue;
                }
                used[j] = 0;
                q.push_back(j);
            }
        }
        used = vec![-1; self.n];
        q.push_back(memo);
        used[memo] = 0;
        while let Some(i) = q.pop_front() {
            for &j in &self.adjoint_list[i] {
                if used[j] >= 0 {
                    continue;
                }
                used[j] = used[i] + 1;
                q.push_back(j);
            }
        }
        *used.iter().max().unwrap() as usize
    }
}