use std::{cmp::Reverse, collections::BinaryHeap};

const INF: isize = std::isize::MAX / 2;
type BinaryHrap<S> = std::collections::BinaryHeap<S>;

#[allow(unused)]
struct SlopeTrick {
    min: isize,
    l: BinaryHrap<isize>,
    r: BinaryHrap<Reverse<isize>>,
}

#[allow(unused)]
impl SlopeTrick {
    pub fn new() -> Self {
        let mut l = BinaryHeap::new();
        let mut r = BinaryHeap::new();
        l.push(-INF);
        r.push(Reverse(INF));
        SlopeTrick {
            min: 0,
            l: l,
            r: r,
        }
    }

    pub fn min(&self) -> isize {
        self.min
    }

    pub fn add_const(&mut self, a: isize) {
        self.min += a;
    }

    pub fn add_plus(&mut self, a: isize) {
        self.min += (self.l.peek().unwrap() - a).max(0);
        self.l.push(a);
        let tmp = self.l.pop().unwrap();
        self.r.push(Reverse(tmp));
    }

    pub fn add_minus(&mut self, a: isize) {
        self.min += (a - self.r.peek().unwrap().0).max(0);
        self.r.push(Reverse(a));
        let tmp = self.r.pop().unwrap().0;
        self.l.push(tmp);
    }

    pub fn add_absolute(&mut self, a: isize) {
        self.add_plus(a);
        self.add_minus(a);
    }
}