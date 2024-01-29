const BUCKET_RATIO: usize = 16;
const SPLIT_RATIO: usize = 24;

#[allow(unused)]
struct SortedMultiSet<T: Clone + Copy + Eq + Ord> {
    size: usize,
    s: Vec<Vec<T>>,
}

#[allow(unused)]
impl<T: Clone + Copy + Eq + Ord> SortedMultiSet<T> {
    pub fn new() -> Self {
        SortedMultiSet {
            size: 0,
            s: vec![vec![]],
        }
    }

    pub fn build(a: &Vec<T>) -> Self {
        let mut a = a.clone();
        let size = a.len();
        if a.windows(2).any(|v| v[0] > v[1]) {
            a.sort();
        }
        let bucket_size = (size as f64 / BUCKET_RATIO as f64).sqrt().ceil() as usize;
        let mut s = vec![];
        for i in 0..bucket_size {
            let t = a[size * i / bucket_size..size * (i + 1) / bucket_size].to_vec();
            s.push(t);
        }
        SortedMultiSet {
            size: size,
            s: s
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    fn position(&self, x: &T) -> (Vec<T>, usize, usize) {
        let mut index = self.s.len() - 1;
        for (i, s) in self.s.iter().enumerate() {
            if x <= s.last().unwrap() {
                index = i;
                break;
            }
        }
        let s = self.s[index].clone();
        let mut ok = -1;
        let mut ng = s.len();
        while ng as isize - ok > 1 {
            let mid = ((ok + ng as isize) / 2) as usize;
            if s[mid] < *x {
                ok = mid as isize;
            } else {
                ng = mid;
            }
        }

        return (s, index, (ok + 1) as usize);
    }

    pub fn contains(&self, x: &T) -> bool {
        if self.size == 0 {
            return false;
        }
        let (a, _, i) = self.position(x);
        i != a.len() && a[i] == *x
    }

    pub fn count(&self, x: T) -> usize {
        self.index_right(x) - self.index(x)
    }

    pub fn add(&mut self, x: T) {
        if self.size == 0 {
            self.s = vec![vec![x]];
            self.size = 1;
        }
        let (mut a, index, i) = self.position(&x);
        self.s[index].insert(i, x);
        a.insert(i, x);
        self.size += 1;
        if a.len() > self.s.len() * SPLIT_RATIO {
            let mid = a.len() >> 1;
            self.s[index] = a[mid..].to_vec();
            self.s.insert(index, a[..mid].to_vec());
        }
    }

    fn pop(&mut self, index: usize, i: usize) -> T {
        let res = self.s[index].remove(i);
        self.size -= 1;
        if self.s[index].len() == 0 {
            self.s.remove(index);
        }
        res
    }

    pub fn discard(&mut self, x: T) -> bool {
        if self.size == 0 {
            return false;
        }
        let (a, index, i) = self.position(&x);
        if i == a.len() || a[i] != x {
            return false;
        }
        self.pop(index, i);
        true
    }

    pub fn lt(&self, x: T) -> Option<T> {
        for a in self.s.iter().rev() {
            if a[0] < x {
                let mut ok = 0;
                let mut ng = a.len();
                while ng - ok > 1 {
                    let mid = (ok + ng) / 2;
                    if a[mid] < x {
                        ok = mid;
                    } else {
                        ng = mid;
                    }
                }
                return Some(a[ok])
            }
        }
        None
    }

    pub fn le(&self, x: T) -> Option<T> {
        for a in self.s.iter().rev() {
            if a[0] <= x {
                let mut ok = 0;
                let mut ng = a.len();
                while ng - ok > 1 {
                    let mid = (ok + ng) / 2;
                    if a[mid] <= x {
                        ok = mid;
                    } else {
                        ng = mid;
                    }
                }
                return Some(a[ok]);
            }
        }
        None
    }

    pub fn gt(&self, x: T) -> Option<T> {
        for a in self.s.iter() {
            if x < a[a.len() - 1] {
                let mut ok = a.len() - 1;
                let mut ng = -1;
                while ok as isize - ng > 1 {
                    let mid = ((ok as isize + ng) / 2) as usize;
                    if x < a[mid] {
                        ok = mid;
                    } else {
                        ng = mid as isize;
                    }
                }
                return Some(a[ok]);
            }
        }
        None
    }

    pub fn ge(&self, x: T) -> Option<T> {
        for a in self.s.iter() {
            if x <= a[a.len() - 1] {
                let mut ok = a.len() - 1;
                let mut ng = -1;
                while ok as isize - ng > 1 {
                    let mid = ((ok as isize + ng) / 2) as usize;
                    if x <= a[mid] {
                        ok = mid;
                    } else {
                        ng = mid as isize;
                    }
                }
                return Some(a[ok]);
            }
        }
        None
    }

    pub fn nth(&self, i: usize) -> Option<T> {
        let mut index = 0;
        for a in self.s.iter() {
            if a.len() + index <= i {
                index += a.len();
            } else {
                return Some(a[i - index])
            }
        }
        None
    }

    pub fn remove(&mut self, i: usize) -> Option<T> {
        let mut index = 0;
        for (j, a) in self.s.iter().enumerate() {
            if a.len() + index <= i {
                index += a.len();
            } else {
                let res = self.pop(j, i - index);
                return Some(res);
            }
        }
        None
    }

    pub fn index(&self, x: T) -> usize {
        let mut res = 0;
        for a in self.s.iter() {
            if a[a.len() - 1] < x {
                res += a.len();
            } else {
                let mut ok = a.len() - 1;
                let mut ng = -1;
                while ok as isize - ng > 1 {
                    let mid = ((ok as isize + ng) / 2) as usize;
                    if x <= a[mid] {
                        ok = mid;
                    } else {
                        ng = mid as isize;
                    }
                }
                res += ok;
                break;
            }
        }
        res
    }

    pub fn index_right(&self, x: T) -> usize {
        let mut res = 0;
        for a in self.s.iter() {
            if a[a.len() - 1] <= x {
                res += a.len();
            } else {
                let mut ok = a.len() - 1;
                let mut ng = -1;
                while ok as isize - ng > 1 {
                    let mid = ((ok as isize + ng) / 2) as usize;
                    if x < a[mid] {
                        ok = mid;
                    } else {
                        ng = mid as isize;
                    }
                }
                res += ok;
                break;
            }
        }
        res
    }
}