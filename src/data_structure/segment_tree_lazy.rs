use crate::math::algebra::monoid::*;

#[allow(unused)]
pub struct SegmentTreeLazy<M, A> {
    n: usize,
    size: usize,
    data: Vec<(M, A)>,
    zeros: usize,  
}

#[allow(unused)]
impl<M, A> SegmentTreeLazy<M, A>
where 
    M: Monoid,
    A: Monoid + Act<M>,
{
    pub fn new(n: usize) -> Self {
        let size = n.next_power_of_two();
        SegmentTreeLazy {
            n: n,
            data: vec![(M::e(), A::e()); 2 * size],
            size: size,
            zeros: size.trailing_zeros() as usize,
        }
    }
    
    pub fn build(a: &Vec<M>) -> Self {
        let a = a.clone();
        let size = a.len().next_power_of_two();
        let mut data = vec![(M::e(), A::e()); 2 * size];
        for (data, a) in data[size..].iter_mut().zip(a.iter()) {
            data.0 = *a;
        }
        let mut res = SegmentTreeLazy {
            n: a.len(),
            size: size,
            data: data,
            zeros: size.trailing_zeros() as usize,
        };
        for i in (1..size).rev() {
            res.save_at(i);
        }
        res
    }
    
    fn apply_at(&mut self, x: usize, f: &A) {
        let tmp = &mut self.data[x];
        *tmp = (f.act(&tmp.0), tmp.1.op(f));
    }

    fn propagate_at(&mut self, x: usize) {
        let f = std::mem::replace(&mut self.data[x].1, A::e());
        self.apply_at(2 * x, &f);
        self.apply_at(2 * x + 1, &f);
    }

    fn save_at(&mut self, x: usize) {
        self.data[x].0 = self.data[2 * x].0.op(&self.data[2 * x + 1].0);
    }

    fn propagate(&mut self, l: usize, r: usize) {
        let l = l + self.size;
        let r = r + self.size;
        for i in (1..=self.zeros).rev() {
            if (l >> i) << i != l {
                self.propagate_at(l >> i);
            }
            if (r >> i) << i != r {
                self.propagate_at((r - 1) >> i);
            }
        }
    }

    fn save(&mut self, l: usize, r: usize) {
        let l = l + self.size;
        let r = r + self.size;
        for i in 1..=self.zeros {
            if (l >> i) << i != l {
                self.save_at(l >> i);
            }
            if (r >> i) << i != r {
                self.save_at((r - 1) >> i);
            }
        }
    }

    pub fn action(&mut self, l: usize, r: usize, f: A) {
        if l == r {
            return;
        }
        self.propagate(l, r);
        let mut x = l + self.size;
        let mut y = r + self.size;
        while x < y {
            if x & 1 == 1 {
                self.apply_at(x, &f);
                x += 1;
            }
            if y & 1 == 1 {
                y -= 1;
                self.apply_at(y, &f);
            }
            x >>= 1;
            y >>= 1;
        }
        self.save(l, r);
    }

    pub fn prod(&mut self, l: usize, r: usize) -> M {
        if l == r {
            return M::e();
        }
        self.propagate(l, r);
        let mut x = l + self.size;
        let mut y = r + self.size;
        let mut p = M::e();
        let mut q = M::e();
        while x < y {
            if x & 1 == 1 {
                p = p.op(&self.data[x].0);
                x += 1;
            }
            if y & 1 == 1 {
                y -= 1;
                q = self.data[y].0.op(&q);
            }
            x >>= 1;
            y >>= 1;
        }
        p.op(&q)
    }

    pub fn set_at(&mut self, x: usize, v: M) {
        let y = x + self.size;
        for i in (1..=self.zeros).rev() {
            self.propagate_at(y >> i);
        }
        self.data[y].0 = v;
        for i in 1..=self.size.trailing_zeros() {
            self.save_at(y >> i);
        }
    }

    pub fn get_at(&mut self, x: usize) -> M {
        let y = x + self.size;
        for i in (1..=self.zeros).rev() {
            self.propagate_at(y >> i);
        }
        self.data[y].0
    }

    pub fn get_all(&mut self) -> Vec<M> {
        for i in 0..self.size {
            self.propagate_at(i);
        }
        for i in (0..self.size).rev() {
            self.save_at(i);
        }
        self.data[self.size..self.size + self.n].iter().map(|(m, a)| *m).collect::<Vec<M>>()
    }
}
