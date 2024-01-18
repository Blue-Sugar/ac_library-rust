#[allow(unused)]
struct BinaryIndexTree {
    n: usize, 
    data: Vec<usize>,
}

#[allow(unused)]
impl BinaryIndexTree {
    pub fn new(n: usize) -> Self {
        BinaryIndexTree {
            n: n,
            data: vec![0; n], 
        }
    }

    pub fn add(&mut self, mut i: usize, x: usize) {
        i += 1;
        while i <= self.n {
            self.data[i - 1] += x;
            i += i & (!i + 1);
        }
    }

    pub fn build(a: &Vec<usize>) -> Self {
        let mut res = Self::new(a.len());
        for (i, &a) in a.iter().enumerate() {
            res.add(i, a);
        }
        res
    }

    // sum of [0, r)
    fn _sum(&self, mut r: usize) -> usize {
        let mut res = 0;
        while r > 0 {
            res += self.data[r - 1];
            r -= r & (!r + 1);
        }
        res
    }

    // sum of [l, r)
    pub fn sum(&self, l: usize, r: usize) -> usize {
        self._sum(r) - self._sum(l)
    }


}