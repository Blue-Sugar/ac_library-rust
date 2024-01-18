#[allow(unused)]
pub struct PDSU {
    n: usize,
    p: Vec<isize>,
    w: Vec<isize>,
}

#[allow(unused)]
impl PDSU {
    pub fn new(n: usize) -> Self {
        PDSU { 
            n: n, 
            p: vec![-1; n],
            w: vec![0; n],
        }
    }

    pub fn root(&self, mut v: usize) -> (usize, isize) {
        let mut pot = self.w[v];
        while self.p[v] >= 0 {
            v = self.p[v] as usize;
            pot += self.w[v];
        }
        (v, pot)
    }

    pub fn is_same(&self, u: usize, v: usize) -> bool {
        self.root(u).0 == self.root(v).0
    }

    pub fn unite(&mut self, u: usize, v: usize, d: isize) -> bool {
        let mut u = self.root(u);
        let mut v  = self.root(v);
        if u.0 == v.0 {
            return u.1 - v.1 == d;
        }
        let mut w = u.1 - v.1 - d;
        if self.p[u.0] > self.p[v.0] {
            std::mem::swap(&mut u, &mut v);
            w *= -1;
        }
        self.p[u.0] += self.p[v.0];
        self.p[v.0] = u.0 as isize;
        self.w[v.0] = w;
        true
    }

    pub fn size(&self, v: usize) -> usize {
        -self.p[self.root(v).0] as usize
    }
}