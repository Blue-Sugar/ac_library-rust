use crate::math::algebra::monoid::*;

#[allow(unused)]
struct SegmentTree<M> {
    n: usize,
    data: Vec<M>,
    size: usize,
}

#[allow(unused)]
impl<M> SegmentTree<M>
where M: Clone + Copy + Monoid {
    pub fn new(n: usize) -> Self {
        let size = n.next_power_of_two();
        SegmentTree {
            n: n,
            data: vec![M::e(); 2 * size],
            size: size,
        }
    }

    pub fn build(a: &Vec<M>) -> Self {
        let size = a.len().next_power_of_two();
        let mut data = vec![M::e(); 2 * size];
        for (i, &a) in a.iter().enumerate() {
            data[size + i] = a;
        }
        for i in (1..size).rev() {
            data[i] = data[2 * i].op(&data[2 * i + 1]);
        }
        SegmentTree {
            n: a.len(),
            data: data,
            size: size,
        }
    }

    pub fn set_at(&mut self, i: usize, x: M) {
        let mut i = i + self.size;
        self.data[i] = x;
        i /= 2;
        while i > 0 {
            self.data[i] = self.data[2 * i].op(&self.data[2 * i + 1]);
            i /= 2;
        }
    }

    pub fn prod(&self, l: usize, r: usize) -> M {
        let mut l = l + self.size;
        let mut r = r + self.size;
        let mut l_res = M::e();
        let mut r_res = M::e();
        while l < r {
            if l % 2 == 1 {
                l_res = l_res.op(&self.data[l]);
                l += 1;
            }
            if r % 2 == 1 {
                r -= 1;
                r_res = self.data[r].op(&r_res);
            }
            l /= 2;
            r /= 2;
        }
        l_res.op(&r_res)
    }

    pub fn all_prod(&self) -> M {
        self.data[1]
    }

    pub fn get(&self, i: usize) -> M {
        self.data[self.size + i]
    }

    pub fn max_right<P>(&self, l: usize, f: P) -> usize 
    where P: Fn(&M) -> bool {
        if l == self.n {
            return self.n;
        }
        let mut l = l + self.size;
        let mut prod = M::e();
        while {
            l >> l.trailing_zeros();
            let v = prod.op(&self.data[l]);
            if f(&v) {
                while l < self.size {
                    l <<= 1;
                    let v = prod.op(&self.data[l]);
                    if f(&v) {
                        prod = v;
                        l += 1;
                    }
                }
                return l - self.size;
            }
            prod = v;
            l += 1;
            l.count_ones() > 1
        } {}
        self.n
    }
}