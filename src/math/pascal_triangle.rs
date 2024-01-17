use std::ops::Add;

#[allow(unused)]
trait Number {
    fn zero() -> Self;
    fn one() -> Self;
}

#[allow(unused)]
struct PascalTriangle<S> {
    size: usize,
    v: Vec<Vec<S>>,
}

#[allow(unused)]
impl<S> PascalTriangle<S> 
where S: Clone + Copy + Add<Output = S> + Number {
    pub fn build(size: usize) -> Self {
        let mut v = vec![vec![S::zero(); size]; size];
        v[1][1] = S::one();
        for i in 1..size {
            for j in 0..i + 1 {
                if j == 0 || j == i {
                    v[i][j] = S::one();
                } else {
                    v[i][j] = v[i - 1][j - 1] + v[i - 1][j];
                }
            }
        }
        PascalTriangle {
            size: size,
            v: v,
        }
    }

    pub fn combination(&self, n: usize, r: usize) -> S {
        self.v[n][r]
    }

    pub fn duplicated_combination(&self, n: usize, r: usize) -> S {
        self.v[n + r - 1][r]
    }
}